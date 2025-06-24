// Vector store implementation for FRMWRX Harvester
// FAISS vector database integration for document storage and similarity search

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::errors::{VectorStoreError, VectorResult};

/// Document representation in the vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub content_type: String,
    pub chunk_index: usize,
    pub total_chunks: usize,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Search result with similarity score
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: Document,
    pub similarity_score: f32,
}

/// Vector store statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorStoreStats {
    pub document_count: usize,
    pub total_embeddings: usize,
    pub index_size_bytes: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// In-memory vector store implementation (simplified FAISS alternative)
/// Note: In production, this would use the actual FAISS library
pub struct VectorStore {
    documents: Arc<RwLock<HashMap<String, Document>>>,
    embeddings: Arc<RwLock<Vec<Vec<f32>>>>,
    document_ids: Arc<RwLock<Vec<String>>>,
    storage_path: std::path::PathBuf,
    dimension: usize,
}

impl VectorStore {
    /// Create a new vector store
    pub async fn new(storage_path: &Path) -> VectorResult<Self> {
        info!("Initializing vector store at: {:?}", storage_path);
        
        // Create storage directory if it doesn't exist
        if let Some(parent) = storage_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| VectorStoreError::Initialization(e.to_string()))?;
        }

        let store = Self {
            documents: Arc::new(RwLock::new(HashMap::new())),
            embeddings: Arc::new(RwLock::new(Vec::new())),
            document_ids: Arc::new(RwLock::new(Vec::new())),
            storage_path: storage_path.to_path_buf(),
            dimension: 384, // Standard embedding dimension
        };

        // Try to load existing data
        if let Err(e) = store.load().await {
            warn!("Could not load existing vector store: {}, starting fresh", e);
        }

        Ok(store)
    }

    /// Add a document to the vector store
    pub async fn add_document(&self, mut document: Document) -> VectorResult<()> {
        info!("Adding document to vector store: {}", document.id);

        // Validate embedding dimension
        if document.embedding.len() != self.dimension {
            return Err(VectorStoreError::AddDocument(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.dimension,
                document.embedding.len()
            )));
        }

        // Normalize embedding
        self.normalize_embedding(&mut document.embedding);

        // Add to storage
        {
            let mut documents = self.documents.write().await;
            let mut embeddings = self.embeddings.write().await;
            let mut document_ids = self.document_ids.write().await;

            documents.insert(document.id.clone(), document.clone());
            embeddings.push(document.embedding.clone());
            document_ids.push(document.id.clone());
        }

        info!("Document added successfully: {}", document.id);
        Ok(())
    }

    /// Add multiple documents in batch
    pub async fn add_documents(&self, documents: Vec<Document>) -> VectorResult<()> {
        info!("Adding {} documents to vector store", documents.len());

        for document in documents {
            self.add_document(document).await?;
        }

        // Save after batch addition
        self.save().await?;
        Ok(())
    }

    /// Search for similar documents
    pub async fn similarity_search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> VectorResult<Vec<SearchResult>> {
        if query_embedding.len() != self.dimension {
            return Err(VectorStoreError::Search(format!(
                "Query embedding dimension mismatch: expected {}, got {}",
                self.dimension,
                query_embedding.len()
            )));
        }

        let mut normalized_query = query_embedding.to_vec();
        self.normalize_embedding(&mut normalized_query);

        let documents = self.documents.read().await;
        let embeddings = self.embeddings.read().await;
        let document_ids = self.document_ids.read().await;

        let mut similarities: Vec<(usize, f32)> = Vec::new();

        // Compute cosine similarity for all embeddings
        for (i, embedding) in embeddings.iter().enumerate() {
            let similarity = self.cosine_similarity(&normalized_query, embedding);
            similarities.push((i, similarity));
        }

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Take top results
        let mut results = Vec::new();
        for (i, similarity) in similarities.into_iter().take(limit) {
            if let Some(doc_id) = document_ids.get(i) {
                if let Some(document) = documents.get(doc_id) {
                    results.push(SearchResult {
                        document: document.clone(),
                        similarity_score: similarity,
                    });
                }
            }
        }

        info!("Found {} similar documents", results.len());
        Ok(results)
    }

    /// Search documents by query string (requires embedding generation)
    pub async fn search(&self, query: &str, limit: usize) -> VectorResult<Vec<Document>> {
        // For now, we'll do a simple text search as a fallback
        // In a real implementation, you would generate embeddings for the query
        let query_lower = query.to_lowercase();
        let documents = self.documents.read().await;

        let mut matches: Vec<(Document, f32)> = documents
            .values()
            .filter_map(|doc| {
                let content_lower = doc.content.to_lowercase();
                if content_lower.contains(&query_lower) {
                    // Simple relevance scoring based on frequency
                    let frequency = content_lower.matches(&query_lower).count() as f32;
                    let relevance = frequency / (doc.content.len() as f32).sqrt();
                    Some((doc.clone(), relevance))
                } else {
                    None
                }
            })
            .collect();

        // Sort by relevance
        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let results: Vec<Document> = matches
            .into_iter()
            .take(limit)
            .map(|(doc, _)| doc)
            .collect();

        info!("Text search found {} documents for query: {}", results.len(), query);
        Ok(results)
    }

    /// Get document by ID
    pub async fn get_document(&self, id: &str) -> VectorResult<Option<Document>> {
        let documents = self.documents.read().await;
        Ok(documents.get(id).cloned())
    }

    /// Remove document by ID
    pub async fn remove_document(&self, id: &str) -> VectorResult<bool> {
        let mut documents = self.documents.write().await;
        let mut embeddings = self.embeddings.write().await;
        let mut document_ids = self.document_ids.write().await;

        if let Some(index) = document_ids.iter().position(|x| x == id) {
            documents.remove(id);
            embeddings.remove(index);
            document_ids.remove(index);
            info!("Document removed: {}", id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get total document count
    pub async fn document_count(&self) -> usize {
        self.documents.read().await.len()
    }

    /// Get vector store statistics
    pub async fn get_stats(&self) -> VectorStoreStats {
        let documents = self.documents.read().await;
        let embeddings = self.embeddings.read().await;

        VectorStoreStats {
            document_count: documents.len(),
            total_embeddings: embeddings.len(),
            index_size_bytes: embeddings.len() * self.dimension * 4, // 4 bytes per f32
            last_updated: chrono::Utc::now(),
        }
    }

    /// Save vector store to disk
    pub async fn save(&self) -> VectorResult<()> {
        info!("Saving vector store to disk");

        let documents = self.documents.read().await;
        let embeddings = self.embeddings.read().await;
        let document_ids = self.document_ids.read().await;

        let data = VectorStoreData {
            documents: documents.clone(),
            embeddings: embeddings.clone(),
            document_ids: document_ids.clone(),
            dimension: self.dimension,
            version: 1,
        };

        let serialized = serde_json::to_string(&data)
            .map_err(|e| VectorStoreError::Save(e.to_string()))?;

        fs::write(&self.storage_path, serialized).await
            .map_err(|e| VectorStoreError::Save(e.to_string()))?;

        info!("Vector store saved successfully");
        Ok(())
    }

    /// Load vector store from disk
    pub async fn load(&self) -> VectorResult<()> {
        if !self.storage_path.exists() {
            return Err(VectorStoreError::Load("Storage file does not exist".to_string()));
        }

        info!("Loading vector store from disk");

        let content = fs::read_to_string(&self.storage_path).await
            .map_err(|e| VectorStoreError::Load(e.to_string()))?;

        let data: VectorStoreData = serde_json::from_str(&content)
            .map_err(|e| VectorStoreError::Load(e.to_string()))?;

        // Validate version compatibility
        if data.version != 1 {
            return Err(VectorStoreError::Load(format!(
                "Unsupported version: {}",
                data.version
            )));
        }

        // Validate dimension consistency
        if data.dimension != self.dimension {
            return Err(VectorStoreError::Load(format!(
                "Dimension mismatch: expected {}, got {}",
                self.dimension,
                data.dimension
            )));
        }

        // Load data
        {
            let mut documents = self.documents.write().await;
            let mut embeddings = self.embeddings.write().await;
            let mut document_ids = self.document_ids.write().await;

            *documents = data.documents;
            *embeddings = data.embeddings;
            *document_ids = data.document_ids;
        }

        info!("Vector store loaded successfully");
        Ok(())
    }

    /// Clear all documents
    pub async fn clear(&self) -> VectorResult<()> {
        info!("Clearing vector store");

        let mut documents = self.documents.write().await;
        let mut embeddings = self.embeddings.write().await;
        let mut document_ids = self.document_ids.write().await;

        documents.clear();
        embeddings.clear();
        document_ids.clear();

        Ok(())
    }

    // Helper methods

    /// Normalize embedding vector
    fn normalize_embedding(&self, embedding: &mut [f32]) {
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in embedding.iter_mut() {
                *value /= magnitude;
            }
        }
    }

    /// Compute cosine similarity between two normalized embeddings
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
}

/// Serializable data structure for persistence
#[derive(Serialize, Deserialize)]
struct VectorStoreData {
    documents: HashMap<String, Document>,
    embeddings: Vec<Vec<f32>>,
    document_ids: Vec<String>,
    dimension: usize,
    version: u32,
}

/// Utility functions for creating documents
impl Document {
    pub fn new(
        url: String,
        title: Option<String>,
        content: String,
        content_type: String,
        chunk_index: usize,
        total_chunks: usize,
        embedding: Vec<f32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url,
            title,
            content,
            content_type,
            chunk_index,
            total_chunks,
            embedding,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_vector_store_basic_operations() {
        let temp_dir = tempdir().unwrap();
        let storage_path = temp_dir.path().join("test_vector_store.json");
        
        let store = VectorStore::new(&storage_path).await.unwrap();

        // Create test document
        let embedding = vec![0.1, 0.2, 0.3]; // Shortened for test
        let doc = Document::new(
            "https://example.com".to_string(),
            Some("Test Document".to_string()),
            "This is test content".to_string(),
            "text/plain".to_string(),
            0,
            1,
            embedding,
        );

        // Test add document
        assert!(store.add_document(doc.clone()).await.is_ok());
        assert_eq!(store.document_count().await, 1);

        // Test get document
        let retrieved = store.get_document(&doc.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, doc.content);

        // Test remove document
        assert!(store.remove_document(&doc.id).await.unwrap());
        assert_eq!(store.document_count().await, 0);
    }

    #[tokio::test]
    async fn test_vector_store_persistence() {
        let temp_dir = tempdir().unwrap();
        let storage_path = temp_dir.path().join("test_persistence.json");

        // Create and populate store
        {
            let store = VectorStore::new(&storage_path).await.unwrap();
            let doc = Document::new(
                "https://example.com".to_string(),
                None,
                "Persistent content".to_string(),
                "text/plain".to_string(),
                0,
                1,
                vec![0.1, 0.2, 0.3],
            );
            store.add_document(doc).await.unwrap();
            store.save().await.unwrap();
        }

        // Load from disk
        {
            let store = VectorStore::new(&storage_path).await.unwrap();
            assert_eq!(store.document_count().await, 1);
        }
    }
}
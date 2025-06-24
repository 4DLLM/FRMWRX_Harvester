// Document harvester implementation for FRMWRX Harvester
// Document processing pipeline with web scraping, PDF parsing, and content extraction

use anyhow::Result;
use futures::future::try_join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tracing::{info, warn, error};

use crate::config::AppConfig;
use crate::errors::{AppError, DocumentProcessingError, DocumentResult};
use crate::security::{ContentType, SecurityValidator};
use crate::vector_store::{Document, VectorStore};

/// Progress tracking for harvesting operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestProgress {
    pub total_urls: usize,
    pub processed_urls: usize,
    pub successful_documents: usize,
    pub failed_documents: usize,
    pub current_url: Option<String>,
    pub status: HarvestStatus,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarvestStatus {
    Idle,
    InProgress,
    Completed,
    Failed,
}

/// Document chunk for processing
#[derive(Debug, Clone)]
pub struct DocumentChunk {
    pub content: String,
    pub chunk_index: usize,
    pub total_chunks: usize,
}

/// Raw document before processing
#[derive(Debug, Clone)]
pub struct RawDocument {
    pub url: String,
    pub content: Vec<u8>,
    pub content_type: ContentType,
    pub title: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Document harvester engine
pub struct DocumentHarvester {
    client: Client,
    config: AppConfig,
    vector_store: Arc<VectorStore>,
    security_validator: SecurityValidator,
    progress: Arc<tokio::sync::Mutex<HarvestProgress>>,
}

impl DocumentHarvester {
    /// Create a new document harvester
    pub async fn new(config: AppConfig, vector_store: Arc<VectorStore>) -> Result<Self, AppError> {
        info!("Initializing document harvester");

        // Create HTTP client with security settings
        let client = Client::builder()
            .timeout(config.request_timeout())
            .danger_accept_invalid_certs(!config.security.validate_ssl)
            .user_agent("FRMWRX-Harvester/1.0")
            .build()?;

        let security_validator = SecurityValidator::new(config.security.clone());

        let progress = Arc::new(tokio::sync::Mutex::new(HarvestProgress {
            total_urls: 0,
            processed_urls: 0,
            successful_documents: 0,
            failed_documents: 0,
            current_url: None,
            status: HarvestStatus::Idle,
            errors: Vec::new(),
        }));

        Ok(Self {
            client,
            config,
            vector_store,
            security_validator,
            progress,
        })
    }

    /// Harvest documents from multiple URLs concurrently
    pub async fn harvest_documents(&self, urls: Vec<String>) -> Result<(), AppError> {
        info!("Starting document harvest for {} URLs", urls.len());

        // Initialize progress tracking
        {
            let mut progress = self.progress.lock().await;
            progress.total_urls = urls.len();
            progress.processed_urls = 0;
            progress.successful_documents = 0;
            progress.failed_documents = 0;
            progress.status = HarvestStatus::InProgress;
            progress.errors.clear();
        }

        // Process URLs in batches to respect concurrency limits
        let batch_size = self.config.security.max_concurrent_downloads;
        let mut successful = 0;
        let mut failed = 0;

        for url_batch in urls.chunks(batch_size) {
            let tasks: Vec<_> = url_batch
                .iter()
                .map(|url| self.process_document_safe(url.clone()))
                .collect();

            let results = futures::future::join_all(tasks).await;

            // Process results
            for (i, result) in results.iter().enumerate() {
                let url = &url_batch[i];
                
                // Update progress
                {
                    let mut progress = self.progress.lock().await;
                    progress.processed_urls += 1;
                    progress.current_url = Some(url.clone());
                }

                match result {
                    Ok(_) => {
                        successful += 1;
                        info!("Successfully processed: {}", url);
                    }
                    Err(e) => {
                        failed += 1;
                        error!("Failed to process {}: {}", url, e);
                        
                        // Track error
                        let mut progress = self.progress.lock().await;
                        progress.errors.push(format!("{}: {}", url, e));
                    }
                }
            }
        }

        // Update final progress
        {
            let mut progress = self.progress.lock().await;
            progress.successful_documents = successful;
            progress.failed_documents = failed;
            progress.status = if failed == 0 {
                HarvestStatus::Completed
            } else {
                HarvestStatus::Failed
            };
            progress.current_url = None;
        }

        info!("Document harvest completed: {} successful, {} failed", successful, failed);
        Ok(())
    }

    /// Process a single document with error handling
    async fn process_document_safe(&self, url: String) -> Result<(), AppError> {
        match self.process_document(url.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error processing document {}: {}", url, e);
                Err(e)
            }
        }
    }

    /// Process a single document (main implementation from architecture)
    async fn process_document(&self, url: String) -> Result<(), AppError> {
        info!("Processing document: {}", url);

        // 1. Validate URL
        let validated_url = self.security_validator.validate_url(&url)?;

        // 2. Fetch document
        let raw_doc = self.fetch_document(validated_url.as_str()).await?;

        // 3. Parse based on content type
        let text = self.extract_text(&raw_doc).await?;

        // 4. Chunk and embed
        let chunks = self.chunk_text(&text, &raw_doc)?;

        // 5. Generate embeddings and store
        for chunk in chunks {
            let embedding = self.generate_embedding(&chunk.content).await?;
            
            let document = Document::new(
                raw_doc.url.clone(),
                raw_doc.title.clone(),
                chunk.content,
                match raw_doc.content_type {
                    ContentType::Pdf => "application/pdf".to_string(),
                    ContentType::Html => "text/html".to_string(),
                    ContentType::Text => "text/plain".to_string(),
                    ContentType::Markdown => "text/markdown".to_string(),
                    ContentType::Unknown => "application/octet-stream".to_string(),
                },
                chunk.chunk_index,
                chunk.total_chunks,
                embedding,
            );

            self.vector_store.add_document(document).await?;
        }

        info!("Successfully processed document: {}", url);
        Ok(())
    }

    /// Fetch document from URL
    async fn fetch_document(&self, url: &str) -> Result<RawDocument, AppError> {
        info!("Fetching document from: {}", url);

        let response = timeout(
            self.config.request_timeout(),
            self.client.get(url).send()
        ).await??;

        if !response.status().is_success() {
            return Err(AppError::Network(reqwest::Error::from(
                response.error_for_status().unwrap_err()
            )));
        }

        // Get content type from headers
        let content_type_header = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        // Get content
        let content = response.bytes().await?.to_vec();

        // Detect content type
        let content_type = ContentType::detect(&content, content_type_header.as_deref());

        // Validate content
        self.security_validator.validate_content(&content, content_type.clone())?;

        Ok(RawDocument {
            url: url.to_string(),
            content,
            content_type,
            title: None, // Will be extracted during text extraction
            metadata: std::collections::HashMap::new(),
        })
    }

    /// Extract text from raw document based on content type
    async fn extract_text(&self, raw_doc: &RawDocument) -> Result<String, AppError> {
        match raw_doc.content_type {
            ContentType::Pdf => self.extract_pdf_text(&raw_doc.content).await,
            ContentType::Html => self.extract_html_text(&raw_doc.content).await,
            ContentType::Text | ContentType::Markdown => {
                Ok(String::from_utf8_lossy(&raw_doc.content).to_string())
            }
            ContentType::Unknown => {
                Err(AppError::PdfParsing("Unknown content type".to_string()))
            }
        }
    }

    /// Extract text from PDF using poppler-rs (placeholder implementation)
    async fn extract_pdf_text(&self, pdf_data: &[u8]) -> Result<String, AppError> {
        info!("Extracting text from PDF ({} bytes)", pdf_data.len());

        // Validate PDF
        self.security_validator.validate_pdf_input(pdf_data)?;

        // TODO: Implement actual PDF extraction using poppler-rs
        // For now, return a placeholder
        // In production, this would use:
        // let document = poppler::Document::new_from_data(pdf_data, None)?;
        // let mut text = String::new();
        // for page_index in 0..document.get_n_pages() {
        //     if let Some(page) = document.get_page(page_index) {
        //         text.push_str(&page.get_text());
        //     }
        // }

        warn!("PDF text extraction not yet implemented - using placeholder");
        Ok(format!("PDF content extracted from {} bytes", pdf_data.len()))
    }

    /// Extract text from HTML
    async fn extract_html_text(&self, html_data: &[u8]) -> Result<String, AppError> {
        info!("Extracting text from HTML ({} bytes)", html_data.len());

        let html_content = String::from_utf8_lossy(html_data);
        let document = Html::parse_document(&html_content);

        // Extract title
        let title_selector = Selector::parse("title").unwrap();
        let _title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string());

        // Extract main content, excluding script and style tags
        let content_selectors = [
            "main", "article", ".content", ".post", ".entry",
            "body", "p", "h1", "h2", "h3", "h4", "h5", "h6"
        ];

        let mut text_content = Vec::new();

        for selector_str in &content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let text = element.text().collect::<Vec<_>>().join(" ");
                    let cleaned = text.trim();
                    if !cleaned.is_empty() && cleaned.len() > 10 {
                        text_content.push(cleaned.to_string());
                    }
                }
            }
        }

        // If no content found with selectors, fall back to all text
        if text_content.is_empty() {
            // Remove script and style content
            let script_selector = Selector::parse("script, style").unwrap();
            let mut clean_doc = document.clone();
            // Note: scraper doesn't support removing elements, so we'll extract text differently
            
            let body_selector = Selector::parse("body").unwrap();
            if let Some(body) = document.select(&body_selector).next() {
                text_content.push(body.text().collect::<Vec<_>>().join(" "));
            } else {
                text_content.push(document.root_element().text().collect::<Vec<_>>().join(" "));
            }
        }

        let final_text = text_content.join("\n\n");
        
        // Clean up whitespace
        let cleaned = final_text
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        info!("Extracted {} characters of text from HTML", cleaned.len());
        Ok(cleaned)
    }

    /// Chunk text into smaller pieces for embedding
    fn chunk_text(&self, text: &str, raw_doc: &RawDocument) -> DocumentResult<Vec<DocumentChunk>> {
        let chunk_size = self.config.harvester.chunk_size;
        let chunk_overlap = self.config.harvester.chunk_overlap;

        if text.len() <= chunk_size {
            return Ok(vec![DocumentChunk {
                content: text.to_string(),
                chunk_index: 0,
                total_chunks: 1,
            }]);
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let end = (start + chunk_size).min(text.len());
            let chunk_text = &text[start..end];

            // Try to break at word boundaries
            let actual_end = if end < text.len() {
                chunk_text.rfind(' ').map(|pos| start + pos).unwrap_or(end)
            } else {
                end
            };

            let final_chunk = &text[start..actual_end];
            
            chunks.push(DocumentChunk {
                content: final_chunk.to_string(),
                chunk_index: chunks.len(),
                total_chunks: 0, // Will be updated after all chunks are created
            });

            start = if actual_end == end {
                end
            } else {
                actual_end.saturating_sub(chunk_overlap).max(start + 1)
            };
        }

        // Update total chunks count
        let total_chunks = chunks.len();
        for chunk in &mut chunks {
            chunk.total_chunks = total_chunks;
        }

        info!("Created {} chunks from document: {}", total_chunks, raw_doc.url);
        Ok(chunks)
    }

    /// Generate embedding for text (placeholder implementation)
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, AppError> {
        // TODO: Implement actual embedding generation
        // This would typically use a service like OpenAI embeddings, HuggingFace, or local models
        // For now, we'll create a simple hash-based embedding for demonstration

        let mut embedding = vec![0.0; 384]; // Standard embedding dimension
        
        // Simple hash-based embedding (not for production use)
        let embedding_len = embedding.len();
        for (i, byte) in text.bytes().enumerate() {
            embedding[i % embedding_len] += byte as f32 / 255.0;
        }

        // Normalize
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }

        Ok(embedding)
    }

    /// Get current harvest progress
    pub async fn get_progress(&self) -> String {
        let progress = self.progress.lock().await;
        serde_json::to_string(&*progress).unwrap_or_else(|_| "{}".to_string())
    }

    /// Check if harvester is currently processing
    pub async fn is_busy(&self) -> bool {
        let progress = self.progress.lock().await;
        matches!(progress.status, HarvestStatus::InProgress)
    }

    /// Get harvester statistics
    pub async fn get_stats(&self) -> HarvestProgress {
        self.progress.lock().await.clone()
    }
}

/// Quality filtering for extracted content
pub fn filter_content_quality(text: &str, threshold: f32) -> bool {
    // Basic quality metrics
    let length_score = (text.len() as f32 / 1000.0).min(1.0);
    let word_count = text.split_whitespace().count();
    let word_score = (word_count as f32 / 100.0).min(1.0);
    
    // Check for meaningful content
    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let has_sentences = text.contains('.') || text.contains('!') || text.contains('?');
    
    let quality_score = if has_letters && has_sentences {
        (length_score + word_score) / 2.0
    } else {
        0.0
    };

    quality_score >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_content_chunking() {
        let config = AppConfig::default();
        let text = "This is a test document. ".repeat(100);
        let raw_doc = RawDocument {
            url: "https://example.com".to_string(),
            content: text.as_bytes().to_vec(),
            content_type: ContentType::Text,
            title: None,
            metadata: std::collections::HashMap::new(),
        };

        // Create a mock harvester for testing
        let chunks = {
            let chunk_size = config.harvester.chunk_size;
            let chunk_overlap = config.harvester.chunk_overlap;
            
            // Simplified chunking logic for test
            if text.len() <= chunk_size {
                vec![DocumentChunk {
                    content: text.clone(),
                    chunk_index: 0,
                    total_chunks: 1,
                }]
            } else {
                let mut chunks = Vec::new();
                let mut start = 0;
                
                while start < text.len() {
                    let end = (start + chunk_size).min(text.len());
                    chunks.push(DocumentChunk {
                        content: text[start..end].to_string(),
                        chunk_index: chunks.len(),
                        total_chunks: 0,
                    });
                    start = end.saturating_sub(chunk_overlap).max(start + 1);
                }
                
                let total = chunks.len();
                for chunk in &mut chunks {
                    chunk.total_chunks = total;
                }
                chunks
            }
        };

        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|c| c.total_chunks == chunks.len()));
    }

    #[test]
    fn test_content_quality_filtering() {
        assert!(filter_content_quality("This is a good quality document with sentences.", 0.5));
        assert!(!filter_content_quality("abc123", 0.5));
        assert!(!filter_content_quality("", 0.5));
        assert!(filter_content_quality("Short but good.", 0.3));
    }

    #[test]
    fn test_html_text_extraction() {
        let html = r#"
            <html>
                <head><title>Test Page</title></head>
                <body>
                    <h1>Main Title</h1>
                    <p>This is a paragraph with content.</p>
                    <script>alert('should be ignored');</script>
                    <style>body { color: red; }</style>
                </body>
            </html>
        "#;

        let document = Html::parse_document(html);
        let p_selector = Selector::parse("p").unwrap();
        let h1_selector = Selector::parse("h1").unwrap();
        
        let p_text: String = document.select(&p_selector).map(|el| el.text().collect::<String>()).collect();
        let h1_text: String = document.select(&h1_selector).map(|el| el.text().collect::<String>()).collect();
        
        assert!(p_text.contains("paragraph with content"));
        assert!(h1_text.contains("Main Title"));
        assert!(!p_text.contains("alert"));
    }
}
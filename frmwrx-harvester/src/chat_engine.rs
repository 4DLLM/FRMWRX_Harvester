// Chat engine implementation for FRMWRX Harvester
// AI processing layer with LLM interface, document context integration, and response generation

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::fs;
use tokio::time::{timeout, Duration};
use tracing::{info, warn, error};

use crate::config::AppConfig;
use crate::errors::{AppError, ChatEngineError, ChatResult};
use crate::vector_store::{Document, SearchResult, VectorStore};

/// Chat message representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context_documents: Option<Vec<String>>, // Document IDs used for context
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Chat conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory {
    pub messages: VecDeque<ChatMessage>,
    pub max_history: usize,
    pub session_id: String,
}

/// LLM API request/response structures
#[derive(Debug, Serialize)]
struct LlmRequest {
    model: String,
    messages: Vec<LlmMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct LlmMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct LlmResponse {
    content: Vec<LlmContent>,
}

#[derive(Debug, Deserialize)]
struct LlmContent {
    text: String,
}

/// Context building result
#[derive(Debug)]
pub struct ChatContext {
    pub system_prompt: String,
    pub user_query: String,
    pub relevant_documents: Vec<Document>,
    pub chat_history: Vec<ChatMessage>,
    pub total_context_length: usize,
}

/// LLM client interface
pub struct LlmClient {
    client: Client,
    config: crate::config::LlmConfig,
}

impl LlmClient {
    pub fn new(config: crate::config::LlmConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Generate response using LLM API
    pub async fn generate_response(&self, context: &ChatContext) -> ChatResult<String> {
        // Prepare messages for LLM
        let mut messages = vec![
            LlmMessage {
                role: "system".to_string(),
                content: context.system_prompt.clone(),
            }
        ];

        // Add chat history
        for msg in &context.chat_history {
            messages.push(LlmMessage {
                role: match msg.role {
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "assistant".to_string(),
                    MessageRole::System => "system".to_string(),
                },
                content: msg.content.clone(),
            });
        }

        // Add current user query
        messages.push(LlmMessage {
            role: "user".to_string(),
            content: context.user_query.clone(),
        });

        let request = LlmRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        // Make API request with retries
        let mut attempts = 0;
        let max_attempts = self.config.retry_attempts;

        while attempts < max_attempts {
            match self.make_api_request(&request).await {
                Ok(response) => {
                    info!("LLM response generated successfully");
                    return Ok(response);
                }
                Err(e) => {
                    attempts += 1;
                    warn!("LLM API attempt {} failed: {}", attempts, e);
                    
                    if attempts >= max_attempts {
                        return Err(ChatEngineError::LlmGeneration(format!(
                            "Failed after {} attempts: {}", max_attempts, e
                        )));
                    }
                    
                    // Exponential backoff
                    let delay = Duration::from_millis(1000 * (1 << attempts));
                    tokio::time::sleep(delay).await;
                }
            }
        }

        Err(ChatEngineError::LlmGeneration("Max retries exceeded".to_string()))
    }

    async fn make_api_request(&self, request: &LlmRequest) -> Result<String, AppError> {
        // Check if API key is available
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| AppError::LlmApi("No API key configured".to_string()))?;

        let response = timeout(
            Duration::from_secs(self.config.timeout_secs),
            self.client
                .post(&self.config.api_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .header("anthropic-version", "2023-06-01")
                .json(request)
                .send()
        ).await??;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::LlmApi(format!("API error: {}", error_text)));
        }

        let llm_response: LlmResponse = response.json().await?;
        
        if llm_response.content.is_empty() {
            return Err(AppError::LlmApi("Empty response from LLM".to_string()));
        }

        Ok(llm_response.content[0].text.clone())
    }

    /// Generate a simple response without API (fallback)
    pub async fn generate_fallback_response(&self, context: &ChatContext) -> String {
        warn!("Using fallback response generation");
        
        let doc_count = context.relevant_documents.len();
        if doc_count > 0 {
            format!(
                "I found {} relevant document(s) related to your query: '{}'. \
                The documents contain information about various topics. \
                However, I'm currently using a fallback response system. \
                To get more detailed answers, please configure an LLM API key.",
                doc_count,
                context.user_query
            )
        } else {
            format!(
                "I understand you're asking about: '{}'. \
                However, I couldn't find any relevant documents in the knowledge base. \
                Try harvesting some documents first, or rephrase your query.",
                context.user_query
            )
        }
    }
}

impl ChatHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_history,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push_back(message);
        
        // Keep only the most recent messages
        while self.messages.len() > self.max_history {
            self.messages.pop_front();
        }
    }

    pub fn get_recent_messages(&self, count: usize) -> Vec<ChatMessage> {
        self.messages
            .iter()
            .rev()
            .take(count)
            .rev()
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

/// Main chat engine
pub struct ChatEngine {
    llm_client: LlmClient,
    vector_store: Arc<VectorStore>,
    chat_history: ChatHistory,
    config: AppConfig,
}

impl ChatEngine {
    /// Create a new chat engine
    pub async fn new(config: AppConfig, vector_store: Arc<VectorStore>) -> Result<Self, AppError> {
        info!("Initializing chat engine");

        let llm_client = LlmClient::new(config.llm.clone());
        let chat_history = ChatHistory::new(20); // Keep last 20 messages

        Ok(Self {
            llm_client,
            vector_store,
            chat_history,
            config,
        })
    }

    /// Process a user query and generate response (main implementation from architecture)
    pub async fn process_query(&mut self, query: String) -> Result<String, AppError> {
        info!("Processing chat query: {}", query);

        // 1. Search relevant documents
        let relevant_docs = self.vector_store
            .search(&query, 5)
            .await?;

        // 2. Build context from documents + chat history
        let context = self.build_context(&query, &relevant_docs).await?;

        // 3. Generate response
        let response = if self.config.llm.api_key.is_some() {
            match self.llm_client.generate_response(&context).await {
                Ok(response) => response,
                Err(e) => {
                    warn!("LLM generation failed, using fallback: {}", e);
                    self.llm_client.generate_fallback_response(&context).await
                }
            }
        } else {
            self.llm_client.generate_fallback_response(&context).await
        };

        // 4. Save to history
        self.add_exchange(query, response.clone()).await?;

        info!("Chat response generated successfully");
        Ok(response)
    }

    /// Build context from query, documents, and chat history
    async fn build_context(&self, query: &str, relevant_docs: &[Document]) -> ChatResult<ChatContext> {
        // Build system prompt with document context
        let mut system_prompt = String::from(
            "You are FRMWRX, an intelligent assistant with access to a knowledge base of documents. \
            Use the provided documents to answer questions accurately and comprehensively. \
            If the documents don't contain enough information, say so clearly.\n\n"
        );

        if !relevant_docs.is_empty() {
            system_prompt.push_str("RELEVANT DOCUMENTS:\n");
            for (i, doc) in relevant_docs.iter().enumerate() {
                system_prompt.push_str(&format!(
                    "\n--- Document {} (from {}) ---\n{}\n",
                    i + 1,
                    doc.url,
                    doc.content
                ));
            }
            system_prompt.push_str("\n---\n\n");
        } else {
            system_prompt.push_str("No relevant documents found in the knowledge base.\n\n");
        }

        system_prompt.push_str(
            "Please provide a helpful response based on the available information. \
            If using information from the documents, mention which document(s) you're referencing."
        );

        // Get recent chat history for context
        let chat_history = self.chat_history.get_recent_messages(10);

        // Calculate context length
        let total_context_length = system_prompt.len() 
            + query.len() 
            + chat_history.iter().map(|msg| msg.content.len()).sum::<usize>()
            + relevant_docs.iter().map(|doc| doc.content.len()).sum::<usize>();

        Ok(ChatContext {
            system_prompt,
            user_query: query.to_string(),
            relevant_documents: relevant_docs.to_vec(),
            chat_history,
            total_context_length,
        })
    }

    /// Add a question-answer exchange to chat history
    async fn add_exchange(&mut self, query: String, response: String) -> ChatResult<()> {
        let document_ids = vec![]; // TODO: Track which documents were used

        // Add user message
        self.chat_history.add_message(ChatMessage {
            role: MessageRole::User,
            content: query,
            timestamp: chrono::Utc::now(),
            context_documents: None,
        });

        // Add assistant response
        self.chat_history.add_message(ChatMessage {
            role: MessageRole::Assistant,
            content: response,
            timestamp: chrono::Utc::now(),
            context_documents: Some(document_ids),
        });

        // Save history to disk
        self.save_history().await?;

        Ok(())
    }

    /// Save chat history to disk
    async fn save_history(&self) -> ChatResult<()> {
        let history_path = &self.config.storage.chat_history_path;
        
        if let Some(parent) = history_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| ChatEngineError::ChatHistory(e.to_string()))?;
        }

        let serialized = serde_json::to_string_pretty(&self.chat_history)
            .map_err(|e| ChatEngineError::ChatHistory(e.to_string()))?;

        fs::write(history_path, serialized).await
            .map_err(|e| ChatEngineError::ChatHistory(e.to_string()))?;

        Ok(())
    }

    /// Load chat history from disk
    pub async fn load_history(&mut self) -> ChatResult<()> {
        let history_path = &self.config.storage.chat_history_path;
        
        if !history_path.exists() {
            info!("No existing chat history found");
            return Ok(());
        }

        let content = fs::read_to_string(history_path).await
            .map_err(|e| ChatEngineError::ChatHistory(e.to_string()))?;

        let history: ChatHistory = serde_json::from_str(&content)
            .map_err(|e| ChatEngineError::ChatHistory(e.to_string()))?;

        self.chat_history = history;
        info!("Chat history loaded successfully");
        
        Ok(())
    }

    /// Clear chat history
    pub async fn clear_history(&mut self) -> ChatResult<()> {
        self.chat_history.clear();
        self.save_history().await?;
        info!("Chat history cleared");
        Ok(())
    }

    /// Get chat history
    pub fn get_history(&self) -> &ChatHistory {
        &self.chat_history
    }

    /// Get chat statistics
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "message_count": self.chat_history.messages.len(),
            "session_id": self.chat_history.session_id,
            "max_history": self.chat_history.max_history,
            "llm_model": self.config.llm.model,
            "api_configured": self.config.llm.api_key.is_some(),
        })
    }

    /// Search documents (convenience method)
    pub async fn search_documents(&self, query: &str, limit: usize) -> Result<Vec<Document>, AppError> {
        self.vector_store.search(query, limit).await
            .map_err(|e| AppError::VectorDB(e.to_string()))
    }
}

/// Utility functions for prompt engineering
pub fn create_system_prompt(context_docs: &[Document]) -> String {
    let mut prompt = String::from(
        "You are FRMWRX, an advanced AI assistant with access to a curated knowledge base. \
        Your role is to provide accurate, helpful, and contextual responses based on the available documents.\n\n"
    );

    if !context_docs.is_empty() {
        prompt.push_str("INSTRUCTIONS:\n");
        prompt.push_str("- Use the provided documents as your primary source of information\n");
        prompt.push_str("- Cite specific documents when referencing information\n");
        prompt.push_str("- If information is not available in the documents, state this clearly\n");
        prompt.push_str("- Provide comprehensive but concise responses\n");
        prompt.push_str("- Maintain a professional and helpful tone\n\n");

        prompt.push_str("AVAILABLE DOCUMENTS:\n");
        for (i, doc) in context_docs.iter().enumerate() {
            prompt.push_str(&format!(
                "Document {}: {} (from {})\n",
                i + 1,
                doc.title.as_deref().unwrap_or("Untitled"),
                doc.url
            ));
        }
        prompt.push('\n');
    } else {
        prompt.push_str("No documents are currently available in the knowledge base. ");
        prompt.push_str("Please inform the user that they may need to harvest documents first.\n\n");
    }

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_chat_history() {
        let mut history = ChatHistory::new(3);
        
        // Add messages
        for i in 0..5 {
            history.add_message(ChatMessage {
                role: if i % 2 == 0 { MessageRole::User } else { MessageRole::Assistant },
                content: format!("Message {}", i),
                timestamp: chrono::Utc::now(),
                context_documents: None,
            });
        }

        // Should only keep the last 3 messages
        assert_eq!(history.messages.len(), 3);
        assert_eq!(history.messages[0].content, "Message 2");
        assert_eq!(history.messages[2].content, "Message 4");
    }

    #[test]
    fn test_system_prompt_creation() {
        let docs = vec![
            Document::new(
                "https://example.com".to_string(),
                Some("Test Doc".to_string()),
                "Test content".to_string(),
                "text/plain".to_string(),
                0,
                1,
                vec![0.1, 0.2, 0.3],
            )
        ];

        let prompt = create_system_prompt(&docs);
        assert!(prompt.contains("FRMWRX"));
        assert!(prompt.contains("Test Doc"));
        assert!(prompt.contains("https://example.com"));
    }

    #[tokio::test]
    async fn test_context_building() {
        let temp_dir = tempdir().unwrap();
        let config = AppConfig {
            storage: crate::config::StorageConfig {
                vector_db_path: temp_dir.path().join("vector_db"),
                document_cache_path: temp_dir.path().join("docs"),
                chat_history_path: temp_dir.path().join("history.json"),
                max_cache_size_mb: 100,
                auto_cleanup: false,
            },
            ..AppConfig::default()
        };

        let vector_store = Arc::new(
            VectorStore::new(&config.storage.vector_db_path).await.unwrap()
        );

        let mut chat_engine = ChatEngine::new(config, vector_store).await.unwrap();

        let docs = vec![
            Document::new(
                "https://example.com".to_string(),
                Some("Test".to_string()),
                "Sample content".to_string(),
                "text/plain".to_string(),
                0,
                1,
                vec![0.1; 384],
            )
        ];

        let context = chat_engine.build_context("test query", &docs).await.unwrap();
        
        assert!(!context.system_prompt.is_empty());
        assert_eq!(context.user_query, "test query");
        assert_eq!(context.relevant_documents.len(), 1);
        assert!(context.total_context_length > 0);
    }
}
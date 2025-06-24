use crate::{ChatMessage, config::AppConfig};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub id: String,
    pub content: String,
    pub source: String,
    pub score: f32,
}

pub struct ChatEngine {
    config: Arc<AppConfig>,
    chat_history: Vec<ChatMessage>,
    // Vector store would go here when implemented
    // vector_store: Arc<VectorStore>,
}

impl ChatEngine {
    pub async fn new(config: Arc<AppConfig>) -> Result<Self> {
        Ok(Self {
            config,
            chat_history: Vec::new(),
        })
    }

    pub async fn process_query(&mut self, query: String) -> Result<String> {
        // Add user message to history
        let user_message = ChatMessage {
            id: Uuid::new_v4().to_string(),
            content: query.clone(),
            sender: "user".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        self.chat_history.push(user_message);

        // For now, simulate AI response processing
        // In a real implementation, this would:
        // 1. Search relevant documents using vector similarity
        // 2. Build context from found documents
        // 3. Send to LLM (OpenAI, Anthropic, or local model)
        // 4. Return the AI response

        let response = self.generate_mock_response(&query).await?;

        // Add assistant response to history
        let assistant_message = ChatMessage {
            id: Uuid::new_v4().to_string(),
            content: response.clone(),
            sender: "assistant".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        self.chat_history.push(assistant_message);

        Ok(response)
    }

    async fn generate_mock_response(&self, query: &str) -> Result<String> {
        // Mock AI response - replace with actual LLM integration
        let response = if query.to_lowercase().contains("document") {
            "I can help you with document-related queries. Currently, I'm a demonstration of the FRMWRX chat interface. When fully implemented, I'll be able to search through harvested documents and provide contextual responses based on their content."
        } else if query.to_lowercase().contains("search") {
            "The search functionality will allow you to find relevant information across all harvested documents using vector similarity search powered by FAISS."
        } else if query.to_lowercase().contains("help") {
            "Welcome to FRMWRX Document Harvester! I'm here to help you interact with your document collection. You can ask me to search for information, explain concepts from your documents, or help you navigate your knowledge base."
        } else {
            &format!("Thank you for your message: '{}'. I'm currently in demonstration mode. When fully implemented, I'll provide intelligent responses based on your harvested documents and advanced AI capabilities.", query)
        };

        // Simulate some processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        Ok(response.to_string())
    }

    pub async fn search_documents(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        // Mock document search - replace with actual vector search
        let mock_results = vec![
            format!("Document 1: Related to '{}'", query),
            format!("Document 2: Contains information about '{}'", query),
            format!("Document 3: Discusses concepts similar to '{}'", query),
        ];

        Ok(mock_results.into_iter().take(limit).collect())
    }

    pub fn get_chat_history(&self) -> Vec<ChatMessage> {
        self.chat_history.clone()
    }

    pub fn clear_history(&mut self) {
        self.chat_history.clear();
    }

    pub async fn add_context_documents(&mut self, documents: Vec<DocumentChunk>) -> Result<()> {
        // This would add documents to the vector store for context
        // Implementation would go here when vector database is integrated
        Ok(())
    }

    fn build_context(&self, query: &str, relevant_docs: &[DocumentChunk]) -> String {
        let mut context = String::new();
        
        // Add recent chat history for context
        let recent_messages: Vec<&ChatMessage> = self.chat_history
            .iter()
            .rev()
            .take(6) // Last 3 exchanges
            .collect();

        if !recent_messages.is_empty() {
            context.push_str("Recent conversation:\n");
            for message in recent_messages.iter().rev() {
                context.push_str(&format!("{}: {}\n", message.sender, message.content));
            }
            context.push_str("\n");
        }

        // Add relevant documents
        if !relevant_docs.is_empty() {
            context.push_str("Relevant documents:\n");
            for doc in relevant_docs {
                context.push_str(&format!("Source: {}\n{}\n\n", doc.source, doc.content));
            }
        }

        context.push_str(&format!("User question: {}\n", query));
        context
    }
}
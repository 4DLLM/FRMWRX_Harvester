use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub security: SecurityConfig,
    pub harvester: HarvesterConfig,
    pub chat: ChatConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub max_pdf_size: usize,
    pub max_concurrent_downloads: usize,
    pub request_timeout_secs: u64,
    pub enable_sandboxing: bool,
    pub allowed_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvesterConfig {
    pub user_agent: String,
    pub max_retries: usize,
    pub retry_delay_ms: u64,
    pub chunk_size: usize,
    pub max_content_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    pub max_history_length: usize,
    pub max_context_documents: usize,
    pub response_timeout_secs: u64,
    pub enable_streaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_directory: String,
    pub vector_index_name: String,
    pub enable_persistence: bool,
    pub max_cache_size_mb: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            security: SecurityConfig::default(),
            harvester: HarvesterConfig::default(),
            chat: ChatConfig::default(),
            storage: StorageConfig::default(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_pdf_size: 50 * 1024 * 1024, // 50MB
            max_concurrent_downloads: 20,
            request_timeout_secs: 30,
            enable_sandboxing: true,
            allowed_domains: vec![], // Empty = allow all
        }
    }
}

impl Default for HarvesterConfig {
    fn default() -> Self {
        Self {
            user_agent: "FRMWRX-Harvester/1.0".to_string(),
            max_retries: 3,
            retry_delay_ms: 1000,
            chunk_size: 1000, // characters per chunk
            max_content_length: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            max_history_length: 50, // messages
            max_context_documents: 10,
            response_timeout_secs: 30,
            enable_streaming: false,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_directory: "./storage".to_string(),
            vector_index_name: "documents".to_string(),
            enable_persistence: true,
            max_cache_size_mb: 512,
        }
    }
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.security.request_timeout_secs)
    }

    pub fn retry_delay(&self) -> Duration {
        Duration::from_millis(self.harvester.retry_delay_ms)
    }

    pub fn response_timeout(&self) -> Duration {
        Duration::from_secs(self.chat.response_timeout_secs)
    }
}
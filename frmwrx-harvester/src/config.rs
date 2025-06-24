// Configuration management for FRMWRX Harvester
// Security configuration and application settings

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tracing::{info, warn};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub security: SecurityConfig,
    pub storage: StorageConfig,
    pub llm: LlmConfig,
    pub harvester: HarvesterConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub max_pdf_size: usize,
    pub max_concurrent_downloads: usize,
    pub request_timeout_secs: u64,
    pub enable_sandboxing: bool,
    pub allowed_domains: Vec<String>,
    pub max_content_length: usize,
    pub validate_ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub vector_db_path: PathBuf,
    pub document_cache_path: PathBuf,
    pub chat_history_path: PathBuf,
    pub max_cache_size_mb: usize,
    pub auto_cleanup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub api_key: Option<String>,
    pub api_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvesterConfig {
    pub max_concurrent_documents: usize,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub quality_threshold: f32,
    pub enable_ocr: bool,
    pub supported_formats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub theme: String,
    pub enable_dev_tools: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_pdf_size: 50 * 1024 * 1024, // 50MB
            max_concurrent_downloads: 20,
            request_timeout_secs: 30,
            enable_sandboxing: true,
            allowed_domains: vec![], // Empty = allow all
            max_content_length: 100 * 1024 * 1024, // 100MB
            validate_ssl: true,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            vector_db_path: PathBuf::from("./storage/vector_db"),
            document_cache_path: PathBuf::from("./storage/documents"),
            chat_history_path: PathBuf::from("./storage/chat_history.json"),
            max_cache_size_mb: 1024, // 1GB
            auto_cleanup: true,
        }
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            api_url: "https://api.anthropic.com/v1/messages".to_string(),
            model: "claude-3-sonnet-20240229".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            timeout_secs: 60,
            retry_attempts: 3,
        }
    }
}

impl Default for HarvesterConfig {
    fn default() -> Self {
        Self {
            max_concurrent_documents: 10,
            chunk_size: 1000,
            chunk_overlap: 200,
            quality_threshold: 0.5,
            enable_ocr: false,
            supported_formats: vec![
                "pdf".to_string(),
                "html".to_string(),
                "txt".to_string(),
                "md".to_string(),
            ],
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            window_width: 1200,
            window_height: 800,
            theme: "dark".to_string(),
            enable_dev_tools: cfg!(debug_assertions),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            security: SecurityConfig::default(),
            storage: StorageConfig::default(),
            llm: LlmConfig::default(),
            harvester: HarvesterConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl AppConfig {
    pub async fn load() -> Result<Self, AppError> {
        let config_path = Path::new("config.json");
        
        if config_path.exists() {
            info!("Loading configuration from config.json");
            let content = fs::read_to_string(config_path).await?;
            let config: Self = serde_json::from_str(&content)?;
            config.validate()?;
            Ok(config)
        } else {
            warn!("No config.json found, using default configuration");
            let config = Self::default();
            config.save().await?;
            Ok(config)
        }
    }

    pub async fn save(&self) -> Result<(), AppError> {
        info!("Saving configuration to config.json");
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = Path::new("config.json").parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write("config.json", content).await?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), AppError> {
        // Validate security settings
        if self.security.max_pdf_size > 200 * 1024 * 1024 {
            return Err(AppError::Config(
                "max_pdf_size cannot exceed 200MB".to_string()
            ));
        }

        if self.security.max_concurrent_downloads > 50 {
            return Err(AppError::Config(
                "max_concurrent_downloads cannot exceed 50".to_string()
            ));
        }

        // Validate storage paths
        if self.storage.vector_db_path.to_string_lossy().is_empty() {
            return Err(AppError::Config(
                "vector_db_path cannot be empty".to_string()
            ));
        }

        // Validate LLM config
        if self.llm.max_tokens > 100_000 {
            return Err(AppError::Config(
                "max_tokens cannot exceed 100,000".to_string()
            ));
        }

        if !(0.0..=2.0).contains(&self.llm.temperature) {
            return Err(AppError::Config(
                "temperature must be between 0.0 and 2.0".to_string()
            ));
        }

        // Validate harvester config
        if self.harvester.chunk_size < 100 {
            return Err(AppError::Config(
                "chunk_size must be at least 100".to_string()
            ));
        }

        if self.harvester.chunk_overlap >= self.harvester.chunk_size {
            return Err(AppError::Config(
                "chunk_overlap must be less than chunk_size".to_string()
            ));
        }

        Ok(())
    }

    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.security.request_timeout_secs)
    }

    pub fn llm_timeout(&self) -> Duration {
        Duration::from_secs(self.llm.timeout_secs)
    }

    pub async fn ensure_directories(&self) -> Result<(), AppError> {
        info!("Ensuring storage directories exist");
        
        let directories = [
            &self.storage.vector_db_path,
            &self.storage.document_cache_path,
        ];

        for dir in directories {
            if let Some(parent) = dir.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::create_dir_all(dir).await?;
        }

        // Ensure chat history directory exists
        if let Some(parent) = self.storage.chat_history_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        Ok(())
    }
}
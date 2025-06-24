// Error handling for FRMWRX Harvester
// Comprehensive error types for all application components

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("PDF parsing error: {0}")]
    PdfParsing(String),
    
    #[error("Vector database error: {0}")]
    VectorDB(String),
    
    #[error("Security violation: {0}")]
    Security(#[from] SecurityError),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("LLM API error: {0}")]
    LlmApi(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
    
    #[error("Operation timed out: {0}")]
    Elapsed(#[from] tokio::time::error::Elapsed),
}

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("File too large: {size} bytes exceeds maximum {max_size} bytes")]
    FileTooLarge { size: usize, max_size: usize },
    
    #[error("Invalid file format: expected {expected}, got {actual}")]
    InvalidFileFormat { expected: String, actual: String },
    
    #[error("Unsafe URL detected: {url}")]
    UnsafeUrl { url: String },
    
    #[error("Domain not allowed: {domain}")]
    DomainNotAllowed { domain: String },
    
    #[error("Content validation failed: {reason}")]
    ContentValidationFailed { reason: String },
    
    #[error("Rate limit exceeded: {limit} requests per {window}")]
    RateLimitExceeded { limit: u32, window: String },
}

#[derive(Error, Debug)]
pub enum DocumentProcessingError {
    #[error("Failed to extract text from document: {0}")]
    TextExtraction(String),
    
    #[error("Failed to chunk document: {0}")]
    Chunking(String),
    
    #[error("Failed to generate embeddings: {0}")]
    Embedding(String),
    
    #[error("Document type not supported: {mime_type}")]
    UnsupportedType { mime_type: String },
    
    #[error("Document processing timeout after {seconds} seconds")]
    ProcessingTimeout { seconds: u64 },
}

#[derive(Error, Debug)]
pub enum VectorStoreError {
    #[error("Failed to initialize vector store: {0}")]
    Initialization(String),
    
    #[error("Failed to add document to vector store: {0}")]
    AddDocument(String),
    
    #[error("Failed to search vector store: {0}")]
    Search(String),
    
    #[error("Failed to save vector store: {0}")]
    Save(String),
    
    #[error("Failed to load vector store: {0}")]
    Load(String),
    
    #[error("Vector store corruption detected: {0}")]
    Corruption(String),
}

#[derive(Error, Debug)]
pub enum ChatEngineError {
    #[error("Failed to generate LLM response: {0}")]
    LlmGeneration(String),
    
    #[error("Failed to build context: {0}")]
    ContextBuilding(String),
    
    #[error("Chat history error: {0}")]
    ChatHistory(String),
    
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}

// Result type aliases for convenience
pub type AppResult<T> = Result<T, AppError>;
pub type SecurityResult<T> = Result<T, SecurityError>;
pub type DocumentResult<T> = Result<T, DocumentProcessingError>;
pub type VectorResult<T> = Result<T, VectorStoreError>;
pub type ChatResult<T> = Result<T, ChatEngineError>;

// Conversion implementations
impl From<DocumentProcessingError> for AppError {
    fn from(err: DocumentProcessingError) -> Self {
        AppError::Generic(anyhow::anyhow!(err))
    }
}

impl From<VectorStoreError> for AppError {
    fn from(err: VectorStoreError) -> Self {
        AppError::VectorDB(err.to_string())
    }
}

impl From<ChatEngineError> for AppError {
    fn from(err: ChatEngineError) -> Self {
        AppError::LlmApi(err.to_string())
    }
}
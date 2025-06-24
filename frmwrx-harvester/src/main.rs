// FRMWRX Rust Chatbot + Document Harvester
// Main application entry point

use anyhow::Result;
use std::sync::Arc;
use tauri::{command, generate_context, generate_handler, Builder, State};
use tokio::sync::Mutex;
use tracing::{info, error};

// Import all modules
mod config;
mod document_harvester;
mod chat_engine;
mod vector_store;
mod errors;
mod security;

use config::{AppConfig, SecurityConfig};
use document_harvester::DocumentHarvester;
use chat_engine::ChatEngine;
use vector_store::VectorStore;
use errors::AppError;

// Application state that will be shared across all Tauri commands
pub struct AppState {
    pub config: AppConfig,
    pub harvester: Arc<Mutex<DocumentHarvester>>,
    pub chat_engine: Arc<Mutex<ChatEngine>>,
    pub vector_store: Arc<VectorStore>,
}

#[command]
async fn start_harvest(
    urls: Vec<String>,
    state: State<'_, AppState>
) -> Result<String, String> {
    info!("Starting document harvest for {} URLs", urls.len());
    
    let harvester = state.harvester.lock().await;
    match harvester.harvest_documents(urls).await {
        Ok(_) => {
            info!("Document harvest completed successfully");
            Ok("Harvest completed successfully".to_string())
        }
        Err(e) => {
            error!("Document harvest failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
async fn send_chat_message(
    message: String,
    state: State<'_, AppState>
) -> Result<String, String> {
    info!("Processing chat message: {}", message);
    
    let mut chat_engine = state.chat_engine.lock().await;
    match chat_engine.process_query(message).await {
        Ok(response) => {
            info!("Chat response generated successfully");
            Ok(response)
        }
        Err(e) => {
            error!("Chat processing failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
async fn get_harvest_progress(
    state: State<'_, AppState>
) -> Result<String, String> {
    let harvester = state.harvester.lock().await;
    Ok(harvester.get_progress().await)
}

#[command]
async fn search_documents(
    query: String,
    limit: Option<usize>,
    state: State<'_, AppState>
) -> Result<Vec<String>, String> {
    info!("Searching documents for: {}", query);
    
    let limit = limit.unwrap_or(5);
    match state.vector_store.search(&query, limit).await {
        Ok(results) => Ok(results.iter().map(|doc| doc.content.clone()).collect()),
        Err(e) => {
            error!("Document search failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
async fn get_system_status(
    state: State<'_, AppState>
) -> Result<String, String> {
    let status = serde_json::json!({
        "status": "running",
        "document_count": state.vector_store.document_count().await,
        "config": {
            "max_pdf_size": state.config.security.max_pdf_size,
            "max_concurrent_downloads": state.config.security.max_concurrent_downloads,
        }
    });
    
    Ok(status.to_string())
}

async fn initialize_app() -> Result<AppState> {
    info!("Initializing FRMWRX Harvester application...");

    // Load configuration
    let config = AppConfig::load().await?;
    info!("Configuration loaded successfully");

    // Initialize vector store
    let vector_store = Arc::new(VectorStore::new(&config.storage.vector_db_path).await?);
    info!("Vector store initialized");

    // Initialize document harvester
    let harvester = Arc::new(Mutex::new(
        DocumentHarvester::new(config.clone(), vector_store.clone()).await?
    ));
    info!("Document harvester initialized");

    // Initialize chat engine
    let chat_engine = Arc::new(Mutex::new(
        ChatEngine::new(config.clone(), vector_store.clone()).await?
    ));
    info!("Chat engine initialized");

    Ok(AppState {
        config,
        harvester,
        chat_engine,
        vector_store,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("frmwrx_harvester=info")
        .init();

    info!("FRMWRX Rust Chatbot + Document Harvester starting...");

    // Initialize the application state
    let app_state = initialize_app().await
        .map_err(|e| {
            error!("Failed to initialize application: {}", e);
            e
        })?;

    info!("Application initialized successfully, starting Tauri...");

    // Build and run the Tauri application
    Builder::default()
        .manage(app_state)
        .invoke_handler(generate_handler![
            start_harvest,
            send_chat_message,
            get_harvest_progress,
            search_documents,
            get_system_status
        ])
        .setup(|app| {
            info!("Tauri application setup complete");
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
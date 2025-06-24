use tauri::{Manager, State};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod chat_engine;
mod document_harvester;
mod config;

use chat_engine::ChatEngine;
use document_harvester::DocumentHarvester;
use config::AppConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub sender: String, // "user" or "assistant"
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HarvestProgress {
    pub total_documents: usize,
    pub processed_documents: usize,
    pub current_document: Option<String>,
    pub status: String,
}

// Application state
pub struct AppState {
    pub chat_engine: Arc<Mutex<ChatEngine>>,
    pub document_harvester: Arc<Mutex<DocumentHarvester>>,
    pub config: Arc<AppConfig>,
}

#[tauri::command]
async fn send_chat_message(
    message: String,
    state: State<'_, AppState>,
) -> Result<ChatResponse, String> {
    let timestamp = chrono::Utc::now().timestamp();
    
    // Create user message
    let user_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        content: message.clone(),
        sender: "user".to_string(),
        timestamp,
    };

    // Process the message through chat engine
    let mut chat_engine = state.chat_engine.lock().map_err(|e| e.to_string())?;
    
    match chat_engine.process_query(message).await {
        Ok(response_content) => {
            let assistant_message = ChatMessage {
                id: Uuid::new_v4().to_string(),
                content: response_content,
                sender: "assistant".to_string(),
                timestamp: timestamp + 1,
            };

            Ok(ChatResponse {
                message: assistant_message,
                success: true,
                error: None,
            })
        }
        Err(e) => {
            Ok(ChatResponse {
                message: ChatMessage {
                    id: Uuid::new_v4().to_string(),
                    content: "I encountered an error processing your message. Please try again.".to_string(),
                    sender: "assistant".to_string(),
                    timestamp: timestamp + 1,
                },
                success: false,
                error: Some(e.to_string()),
            })
        }
    }
}

#[tauri::command]
async fn start_document_harvest(
    urls: Vec<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut harvester = state.document_harvester.lock().map_err(|e| e.to_string())?;
    
    match harvester.harvest_documents(urls).await {
        Ok(_) => Ok("Document harvest completed successfully".to_string()),
        Err(e) => Err(format!("Harvest failed: {}", e)),
    }
}

#[tauri::command]
async fn get_harvest_progress(
    state: State<'_, AppState>,
) -> Result<HarvestProgress, String> {
    let harvester = state.document_harvester.lock().map_err(|e| e.to_string())?;
    Ok(harvester.get_progress())
}

#[tauri::command]
async fn search_documents(
    query: String,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let chat_engine = state.chat_engine.lock().map_err(|e| e.to_string())?;
    
    match chat_engine.search_documents(&query, limit.unwrap_or(10)).await {
        Ok(results) => Ok(results),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_chat_history(
    state: State<'_, AppState>,
) -> Result<Vec<ChatMessage>, String> {
    let chat_engine = state.chat_engine.lock().map_err(|e| e.to_string())?;
    Ok(chat_engine.get_chat_history())
}

#[tauri::command]
async fn clear_chat_history(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut chat_engine = state.chat_engine.lock().map_err(|e| e.to_string())?;
    chat_engine.clear_history();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();

    // Create application config
    let config = Arc::new(AppConfig::default());

    // Initialize components
    let chat_engine = Arc::new(Mutex::new(ChatEngine::new(config.clone()).await?));
    let document_harvester = Arc::new(Mutex::new(DocumentHarvester::new(config.clone()).await?));

    // Create app state
    let app_state = AppState {
        chat_engine,
        document_harvester,
        config,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            send_chat_message,
            start_document_harvest,
            get_harvest_progress,
            search_documents,
            get_chat_history,
            clear_chat_history
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
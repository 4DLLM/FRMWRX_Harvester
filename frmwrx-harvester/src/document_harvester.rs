use crate::{HarvestProgress, config::AppConfig};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub content_type: String,
    pub size: usize,
    pub processed_at: i64,
}

pub struct DocumentHarvester {
    config: Arc<AppConfig>,
    client: reqwest::Client,
    progress: HarvestProgress,
}

impl DocumentHarvester {
    pub async fn new(config: Arc<AppConfig>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("FRMWRX-Harvester/1.0")
            .build()?;

        Ok(Self {
            config,
            client,
            progress: HarvestProgress {
                total_documents: 0,
                processed_documents: 0,
                current_document: None,
                status: "Ready".to_string(),
            },
        })
    }

    pub async fn harvest_documents(&mut self, urls: Vec<String>) -> Result<()> {
        self.progress.total_documents = urls.len();
        self.progress.processed_documents = 0;
        self.progress.status = "Processing".to_string();

        for (index, url) in urls.iter().enumerate() {
            self.progress.current_document = Some(url.clone());
            
            match self.process_single_document(url).await {
                Ok(document) => {
                    self.progress.processed_documents = index + 1;
                    tracing::info!("Successfully processed document: {}", url);
                    
                    // Here you would store the document in your vector database
                    // self.store_document(document).await?;
                }
                Err(e) => {
                    tracing::warn!("Failed to process document {}: {}", url, e);
                    // Continue processing other documents even if one fails
                }
            }
        }

        self.progress.status = "Completed".to_string();
        self.progress.current_document = None;

        Ok(())
    }

    async fn process_single_document(&self, url: &str) -> Result<Document> {
        // Validate URL
        if !self.is_valid_url(url) {
            return Err(anyhow!("Invalid URL: {}", url));
        }

        // Fetch document
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("HTTP error {}: {}", response.status(), url));
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let content_bytes = response.bytes().await?;
        
        // Process based on content type
        let (title, content) = match self.detect_content_type(&content_type, &content_bytes) {
            ContentType::Html => self.process_html(&content_bytes).await?,
            ContentType::Pdf => self.process_pdf(&content_bytes).await?,
            ContentType::Text => self.process_text(&content_bytes).await?,
            ContentType::Unknown => return Err(anyhow!("Unsupported content type: {}", content_type)),
        };

        Ok(Document {
            id: uuid::Uuid::new_v4().to_string(),
            url: url.to_string(),
            title,
            content,
            content_type,
            size: content_bytes.len(),
            processed_at: chrono::Utc::now().timestamp(),
        })
    }

    fn detect_content_type(&self, content_type: &str, _data: &[u8]) -> ContentType {
        if content_type.contains("text/html") {
            ContentType::Html
        } else if content_type.contains("application/pdf") {
            ContentType::Pdf
        } else if content_type.contains("text/plain") {
            ContentType::Text
        } else {
            ContentType::Unknown
        }
    }

    async fn process_html(&self, data: &[u8]) -> Result<(Option<String>, String)> {
        let html = String::from_utf8_lossy(data);
        
        // Simple HTML parsing - in production, use a proper HTML parser like scraper
        let title = self.extract_html_title(&html);
        let content = self.extract_html_content(&html);
        
        Ok((title, content))
    }

    async fn process_pdf(&self, _data: &[u8]) -> Result<(Option<String>, String)> {
        // PDF processing would use poppler-rs here
        // For now, return placeholder
        Ok((
            Some("PDF Document".to_string()),
            "PDF content extraction not yet implemented. This would use poppler-rs to extract text from PDF documents.".to_string()
        ))
    }

    async fn process_text(&self, data: &[u8]) -> Result<(Option<String>, String)> {
        let content = String::from_utf8_lossy(data).to_string();
        Ok((None, content))
    }

    fn extract_html_title(&self, html: &str) -> Option<String> {
        // Simple title extraction - use proper HTML parser in production
        if let Some(start) = html.find("<title>") {
            if let Some(end) = html[start + 7..].find("</title>") {
                return Some(html[start + 7..start + 7 + end].trim().to_string());
            }
        }
        None
    }

    fn extract_html_content(&self, html: &str) -> String {
        // Simple content extraction - use proper HTML parser in production
        // This is a very basic implementation
        let mut content = html.to_string();
        
        // Remove scripts and styles
        while let Some(start) = content.find("<script") {
            if let Some(end) = content[start..].find("</script>") {
                content.replace_range(start..start + end + 9, "");
            } else {
                break;
            }
        }
        
        while let Some(start) = content.find("<style") {
            if let Some(end) = content[start..].find("</style>") {
                content.replace_range(start..start + end + 8, "");
            } else {
                break;
            }
        }
        
        // Remove HTML tags (very basic)
        let mut result = String::new();
        let mut in_tag = false;
        
        for char in content.chars() {
            match char {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ => if !in_tag { result.push(char); }
            }
        }
        
        // Clean up whitespace
        result.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn is_valid_url(&self, url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }

    pub fn get_progress(&self) -> HarvestProgress {
        self.progress.clone()
    }
}

#[derive(Debug)]
enum ContentType {
    Html,
    Pdf,
    Text,
    Unknown,
}
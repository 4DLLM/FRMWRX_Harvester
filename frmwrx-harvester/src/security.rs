// Security module for FRMWRX Harvester
// Input validation, sandboxing, and security enforcement

use std::time::Duration;
use tokio::time::timeout;
use tracing::{warn, error};
use url::Url;

use crate::config::SecurityConfig;
use crate::errors::{SecurityError, SecurityResult};

/// Maximum file size constants
const MAX_PDF_SIZE: usize = 200 * 1024 * 1024; // 200MB absolute max
const MAX_HTML_SIZE: usize = 50 * 1024 * 1024;  // 50MB for HTML
const MAX_TEXT_SIZE: usize = 10 * 1024 * 1024;  // 10MB for text

/// Content type detection and validation
#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Pdf,
    Html,
    Text,
    Markdown,
    Unknown,
}

impl ContentType {
    pub fn detect(data: &[u8], content_type_header: Option<&str>) -> Self {
        // Check magic numbers first
        if data.starts_with(b"%PDF") {
            return ContentType::Pdf;
        }
        
        if data.starts_with(b"<!DOCTYPE html") || data.starts_with(b"<html") {
            return ContentType::Html;
        }
        
        // Check content type header
        if let Some(ct) = content_type_header {
            let ct = ct.to_lowercase();
            if ct.contains("application/pdf") {
                return ContentType::Pdf;
            } else if ct.contains("text/html") {
                return ContentType::Html;
            } else if ct.contains("text/markdown") {
                return ContentType::Markdown;
            } else if ct.contains("text/plain") {
                return ContentType::Text;
            }
        }
        
        // Try to detect from content
        let text_sample = String::from_utf8_lossy(&data[..data.len().min(1024)]);
        if text_sample.contains("# ") || text_sample.contains("## ") {
            return ContentType::Markdown;
        }
        
        // Default to text if it looks like valid UTF-8
        if String::from_utf8(data.to_vec()).is_ok() {
            ContentType::Text
        } else {
            ContentType::Unknown
        }
    }
    
    pub fn max_size(&self) -> usize {
        match self {
            ContentType::Pdf => MAX_PDF_SIZE,
            ContentType::Html => MAX_HTML_SIZE,
            ContentType::Text | ContentType::Markdown => MAX_TEXT_SIZE,
            ContentType::Unknown => 1024, // Very restrictive for unknown types
        }
    }
}

/// Security validator for input validation and sandboxing
pub struct SecurityValidator {
    config: SecurityConfig,
}

impl SecurityValidator {
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }

    /// Validate PDF input with security checks
    pub fn validate_pdf_input(&self, data: &[u8]) -> SecurityResult<()> {
        // Size limits
        if data.len() > self.config.max_pdf_size {
            return Err(SecurityError::FileTooLarge {
                size: data.len(),
                max_size: self.config.max_pdf_size,
            });
        }

        // Magic number validation
        if !data.starts_with(b"%PDF") {
            return Err(SecurityError::InvalidFileFormat {
                expected: "PDF".to_string(),
                actual: "Unknown".to_string(),
            });
        }

        // Check for suspicious patterns
        let content = String::from_utf8_lossy(data);
        if content.contains("/JS") || content.contains("/JavaScript") {
            warn!("PDF contains JavaScript, proceeding with caution");
        }

        if content.contains("/Launch") || content.contains("/URI") {
            warn!("PDF contains external references, proceeding with caution");
        }

        Ok(())
    }

    /// Validate URL for safety
    pub fn validate_url(&self, url: &str) -> SecurityResult<Url> {
        let parsed_url = Url::parse(url)
            .map_err(|_| SecurityError::UnsafeUrl { url: url.to_string() })?;

        // Check scheme
        match parsed_url.scheme() {
            "http" | "https" => {},
            _ => return Err(SecurityError::UnsafeUrl { url: url.to_string() }),
        }

        // Check domain restrictions if configured
        if !self.config.allowed_domains.is_empty() {
            if let Some(domain) = parsed_url.domain() {
                let allowed = self.config.allowed_domains.iter()
                    .any(|allowed_domain| domain.ends_with(allowed_domain));
                
                if !allowed {
                    return Err(SecurityError::DomainNotAllowed {
                        domain: domain.to_string(),
                    });
                }
            }
        }

        // Check for suspicious patterns in URL
        let url_str = parsed_url.as_str();
        if url_str.contains("../") || url_str.contains("..\\") {
            return Err(SecurityError::UnsafeUrl { url: url.to_string() });
        }

        Ok(parsed_url)
    }

    /// Validate content based on type and size
    pub fn validate_content(&self, data: &[u8], content_type: ContentType) -> SecurityResult<()> {
        let max_size = content_type.max_size().min(self.config.max_content_length);
        
        if data.len() > max_size {
            return Err(SecurityError::FileTooLarge {
                size: data.len(),
                max_size,
            });
        }

        match content_type {
            ContentType::Pdf => self.validate_pdf_input(data)?,
            ContentType::Html => self.validate_html_content(data)?,
            ContentType::Text | ContentType::Markdown => self.validate_text_content(data)?,
            ContentType::Unknown => {
                return Err(SecurityError::ContentValidationFailed {
                    reason: "Unknown content type not allowed".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate HTML content
    fn validate_html_content(&self, data: &[u8]) -> SecurityResult<()> {
        let content = String::from_utf8_lossy(data);
        
        // Check for suspicious script content
        if content.to_lowercase().contains("<script") {
            warn!("HTML contains script tags, proceeding with caution");
        }

        // Check for suspicious external references
        if content.contains("javascript:") || content.contains("data:") {
            warn!("HTML contains suspicious URLs, proceeding with caution");
        }

        Ok(())
    }

    /// Validate text content
    fn validate_text_content(&self, data: &[u8]) -> SecurityResult<()> {
        // Ensure it's valid UTF-8
        String::from_utf8(data.to_vec())
            .map_err(|_| SecurityError::ContentValidationFailed {
                reason: "Invalid UTF-8 encoding".to_string(),
            })?;

        Ok(())
    }

    /// Sandboxed processing with timeout
    pub async fn process_with_timeout<F, R, E>(&self, operation: F) -> SecurityResult<R>
    where
        F: std::future::Future<Output = Result<R, E>>,
        E: std::fmt::Display,
    {
        let timeout_duration = Duration::from_secs(self.config.request_timeout_secs);
        
        match timeout(timeout_duration, operation).await {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(e)) => Err(SecurityError::ContentValidationFailed {
                reason: format!("Operation failed: {}", e),
            }),
            Err(_) => Err(SecurityError::ContentValidationFailed {
                reason: format!("Operation timed out after {} seconds", self.config.request_timeout_secs),
            }),
        }
    }

    /// Check if content appears to be malicious
    pub fn scan_for_malicious_content(&self, content: &str) -> SecurityResult<()> {
        let suspicious_patterns = [
            r"eval\s*\(",
            r"document\.cookie",
            r"window\.location",
            r"<script[^>]*>.*?</script>",
            r"javascript:",
            r"vbscript:",
            r"data:text/html",
            r"onclick\s*=",
            r"onerror\s*=",
        ];

        for pattern in &suspicious_patterns {
            if regex::Regex::new(pattern)
                .unwrap()
                .is_match(&content.to_lowercase())
            {
                warn!("Detected suspicious pattern: {}", pattern);
                // Note: We warn but don't block, as legitimate content might contain these
            }
        }

        Ok(())
    }

    /// Rate limiting check (placeholder for future implementation)
    pub fn check_rate_limit(&self, _client_id: &str) -> SecurityResult<()> {
        // TODO: Implement proper rate limiting with Redis or in-memory store
        Ok(())
    }
}

/// Secure PDF processing function
pub async fn process_pdf_secure(
    pdf_data: &[u8], 
    validator: &SecurityValidator
) -> SecurityResult<String> {
    // Validate input
    validator.validate_pdf_input(pdf_data)?;
    
    // Process in timeout wrapper
    validator.process_with_timeout::<_, String, SecurityError>(async {
        // Note: This is a placeholder for actual PDF processing
        // In a real implementation, you would use poppler-rs here
        Ok::<String, SecurityError>("PDF content extracted".to_string())
    }).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_detection() {
        let pdf_data = b"%PDF-1.4\n...";
        assert_eq!(ContentType::detect(pdf_data, None), ContentType::Pdf);

        let html_data = b"<!DOCTYPE html><html>...";
        assert_eq!(ContentType::detect(html_data, None), ContentType::Html);

        let text_data = b"This is plain text";
        assert_eq!(ContentType::detect(text_data, Some("text/plain")), ContentType::Text);
    }

    #[test]
    fn test_url_validation() {
        let config = SecurityConfig::default();
        let validator = SecurityValidator::new(config);

        assert!(validator.validate_url("https://example.com").is_ok());
        assert!(validator.validate_url("http://example.com").is_ok());
        assert!(validator.validate_url("ftp://example.com").is_err());
        assert!(validator.validate_url("javascript:alert(1)").is_err());
    }

    #[test]
    fn test_pdf_validation() {
        let config = SecurityConfig::default();
        let validator = SecurityValidator::new(config);

        let valid_pdf = b"%PDF-1.4\nSome PDF content";
        assert!(validator.validate_pdf_input(valid_pdf).is_ok());

        let invalid_pdf = b"Not a PDF";
        assert!(validator.validate_pdf_input(invalid_pdf).is_err());

        let oversized_pdf = vec![0u8; 100 * 1024 * 1024]; // 100MB
        assert!(validator.validate_pdf_input(&oversized_pdf).is_err());
    }
}
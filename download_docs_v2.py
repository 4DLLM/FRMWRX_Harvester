#!/usr/bin/env python3
"""
FRMWRX Documentation Downloader v2
Downloads and organizes documentation for the Rust tech stack with proper HTML parsing
"""

import os
import sys
import requests
import time
from pathlib import Path
from urllib.parse import urljoin, urlparse
from bs4 import BeautifulSoup
import re
import json
from typing import Dict, List, Optional, Tuple
import logging
import html2text

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('download_docs_v2.log'),
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

class DocumentationDownloaderV2:
    def __init__(self, base_dir: str = "frmwrx-harvester"):
        self.base_dir = Path(base_dir)
        self.docs_dir = self.base_dir / "docs"
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'
        })
        
        # HTML to Markdown converter
        self.h2t = html2text.HTML2Text()
        self.h2t.ignore_links = False
        self.h2t.ignore_images = False
        self.h2t.body_width = 0  # No line wrapping
        
        self.setup_directories()
    
    def setup_directories(self):
        """Create the directory structure"""
        directories = [
            "rust", "tokio", "tauri", "reqwest", "poppler-rs", "faiss", "serde"
        ]
        
        for dir_name in directories:
            (self.docs_dir / dir_name).mkdir(parents=True, exist_ok=True)
        
        logger.info(f"Created directory structure in {self.docs_dir}")
    
    def download_with_retry(self, url: str, max_retries: int = 3) -> Optional[requests.Response]:
        """Download content with retry logic"""
        for attempt in range(max_retries):
            try:
                logger.info(f"Downloading {url} (attempt {attempt + 1})")
                response = self.session.get(url, timeout=30)
                response.raise_for_status()
                return response
            except requests.RequestException as e:
                logger.warning(f"Attempt {attempt + 1} failed: {e}")
                if attempt < max_retries - 1:
                    time.sleep(2 ** attempt)  # Exponential backoff
                else:
                    logger.error(f"Failed to download {url} after {max_retries} attempts")
                    return None
    
    def extract_main_content(self, soup: BeautifulSoup, url: str) -> str:
        """Extract main content from HTML, removing navigation and UI elements"""
        # Remove common navigation and UI elements
        selectors_to_remove = [
            'nav', 'header', 'footer', '.nav', '.navigation', '.sidebar',
            '.menu', '.breadcrumb', '.search', '.theme-switcher',
            'script', 'style', '.advertisement', '.ads'
        ]
        
        for selector in selectors_to_remove:
            for element in soup.select(selector):
                element.decompose()
        
        # Try to find main content area
        main_selectors = [
            'main', '.main', '#main', '.content', '#content',
            '.documentation', '#documentation', 'article', '.article'
        ]
        
        main_content = None
        for selector in main_selectors:
            main_content = soup.select_one(selector)
            if main_content:
                break
        
        # If no main content found, use body
        if not main_content:
            main_content = soup.find('body') or soup
        
        # Convert to markdown
        markdown = self.h2t.handle(str(main_content))
        
        # Clean up the markdown
        markdown = self.clean_markdown(markdown)
        
        return markdown
    
    def clean_markdown(self, markdown: str) -> str:
        """Clean up markdown content"""
        # Remove excessive whitespace
        markdown = re.sub(r'\n\s*\n\s*\n', '\n\n', markdown)
        
        # Remove empty lines at start and end
        markdown = markdown.strip()
        
        # Fix common issues
        markdown = re.sub(r'\[([^\]]+)\]\(#\)', r'\1', markdown)  # Remove empty links
        markdown = re.sub(r'\[([^\]]+)\]\(javascript:void\(0\)\)', r'\1', markdown)
        
        return markdown
    
    def save_markdown(self, content: str, filepath: Path, source_url: str):
        """Save content as markdown file"""
        header = f"# {filepath.stem.replace('-', ' ').title()}\n\n"
        header += f"*Source: {source_url}*\n\n"
        header += "---\n\n"
        
        full_content = header + content
        
        try:
            filepath.write_text(full_content, encoding='utf-8')
            logger.info(f"✅ Saved {filepath}")
            return True
        except Exception as e:
            logger.error(f"❌ Failed to save {filepath}: {e}")
            return False
    
    def download_rust_docs(self):
        """Download Rust documentation"""
        logger.info("Downloading Rust documentation...")
        
        rust_docs = {
            "rust-book.md": "https://doc.rust-lang.org/book/",
            "cargo-guide.md": "https://doc.rust-lang.org/cargo/",
            "std-library.md": "https://doc.rust-lang.org/std/"
        }
        
        for filename, url in rust_docs.items():
            response = self.download_with_retry(url)
            if response:
                soup = BeautifulSoup(response.content, 'html.parser')
                content = self.extract_main_content(soup, url)
                filepath = self.docs_dir / "rust" / filename
                self.save_markdown(content, filepath, url)
            else:
                logger.error(f"Failed to download {filename}")
    
    def download_tokio_docs(self):
        """Download Tokio documentation"""
        logger.info("Downloading Tokio documentation...")
        
        tokio_docs = {
            "tokio-tutorial.md": "https://tokio.rs/tokio/tutorial",
            "async-book.md": "https://rust-lang.github.io/async-book/"
        }
        
        for filename, url in tokio_docs.items():
            response = self.download_with_retry(url)
            if response:
                soup = BeautifulSoup(response.content, 'html.parser')
                content = self.extract_main_content(soup, url)
                filepath = self.docs_dir / "tokio" / filename
                self.save_markdown(content, filepath, url)
            else:
                logger.error(f"Failed to download {filename}")
    
    def download_tauri_docs(self):
        """Download Tauri documentation"""
        logger.info("Downloading Tauri documentation...")
        
        tauri_docs = {
            "tauri-guide.md": "https://tauri.app/v1/guides/",
            "api-reference.md": "https://docs.rs/tauri/"
        }
        
        for filename, url in tauri_docs.items():
            response = self.download_with_retry(url)
            if response:
                soup = BeautifulSoup(response.content, 'html.parser')
                content = self.extract_main_content(soup, url)
                filepath = self.docs_dir / "tauri" / filename
                self.save_markdown(content, filepath, url)
            else:
                logger.error(f"Failed to download {filename}")
    
    def download_reqwest_docs(self):
        """Download reqwest documentation"""
        logger.info("Downloading reqwest documentation...")
        url = "https://docs.rs/reqwest/"
        response = self.download_with_retry(url)
        if response:
            soup = BeautifulSoup(response.content, 'html.parser')
            content = self.extract_main_content(soup, url)
            filepath = self.docs_dir / "reqwest" / "reqwest-docs.md"
            self.save_markdown(content, filepath, url)
        else:
            logger.error("Failed to download reqwest docs")
    
    def download_poppler_docs(self):
        """Download poppler-rs documentation"""
        logger.info("Downloading poppler-rs documentation...")
        url = "https://docs.rs/poppler-rs/"
        response = self.download_with_retry(url)
        if response:
            soup = BeautifulSoup(response.content, 'html.parser')
            content = self.extract_main_content(soup, url)
            filepath = self.docs_dir / "poppler-rs" / "poppler-api.md"
            self.save_markdown(content, filepath, url)
        else:
            logger.error("Failed to download poppler-rs docs")
    
    def download_faiss_docs(self):
        """Download FAISS Rust bindings documentation"""
        logger.info("Downloading FAISS Rust documentation...")
        github_url = "https://raw.githubusercontent.com/Enet4/faiss-rs/master/README.md"
        response = self.download_with_retry(github_url)
        if response:
            content = f"# FAISS Rust Bindings\n\n"
            content += f"*Source: FAISS Rust GitHub Repository (Enet4/faiss-rs)*\n\n"
            content += "---\n\n"
            content += response.text
            filepath = self.docs_dir / "faiss" / "faiss-rust.md"
            self.save_markdown(content, filepath, github_url)
        else:
            logger.error("Failed to download FAISS docs")
    
    def download_serde_docs(self):
        """Download serde documentation"""
        logger.info("Downloading serde documentation...")
        url = "https://serde.rs/"
        response = self.download_with_retry(url)
        if response:
            soup = BeautifulSoup(response.content, 'html.parser')
            content = self.extract_main_content(soup, url)
            filepath = self.docs_dir / "serde" / "serde-guide.md"
            self.save_markdown(content, filepath, url)
        else:
            logger.error("Failed to download serde docs")
    
    def run(self):
        """Run the complete download process"""
        logger.info("Starting FRMWRX documentation download...")
        
        download_methods = [
            self.download_rust_docs,
            self.download_tokio_docs,
            self.download_tauri_docs,
            self.download_reqwest_docs,
            self.download_poppler_docs,
            self.download_faiss_docs,
            self.download_serde_docs
        ]
        
        success_count = 0
        total_count = len(download_methods)
        
        for method in download_methods:
            try:
                method()
                success_count += 1
            except Exception as e:
                logger.error(f"Error in {method.__name__}: {e}")
        
        logger.info(f"Download complete! {success_count}/{total_count} documentation sets processed.")
        
        # Verify downloads
        self.verify_downloads()
    
    def verify_downloads(self):
        """Verify that files were downloaded and have content"""
        logger.info("Verifying downloads...")
        
        expected_files = [
            "rust/rust-book.md", "rust/cargo-guide.md", "rust/std-library.md",
            "tokio/tokio-tutorial.md", "tokio/async-book.md",
            "tauri/tauri-guide.md", "tauri/api-reference.md",
            "reqwest/reqwest-docs.md", "poppler-rs/poppler-api.md",
            "faiss/faiss-rust.md", "serde/serde-guide.md"
        ]
        
        for file_path in expected_files:
            full_path = self.docs_dir / file_path
            if full_path.exists():
                size = full_path.stat().st_size
                if size > 100:  # At least 100 bytes
                    logger.info(f"✅ {file_path} ({size} bytes)")
                else:
                    logger.warning(f"⚠️  {file_path} is very small ({size} bytes)")
            else:
                logger.error(f"❌ {file_path} missing")

if __name__ == "__main__":
    downloader = DocumentationDownloaderV2()
    downloader.run() 
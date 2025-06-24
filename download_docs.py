#!/usr/bin/env python3
"""
FRMWRX Documentation Downloader
Downloads and organizes documentation for the Rust tech stack
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

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('download_docs.log'),
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

class DocumentationDownloader:
    def __init__(self, base_dir: str = "frmwrx-harvester"):
        self.base_dir = Path(base_dir)
        self.docs_dir = self.base_dir / "docs"
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'
        })
        
        # Create directories if they don't exist
        self._create_directories()
    
    def _create_directories(self):
        """Create the docs directory structure"""
        directories = [
            "rust", "tokio", "tauri", "reqwest", 
            "poppler-rs", "faiss", "serde"
        ]
        
        for dir_name in directories:
            (self.docs_dir / dir_name).mkdir(parents=True, exist_ok=True)
            logger.info(f"Created directory: {self.docs_dir / dir_name}")
    
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
    
    def clean_html_to_markdown(self, html_content: str, title: str = "") -> str:
        """Convert HTML content to markdown"""
        soup = BeautifulSoup(html_content, 'html.parser')
        
        # Remove script and style elements
        for script in soup(["script", "style"]):
            script.decompose()
        
        # Extract text and basic formatting
        text = soup.get_text()
        
        # Clean up whitespace
        lines = (line.strip() for line in text.splitlines())
        chunks = (phrase.strip() for line in lines for phrase in line.split("  "))
        text = '\n'.join(chunk for chunk in chunks if chunk)
        
        # Create markdown content
        markdown_content = f"# {title}\n\n"
        markdown_content += f"*Source: {title}*\n\n"
        markdown_content += "---\n\n"
        markdown_content += text
        
        return markdown_content
    
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
                content = self.clean_html_to_markdown(response.text, filename.replace('.md', ''))
                filepath = self.docs_dir / "rust" / filename
                filepath.write_text(content, encoding='utf-8')
                logger.info(f"✓ Downloaded {filename}")
            else:
                logger.error(f"✗ Failed to download {filename}")
    
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
                content = self.clean_html_to_markdown(response.text, filename.replace('.md', ''))
                filepath = self.docs_dir / "tokio" / filename
                filepath.write_text(content, encoding='utf-8')
                logger.info(f"✓ Downloaded {filename}")
            else:
                logger.error(f"✗ Failed to download {filename}")
    
    def download_tauri_docs(self):
        """Download Tauri documentation"""
        logger.info("Downloading Tauri documentation...")
        
        tauri_docs = {
            "tauri-guide.md": "https://tauri.app/v1/guides/",
            "api-reference.md": "https://tauri.app/v1/api/"
        }
        
        for filename, url in tauri_docs.items():
            response = self.download_with_retry(url)
            if response:
                content = self.clean_html_to_markdown(response.text, filename.replace('.md', ''))
                filepath = self.docs_dir / "tauri" / filename
                filepath.write_text(content, encoding='utf-8')
                logger.info(f"✓ Downloaded {filename}")
            else:
                logger.error(f"✗ Failed to download {filename}")
    
    def download_reqwest_docs(self):
        """Download reqwest documentation"""
        logger.info("Downloading reqwest documentation...")
        
        url = "https://docs.rs/reqwest/"
        response = self.download_with_retry(url)
        if response:
            content = self.clean_html_to_markdown(response.text, "reqwest")
            filepath = self.docs_dir / "reqwest" / "reqwest-docs.md"
            filepath.write_text(content, encoding='utf-8')
            logger.info("✓ Downloaded reqwest-docs.md")
        else:
            logger.error("✗ Failed to download reqwest documentation")
    
    def download_poppler_rs_docs(self):
        """Download poppler-rs documentation"""
        logger.info("Downloading poppler-rs documentation...")
        
        url = "https://docs.rs/poppler-rs/"
        response = self.download_with_retry(url)
        if response:
            content = self.clean_html_to_markdown(response.text, "poppler-rs")
            filepath = self.docs_dir / "poppler-rs" / "poppler-api.md"
            filepath.write_text(content, encoding='utf-8')
            logger.info("✓ Downloaded poppler-api.md")
        else:
            logger.error("✗ Failed to download poppler-rs documentation")
    
    def download_faiss_docs(self):
        """Download FAISS Rust bindings documentation"""
        logger.info("Downloading FAISS Rust documentation...")
        
        # Try to get FAISS Rust bindings from GitHub
        github_url = "https://raw.githubusercontent.com/facebookresearch/faiss-rs/main/README.md"
        response = self.download_with_retry(github_url)
        if response:
            content = f"# FAISS Rust Bindings\n\n"
            content += f"*Source: FAISS Rust GitHub Repository*\n\n"
            content += "---\n\n"
            content += response.text
            filepath = self.docs_dir / "faiss" / "faiss-rust.md"
            filepath.write_text(content, encoding='utf-8')
            logger.info("✓ Downloaded faiss-rust.md")
        else:
            logger.error("✗ Failed to download FAISS documentation")
    
    def download_serde_docs(self):
        """Download serde documentation"""
        logger.info("Downloading serde documentation...")
        
        url = "https://serde.rs/"
        response = self.download_with_retry(url)
        if response:
            content = self.clean_html_to_markdown(response.text, "serde")
            filepath = self.docs_dir / "serde" / "serde-guide.md"
            filepath.write_text(content, encoding='utf-8')
            logger.info("✓ Downloaded serde-guide.md")
        else:
            logger.error("✗ Failed to download serde documentation")
    
    def verify_downloads(self) -> Dict[str, List[str]]:
        """Verify that all downloads were successful"""
        logger.info("Verifying downloads...")
        
        expected_files = {
            "rust": ["rust-book.md", "cargo-guide.md", "std-library.md"],
            "tokio": ["tokio-tutorial.md", "async-book.md"],
            "tauri": ["tauri-guide.md", "api-reference.md"],
            "reqwest": ["reqwest-docs.md"],
            "poppler-rs": ["poppler-api.md"],
            "faiss": ["faiss-rust.md"],
            "serde": ["serde-guide.md"]
        }
        
        results = {"success": [], "missing": []}
        
        for category, files in expected_files.items():
            for filename in files:
                filepath = self.docs_dir / category / filename
                if filepath.exists() and filepath.stat().st_size > 0:
                    results["success"].append(f"{category}/{filename}")
                    logger.info(f"✓ Verified: {category}/{filename}")
                else:
                    results["missing"].append(f"{category}/{filename}")
                    logger.error(f"✗ Missing or empty: {category}/{filename}")
        
        return results
    
    def run(self):
        """Run the complete documentation download process"""
        logger.info("Starting FRMWRX documentation download...")
        logger.info(f"Base directory: {self.base_dir.absolute()}")
        
        try:
            # Download all documentation
            self.download_rust_docs()
            self.download_tokio_docs()
            self.download_tauri_docs()
            self.download_reqwest_docs()
            self.download_poppler_rs_docs()
            self.download_faiss_docs()
            self.download_serde_docs()
            
            # Verify downloads
            results = self.verify_downloads()
            
            # Print summary
            logger.info("\n" + "="*50)
            logger.info("DOWNLOAD SUMMARY")
            logger.info("="*50)
            logger.info(f"Successfully downloaded: {len(results['success'])} files")
            logger.info(f"Missing or failed: {len(results['missing'])} files")
            
            if results['success']:
                logger.info("\nSuccessfully downloaded:")
                for file in results['success']:
                    logger.info(f"  ✓ {file}")
            
            if results['missing']:
                logger.info("\nMissing or failed:")
                for file in results['missing']:
                    logger.info(f"  ✗ {file}")
            
            logger.info("\nDocumentation download complete!")
            
        except Exception as e:
            logger.error(f"Unexpected error during download: {e}")
            raise

def main():
    """Main entry point"""
    if len(sys.argv) > 1:
        base_dir = sys.argv[1]
    else:
        base_dir = "frmwrx-harvester"
    
    downloader = DocumentationDownloader(base_dir)
    downloader.run()

if __name__ == "__main__":
    main() 
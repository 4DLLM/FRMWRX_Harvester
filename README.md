# FRMWRX Harvester 🚀

**Enterprise AI-Powered Document Harvesting & Knowledge Management Platform**

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![React](https://img.shields.io/badge/react-%2320232a.svg?style=for-the-badge&logo=react&logoColor=%2361DAFB)
![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)

## 🎯 Overview

FRMWRX Harvester is a high-performance, cross-platform application that combines advanced document processing with AI-powered chat capabilities. Built with Rust for maximum performance and React for modern UI/UX, it provides enterprise-grade document harvesting, intelligent knowledge base management, and context-aware AI interactions.

## ✨ Key Features

### 🔍 **Intelligent Document Harvesting**
- **Concurrent Processing**: Multi-threaded document fetching with configurable batch sizes
- **Multi-Format Support**: HTML, PDF, and various document formats
- **Smart Content Extraction**: Advanced parsing with content quality filtering
- **Real-Time Progress Monitoring**: Live statistics and processing updates

### 🤖 **AI-Powered Chat Interface**
- **Context-Aware Responses**: AI chat with automatic document context integration
- **Conversation Memory**: Persistent chat history and session management
- **Fallback Handling**: Graceful degradation when external services are unavailable
- **Flexible LLM Integration**: Support for multiple AI providers (OpenAI, etc.)

### 🗄️ **Vector Knowledge Base**
- **Similarity Search**: Advanced vector-based document retrieval
- **In-Memory Performance**: Optimized for speed with optional persistence
- **Semantic Understanding**: Content embeddings for intelligent matching
- **Scalable Storage**: Ready for external vector database integration

### 🛡️ **Enterprise Security**
- **Input Validation**: Comprehensive security checks and sanitization
- **Sandboxed Processing**: Isolated execution environments
- **Rate Limiting**: Configurable request throttling
- **Content Filtering**: Malicious content detection and prevention

### 🎨 **Modern User Interface**
- **Cross-Platform Desktop**: Native app for Windows, macOS, and Linux
- **Responsive Design**: Modern React-based interface
- **Real-Time Updates**: Live progress monitoring and status indicators
- **Intuitive Workflow**: Streamlined document processing and chat interface

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   React UI      │    │   Tauri Bridge   │    │  Rust Backend   │
│                 │◄──►│                  │◄──►│                 │
│ • Chat Interface│    │ • API Commands   │    │ • Doc Harvester │
│ • Progress View │    │ • Event System   │    │ • Vector Store  │
│ • Knowledge Hub │    │ • Security Layer │    │ • Chat Engine   │
│ • Settings      │    │ • File System    │    │ • Security Core │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Core Components

- **Document Harvester**: High-performance web scraping engine with concurrent processing
- **Vector Store**: Intelligent document storage with semantic search capabilities  
- **Chat Engine**: AI integration layer with context management and conversation flow
- **Security Framework**: Comprehensive validation, sandboxing, and threat prevention
- **Configuration System**: Flexible JSON-based settings with validation and persistence

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70+ with Cargo
- **Node.js** 18+ with npm
- **System Dependencies**: WebKit, GTK (Linux), WebView2 (Windows)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/4DLLM/FRMWRX_Harvester.git
   cd FRMWRX_Harvester
   ```

2. **Install Tauri CLI**
   ```bash
   cargo install tauri-cli --version "^1.0"
   ```

3. **Install dependencies**
   ```bash
   # Rust dependencies
   cd frmwrx-harvester
   cargo build
   
   # Frontend dependencies  
   cd ui
   npm install
   cd ..
   ```

4. **Run the application**
   ```bash
   # Development mode
   cargo tauri dev
   
   # OR frontend only
   cd ui && npm run dev
   ```

## 🔧 Configuration

### Basic Configuration

Create or modify `config.json` in the application directory:

```json
{
  "security": {
    "input_validation": true,
    "max_file_size_mb": 10,
    "timeout_seconds": 30,
    "allowed_domains": ["*"]
  },
  "harvester": {
    "concurrent_requests": 10,
    "batch_size": 5,
    "retry_attempts": 3
  },
  "llm": {
    "provider": "openai",
    "model": "gpt-3.5-turbo",
    "api_key": "your-api-key-here",
    "max_tokens": 2048
  },
  "storage": {
    "documents_path": "./data/documents",
    "vector_db_path": "./data/vectors",
    "enable_persistence": true
  }
}
```

### Environment Variables

```bash
# LLM Configuration
OPENAI_API_KEY=your_openai_api_key
LLM_MODEL=gpt-3.5-turbo

# Security Settings
FRMWRX_MAX_FILE_SIZE=10485760
FRMWRX_TIMEOUT=30

# Storage Paths
FRMWRX_DATA_DIR=./data
FRMWRX_CONFIG_PATH=./config.json
```

## � Usage Guide

### 1. Document Harvesting

1. **Navigate to Harvest Tab**
2. **Add URLs**: Enter single URLs or paste multiple URLs (one per line)
3. **Configure Settings**: Adjust batch size and concurrent requests if needed
4. **Start Harvesting**: Click "Start Harvest" and monitor real-time progress
5. **Review Results**: Check processing statistics and any errors

### 2. AI Chat Interface

1. **Open Chat Tab**
2. **Ask Questions**: Type queries related to your harvested documents
3. **Context Integration**: The AI automatically uses relevant documents as context
4. **Review Responses**: Get intelligent answers based on your knowledge base

### 3. Knowledge Base Management

1. **Browse Tab**: Explore harvested documents
2. **Search**: Find specific content using text or semantic search
3. **Document Preview**: View content and metadata
4. **Filter & Sort**: Organize documents by relevance, date, or source

## 🏭 Production Deployment

### Build for Production

```bash
# Create optimized builds
cargo tauri build

# Outputs will be in:
# - Windows: target/release/bundle/msi/
# - macOS: target/release/bundle/dmg/
# - Linux: target/release/bundle/deb/ and bundle/rpm/
```

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
RUN apt-get update && apt-get install -y nodejs npm
COPY . /app
WORKDIR /app/frmwrx-harvester
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/frmwrx-harvester/target/release/frmwrx-harvester /usr/local/bin/
EXPOSE 3000
CMD ["frmwrx-harvester"]
```

### System Requirements

- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 1GB for application, additional space for document storage
- **CPU**: Multi-core processor for optimal performance
- **Network**: Internet connection for document harvesting and AI services

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test document_harvester
cargo test vector_store
cargo test chat_engine
cargo test security

# Frontend tests
cd ui && npm test
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org/) for performance and safety
- UI powered by [React](https://reactjs.org/) and [Tauri](https://tauri.app/)
- Vector operations inspired by modern ML practices
- Security framework follows industry best practices

---

<div align="center">
  <strong>Built with ❤️ for intelligent document processing</strong>
  <br>
  <sub>FRMWRX Harvester - Where AI meets Document Intelligence</sub>
</div> 
# FRMWRX Rust Chatbot + Document Harvester

A high-performance Rust-based proof of concept that combines intelligent document harvesting with an AI-powered chatbot, designed to demonstrate the core technologies that will power the FRMWRX consciousness-aware AI platform.

## 🚀 Features

- **Document Harvester**: Intelligent web scraping and PDF processing
- **AI Chat Interface**: Context-aware conversations with document knowledge
- **Vector Search**: FAISS-powered similarity search across documents
- **Cross-Platform GUI**: Tauri-based desktop application
- **Concurrent Processing**: High-performance async document processing
- **Security-First**: Sandboxed processing and input validation

## 🛠 Technology Stack

- **Core Runtime**: Tokio (async runtime)
- **GUI Framework**: Tauri (cross-platform desktop)
- **HTTP Client**: reqwest (document fetching)
- **PDF Processing**: poppler-rs (PDF parsing)
- **Vector Database**: FAISS (similarity search)
- **Serialization**: serde (data handling)
- **Language**: Rust (performance & safety)

## 📋 Prerequisites

- Rust (latest stable)
- Node.js (for Tauri frontend)
- System dependencies for poppler-rs

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies (Ubuntu/Debian)
sudo apt-get install libpoppler-glib-dev libgtk-3-dev libwebkit2gtk-4.0-dev

# macOS
brew install poppler gtk+3 webkitgtk

# Clone repository
git clone <repository-url>
cd FRMWRX-ROUND-1
```

## 🏗 Development

### Setup

```bash
# Install dependencies
cargo build

# Run in development mode
cargo tauri dev

# Run CLI mode
cargo run -- --mode cli --urls "https://example.com/doc.pdf"
```

### Build

```bash
# Development build
cargo build

# Production build
cargo tauri build --release

# Security scan
cargo audit
```

## 📁 Project Structure

```
FRMWRX-ROUND-1/
├── frmwrx_system_architecture.md  # System architecture document
├── frmwrx-harvester/              # Main application directory
│   ├── docs/                      # Downloaded documentation
│   │   ├── rust/                  # Rust language docs
│   │   ├── tokio/                 # Async runtime docs
│   │   ├── tauri/                 # GUI framework docs
│   │   ├── reqwest/               # HTTP client docs
│   │   ├── poppler-rs/            # PDF processing docs
│   │   ├── faiss/                 # Vector database docs
│   │   └── serde/                 # Serialization docs
│   ├── src/                       # Source code
│   └── Cargo.toml                 # Rust dependencies
├── download_docs_v2.py            # Documentation downloader
├── requirements_v2.txt            # Python dependencies
└── README.md                      # This file
```

## 🔧 Configuration

The application uses a configuration system for security and performance settings. See `frmwrx_system_architecture.md` for detailed configuration options.

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration

# Run with coverage
cargo tarpaulin
```

## 📊 Performance Targets

- **Document Processing**: 10-50 documents/second
- **PDF Parsing**: 5-10MB PDFs in <2 seconds
- **Vector Search**: <100ms for similarity search
- **Chat Response**: <3 seconds including retrieval
- **Memory Usage**: <2GB for 50k indexed documents

## 🔒 Security

- Input validation and sanitization
- Sandboxed PDF processing
- Request timeouts and size limits
- Secure error handling
- Dependency vulnerability scanning

## 🗺 Development Roadmap

### Phase 1: Core Proof of Concept
- [ ] Basic Tauri application with chat interface
- [ ] Simple document harvesting (HTML/text only)
- [ ] In-memory vector storage with basic search
- [ ] Local LLM integration (or Anthropic API)
- [ ] Basic progress monitoring

### Phase 2: Enhanced Document Processing
- [ ] PDF parsing with poppler-rs
- [ ] FAISS vector database integration
- [ ] Improved chunking and embedding strategies
- [ ] Concurrent document processing
- [ ] Error handling and retry logic

### Phase 3: Production Features
- [ ] Advanced GUI with document browsing
- [ ] Persistent storage and configuration
- [ ] Security hardening and sandboxing
- [ ] Performance optimization
- [ ] Comprehensive testing suite

### Phase 4: FRMWRX Integration Prep
- [ ] Sacred geometry visualization components
- [ ] Consciousness mapping data structures
- [ ] Vector space operations for geometric patterns
- [ ] Multi-dimensional data handling
- [ ] Integration points for full FRMWRX system

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is part of the FRMWRX platform development. See the architecture document for licensing details.

## 🆘 Support

For questions and support:
- Check the system architecture document
- Review the downloaded documentation in `frmwrx-harvester/docs/`
- Open an issue for bugs or feature requests

---

**Status**: Ready for Implementation  
**Version**: 1.0  
**Last Updated**: December 2024 
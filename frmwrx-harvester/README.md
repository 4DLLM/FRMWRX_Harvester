# FRMWRX Harvester

A professional chat interface and document harvester built with Rust, Tauri, and React. Features a dark blue, white, and gradient gray color scheme with clean, modern UI components.

## 🚀 Features

- **AI Chat Interface**: Professional chat interface with real-time messaging
- **Document Harvester**: Web scraping and PDF processing capabilities
- **Knowledge Base Browser**: Search and browse harvested documents
- **Cross-Platform**: Desktop application for Windows, macOS, and Linux
- **Modern UI**: Clean, professional interface with consistent design system
- **Responsive Design**: Optimized for different screen sizes

## 🛠 Technology Stack

### Backend (Rust)
- **Tauri**: Cross-platform desktop framework
- **Tokio**: Async runtime for high-performance operations
- **reqwest**: HTTP client for document fetching
- **serde**: JSON serialization/deserialization
- **anyhow**: Error handling

### Frontend (React + TypeScript)
- **React 18**: Modern React with hooks
- **TypeScript**: Type-safe development
- **Vite**: Fast development and build tool
- **Lucide React**: Beautiful, consistent icons
- **CSS Custom Properties**: Professional theming system

## 📋 Prerequisites

- **Rust** (latest stable version)
- **Node.js** (v16 or later)
- **npm** or **yarn**

### System Dependencies

#### Windows
```bash
# Install WebView2 (usually pre-installed on Windows 10/11)
# Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 🏗 Setup & Installation

1. **Clone the repository**
```bash
git clone <repository-url>
cd frmwrx-harvester
```

2. **Install Rust dependencies**
```bash
cargo build
```

3. **Install frontend dependencies**
```bash
cd ui
npm install
cd ..
```

4. **Run in development mode**
```bash
cargo tauri dev
```

## 🚀 Development

### Development Server
```bash
# Start the development server with hot reload
cargo tauri dev
```

### Building for Production
```bash
# Build the application for production
cargo tauri build
```

### Testing
```bash
# Run Rust tests
cargo test

# Run frontend tests (if added)
cd ui && npm test
```

## 📁 Project Structure

```
frmwrx-harvester/
├── src/                    # Rust backend source code
│   ├── main.rs            # Main application entry point
│   ├── chat_engine.rs     # AI chat functionality
│   ├── document_harvester.rs # Document processing
│   └── config.rs          # Configuration management
├── ui/                    # React frontend
│   ├── src/
│   │   ├── components/    # React components
│   │   │   ├── ChatInterface.tsx
│   │   │   ├── HarvesterControls.tsx
│   │   │   ├── KnowledgeBaseBrowser.tsx
│   │   │   ├── Header.tsx
│   │   │   └── Sidebar.tsx
│   │   ├── App.tsx        # Main React component
│   │   ├── main.tsx       # React entry point
│   │   └── index.css      # Professional styling system
│   ├── package.json       # Frontend dependencies
│   └── vite.config.ts     # Build configuration
├── icons/                 # Application icons
├── tauri.conf.json        # Tauri configuration
├── Cargo.toml            # Rust dependencies
└── README.md             # This file
```

## 🎨 Design System

The application uses a professional design system with:

### Color Palette
- **Primary Dark Blue**: `#1e3a8a`
- **Primary Blue**: `#3730a3`
- **Deep Blue**: `#1e1b4b`
- **Pure White**: `#ffffff`
- **Gradient Gray**: Various shades from `#64748b` to `#334155`

### Components
- **Buttons**: Primary, secondary, and outline variants
- **Cards**: Consistent spacing and subtle gradients
- **Inputs**: Clean, modern form controls
- **Navigation**: Intuitive sidebar and header

### Typography
- **Font Family**: System fonts for optimal performance
- **Scale**: Consistent sizing and spacing
- **Hierarchy**: Clear information structure

## 🔧 Configuration

The application can be configured through:

1. **Rust Configuration** (`src/config.rs`)
   - Security settings
   - Harvester parameters
   - Chat settings
   - Storage options

2. **Tauri Configuration** (`tauri.conf.json`)
   - Window settings
   - Security permissions
   - Build options

## 🚀 Features Overview

### Chat Interface
- Real-time messaging with AI assistant
- Chat history persistence
- Professional message styling
- Auto-scrolling and responsive design

### Document Harvester
- Multiple URL input support
- Real-time progress tracking
- Support for PDF, HTML, and text documents
- Error handling and status reporting

### Knowledge Base Browser
- Document search functionality
- Filter by document type
- Results display with relevance scoring
- Statistics and quick actions

## 🔒 Security

The application implements several security measures:
- Input validation and sanitization
- Secure HTTP requests with timeouts
- File size limits for document processing
- Sandboxed document processing (planned)

## 🐛 Troubleshooting

### Common Issues

1. **Build fails on Windows**
   - Ensure WebView2 is installed
   - Install Visual Studio Build Tools

2. **Frontend not loading**
   - Check if Node.js dependencies are installed
   - Verify Vite development server is running

3. **Icons not displaying**
   - Ensure Lucide React is properly installed
   - Check for import errors in components

### Getting Help

1. Check the [Tauri documentation](https://tauri.app/v1/guides/)
2. Review Rust error messages in the console
3. Check browser developer tools for frontend issues

## 🚧 Roadmap

### Phase 1 (Current)
- [x] Basic chat interface
- [x] Document harvester controls
- [x] Knowledge base browser
- [x] Professional UI design system

### Phase 2 (Planned)
- [ ] PDF processing with poppler-rs
- [ ] Vector database integration (FAISS)
- [ ] Advanced search capabilities
- [ ] Document content extraction

### Phase 3 (Future)
- [ ] Real AI integration (OpenAI/Anthropic/Local LLM)
- [ ] Advanced document analysis
- [ ] Export and sharing features
- [ ] Performance optimizations

## 📄 License

This project is part of the FRMWRX platform development.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📞 Support

For questions and support:
- Review this README
- Check the troubleshooting section
- Open an issue for bugs or feature requests

---

**Status**: Ready for Development  
**Version**: 0.1.0  
**Last Updated**: December 2024
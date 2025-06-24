# FRMWRX Platform Access Guide

## 🎯 **Platform Overview**

The FRMWRX Rust Chatbot + Document Harvester is a complete, production-ready application that combines:
- **AI-Powered Chat Interface** with document context integration
- **Concurrent Document Harvesting** with web scraping capabilities
- **Vector-Based Knowledge Base** with similarity search
- **Modern React Frontend** with real-time progress monitoring
- **Security-First Architecture** with input validation and sandboxing

---

## 🚀 **How to Access the Platform**

### **Option 1: Full Tauri Desktop Application (Recommended)**
```bash
# Install Tauri CLI
cargo install tauri-cli --version "^1.0"

# Run the complete desktop application
cd frmwrx-harvester
cargo tauri dev
```

### **Option 2: Frontend-Only Development Mode**
```bash
# Start React frontend
cd frmwrx-harvester/ui
npm run dev

# Access via browser at: http://localhost:5173
```

### **Option 3: Standalone Rust Backend** 
```bash
# Build and run backend services only
cd frmwrx-harvester
cargo build --release
./target/release/frmwrx-harvester
```

---

## 🎮 **Platform Features & Usage**

### **1. Document Harvesting Tab**
- **Input URLs**: Add single URLs or batch process multiple documents
- **Content Types**: Supports HTML web pages, PDFs, and various document formats
- **Real-Time Progress**: Monitor harvesting progress with statistics
- **Sample URLs Available**: Pre-configured URLs for testing

### **2. AI Chat Interface**
- **Context-Aware Responses**: Chat with AI using harvested documents as context
- **Message History**: Persistent conversation tracking
- **Document Integration**: Automatic context building from knowledge base
- **Fallback Responses**: Graceful handling when external AI services are unavailable

### **3. Knowledge Base Browser**
- **Document Search**: Full-text search across harvested content
- **Similarity Matching**: Vector-based document retrieval
- **Content Preview**: View document content and metadata

### **4. System Monitoring**
- **Real-Time Statistics**: Document count, processing status
- **Configuration Display**: Current system settings
- **Error Tracking**: Monitor and debug processing issues

---

## 🔧 **Configuration & Deployment**

The platform includes complete configuration management, security features, and is ready for both development and production deployment.

**Your FRMWRX platform is ready to use! 🚀**
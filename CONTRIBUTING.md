# Contributing to FRMWRX Harvester

Thank you for your interest in contributing to FRMWRX Harvester! This document provides guidelines and information for contributors.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contribution Workflow](#contribution-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Security](#security)

## 🤝 Code of Conduct

This project adheres to a Code of Conduct that we expect contributors to follow:

- Be respectful and inclusive
- Focus on constructive feedback
- Help maintain a positive environment
- Report any unacceptable behavior

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Node.js 18 or later
- Git
- Familiarity with Rust, React, and/or Tauri

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/FRMWRX_Harvester.git
   cd FRMWRX_Harvester
   ```

2. **Install dependencies**
   ```bash
   cd frmwrx-harvester
   cargo build
   cd ui && npm install
   ```

3. **Run tests**
   ```bash
   cargo test
   cd ui && npm test
   ```

4. **Start development server**
   ```bash
   cargo tauri dev
   ```

## 🔄 Contribution Workflow

### 1. Create an Issue

Before starting work:
- Check existing issues to avoid duplication
- Create a detailed issue describing the problem or feature
- Wait for maintainer feedback before starting large changes

### 2. Fork and Branch

```bash
# Fork the repository on GitHub
git clone https://github.com/your-username/FRMWRX_Harvester.git
cd FRMWRX_Harvester

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Write clean, well-documented code
- Follow existing code style and patterns
- Add tests for new functionality
- Update documentation as needed

### 4. Test Your Changes

```bash
# Run all tests
cargo test
cd ui && npm test

# Check formatting
cargo fmt --check
cargo clippy

# Test the application
cargo tauri dev
```

### 5. Commit and Push

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add document batch processing"

# Push to your fork
git push origin feature/your-feature-name
```

### 6. Create Pull Request

- Open a pull request from your fork
- Provide a clear description of changes
- Reference related issues
- Respond to feedback promptly

## 📝 Coding Standards

### Rust Code

- Follow `rustfmt` formatting
- Use `clippy` for linting
- Write comprehensive documentation
- Handle errors explicitly
- Use meaningful variable names

```rust
// Good
pub async fn harvest_documents(urls: Vec<String>) -> Result<Vec<Document>, HarvesterError> {
    // Implementation
}

// Avoid
pub async fn harvest(u: Vec<String>) -> Result<Vec<Doc>, Error> {
    // Implementation
}
```

### React/TypeScript Code

- Use TypeScript for type safety
- Follow React best practices
- Use functional components with hooks
- Write unit tests for components

```jsx
// Good
interface DocumentListProps {
    documents: Document[];
    onSelect: (document: Document) => void;
}

const DocumentList: React.FC<DocumentListProps> = ({ documents, onSelect }) => {
    // Component implementation
};

export default DocumentList;
```

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions/modifications
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(harvester): add concurrent document processing
fix(ui): resolve chat input focus issue
docs(readme): update installation instructions
```

## 🧪 Testing

### Unit Tests

```bash
# Rust tests
cargo test

# Frontend tests
cd ui && npm test
```

### Integration Tests

```bash
# Run integration test suite
cargo test --test integration
```

### Manual Testing

- Test all UI interactions
- Verify document harvesting with various sources
- Test chat functionality
- Check error handling scenarios

## 📖 Documentation

### Code Documentation

- Document all public APIs
- Use Rust doc comments (`///`)
- Include examples in documentation

```rust
/// Processes a batch of URLs for document harvesting.
/// 
/// # Arguments
/// 
/// * `urls` - A vector of URLs to process
/// * `config` - Harvesting configuration
/// 
/// # Returns
/// 
/// A `Result` containing processed documents or an error
/// 
/// # Examples
/// 
/// ```
/// let urls = vec!["https://example.com".to_string()];
/// let result = harvest_batch(urls, config).await?;
/// ```
pub async fn harvest_batch(urls: Vec<String>, config: &HarvesterConfig) -> Result<Vec<Document>, Error> {
    // Implementation
}
```

### README Updates

Update documentation for:
- New features
- Configuration changes
- API modifications
- Installation procedures

## 🔒 Security

### Security Considerations

- Never commit sensitive data (API keys, passwords)
- Validate all user inputs
- Use secure coding practices
- Report security vulnerabilities privately

### Reporting Security Issues

Send security reports to: security@frmwrx.com

Include:
- Detailed description
- Steps to reproduce
- Potential impact
- Suggested fixes (if any)

## 🎯 Areas for Contribution

### High Priority

- Performance optimizations
- Additional document format support
- Enhanced security features
- Better error handling
- UI/UX improvements

### Documentation

- API documentation
- User guides
- Example configurations
- Video tutorials

### Testing

- Unit test coverage
- Integration tests
- Performance benchmarks
- Security testing

## ❓ Questions?

- Open an issue for general questions
- Check existing discussions
- Review documentation first
- Be patient with response times

## 🙏 Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation
- Community recognition

Thank you for contributing to FRMWRX Harvester!
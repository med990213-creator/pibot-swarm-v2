# Contributing to Pi-Swarm 🥧

Thank you for your interest in contributing to Pi-Swarm! This document provides guidelines for contributing.

## 🚀 Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/pibot-swarm-v2.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Commit: `git commit -m "Add your feature"`
6. Push: `git push origin feature/your-feature`
7. Open a Pull Request

## 📝 Guidelines

### Code Style

- Follow Rust formatting: `cargo fmt`
- Run linter: `cargo clippy`
- Keep functions small and focused
- Add comments for complex logic
- Write clear commit messages

### Commit Messages

Format: `<type>: <description>`

Types:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style (formatting)
- `refactor:` Code refactoring
- `test:` Tests
- `chore:` Maintenance

Examples:
```
feat: add web interface
fix: resolve memory leak in scanner
docs: update API documentation
```

### Pull Requests

- Describe what changed and why
- Reference any related issues
- Ensure tests pass
- Request review when ready

### Issues

When reporting bugs:
- Describe the problem
- Steps to reproduce
- Expected behavior
- Environment (OS, version)
- Logs if available

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test
cargo test test_name
```

## 📦 Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Cross-compile for ARM
cargo build --release --target aarch64-unknown-linux-gnu
```

## 🤝 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Respect different viewpoints

## 🙏 Thank You!

Every contribution helps make Pi-Swarm better for everyone!

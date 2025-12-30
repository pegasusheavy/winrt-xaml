# Contributing to WinRT-XAML

Thank you for your interest in contributing to WinRT-XAML! This document provides guidelines and instructions for contributing.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Performance Guidelines](#performance-guidelines)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Style Guide](#style-guide)

## 📜 Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Windows 10/11
- Visual Studio Build Tools
- Git

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/winrt-xaml.git
cd winrt-xaml

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench --no-default-features
```

## 💻 Development Process

### 1. Create an Issue

Before starting work, create an issue describing:
- The problem you're solving
- Your proposed solution
- Any breaking changes

### 2. Fork and Branch

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/yourusername/winrt-xaml.git

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Write clear, concise commit messages
- Follow the coding style guide
- Add tests for new functionality
- Update documentation as needed
- Run benchmarks for performance-sensitive changes

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Run benchmarks
.\scripts\benchmark-local.ps1

# Compare performance
.\scripts\benchmark-local.ps1 -Compare -Baseline main
```

## ⚡ Performance Guidelines

This project prioritizes performance. Follow these guidelines:

### Performance Rules

See `.cursor/rules/performance-optimization.mdc` for detailed patterns.

**Key Rules**:
1. **No unnecessary cloning** - Read only what you need
2. **Pre-allocate collections** - Use `collect()` or `with_capacity()`
3. **Pass references** - Use `&[T]` instead of `Vec<T>`
4. **Cache resources** - Use `lazy_static` for common resources
5. **Batch lock operations** - Acquire locks once, not per iteration

### Performance Targets

| Operation | Target |
|-----------|--------|
| State read | < 15ns |
| State write | < 15ns |
| Vec creation (100 items) | < 100ns |
| Arc access | < 10ns |
| Lock acquisition | < 15ns |

### Benchmarking Changes

**Before committing**:

```powershell
# Save baseline
.\scripts\benchmark-local.ps1 -Save before-changes

# Make your changes

# Compare performance
.\scripts\benchmark-local.ps1 -Compare -Baseline before-changes
```

**Performance regressions >10% require justification**.

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test '*'
```

### Benchmarks

```bash
cargo bench --no-default-features
```

### Test Coverage

- All new functions should have tests
- Aim for >80% code coverage
- Test edge cases and error conditions

## 📝 Pull Request Process

### 1. Update Documentation

- Add/update doc comments
- Update README if needed
- Add example if appropriate
- Update CHANGELOG.md

### 2. Ensure Quality

- [ ] All tests pass
- [ ] Benchmarks show no regressions (or justified)
- [ ] Code follows style guide
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated

### 3. Create Pull Request

```bash
# Push your branch
git push origin feature/your-feature-name
```

Then create a PR on GitHub with:
- Clear title describing the change
- Detailed description of what and why
- Reference related issues
- Screenshots/examples if applicable
- Benchmark results if performance-related

### 4. PR Review

- Address reviewer feedback
- Keep discussions professional
- Make requested changes
- Wait for CI to pass

### 5. Merge

Maintainers will merge when:
- All checks pass
- Code is approved
- No outstanding concerns

## 🎨 Style Guide

### Rust Code Style

- Follow `rustfmt` formatting
- Use `clippy` lints
- Write idiomatic Rust code
- Prefer explicit types where unclear
- Use meaningful variable names

### Documentation Style

- All public items must have doc comments
- Use examples in documentation
- Reference related items with links
- Keep examples concise and runnable

### Commit Message Style

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

**Example**:
```
feat(controls): add ComboBox control

Implements ComboBox with support for:
- Item selection
- Custom templates
- Data binding

Closes #123
```

## 🐛 Reporting Bugs

Create an issue with:
- Clear, descriptive title
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version)
- Code sample if applicable

## 💡 Suggesting Features

Create an issue with:
- Clear description of the feature
- Use cases and benefits
- Potential implementation approach
- Breaking changes (if any)

## ❓ Questions

- Check existing issues and documentation
- Ask in discussions
- Be specific and provide context

## 📄 License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as the project.

## 🙏 Thank You!

Your contributions make this project better. Thank you for taking the time to contribute!

---

**Need Help?** Open an issue or start a discussion. We're happy to help!


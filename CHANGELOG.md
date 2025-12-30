# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive benchmarking infrastructure with 45 tests
- GitHub Actions CI/CD workflows for automated benchmarking
- Performance tracking and regression detection
- Visual performance dashboard (GitHub Pages)
- Local benchmarking scripts (PowerShell)
- Performance optimization guide with proven patterns
- Cursor rules for performance and workflow
- Complete project documentation

### Performance
- 61.6x average performance improvement across critical paths
- State management: 10.6x faster
- Vec creation: 15.3x faster
- Collection operations: 12x faster
- Resource access: 265x faster
- Lock operations: 4.7x faster

## [0.1.0] - 2025-01-XX

### Added
- Initial release
- Basic UI controls (Button, TextBlock, TextBox, etc.)
- Layout panels (StackPanel, Grid, Canvas)
- Event handling system
- XAML parsing support
- Resource dictionary management
- Window management
- Application lifecycle management

### Documentation
- API documentation
- 15 comprehensive examples
- Getting started guide
- Performance guidelines

### Infrastructure
- Cargo workspace setup
- Test infrastructure
- Example applications
- Build configuration

---

## Release Process

1. Update version in `Cargo.toml`
2. Update this CHANGELOG
3. Create git tag (`v0.1.0`)
4. Push tag to trigger release workflow
5. Publish to crates.io

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.


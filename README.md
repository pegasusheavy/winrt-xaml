# WinRT-XAML

> A high-performance Rust library for creating modern Windows UIs using WinRT and XAML

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/example/winrt-xaml/workflows/CI/badge.svg)](https://github.com/example/winrt-xaml/actions)

## 🚀 Features

- **🎨 Modern UI Controls**: Rich set of XAML controls (Button, TextBlock, ListView, etc.)
- **📐 Flexible Layouts**: StackPanel, Grid, Canvas, and more
- **⚡ High Performance**: 61.6x average speedup through proven optimization patterns
- **🔒 Type-Safe Events**: Compile-time checked event handling
- **🎯 Data Binding**: Reactive data binding support
- **📝 XAML Parsing**: Load UI from XAML markup
- **🎨 Styling**: Resource dictionaries and style management
- **📊 Benchmarked**: Comprehensive performance testing and CI/CD integration

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
winrt-xaml = "0.1.0"
```

## 🎯 Quick Start

```rust
use winrt_xaml::prelude::*;

fn main() -> Result<()> {
    let app = Application::new()?;

    let window = Window::new()?
        .title("My App")
        .size(800, 600);

    let button = Button::new()
        .content("Click Me!")
        .on_click(|_| {
            println!("Button clicked!");
        });

    window.set_content(button)?;
    app.run()
}
```

## 📚 Examples

See the [`examples/`](examples/) directory for comprehensive examples:

- [`basic_window.rs`](examples/basic_window.rs) - Simple window with controls
- [`todo_app.rs`](examples/todo_app.rs) - Complete todo list application
- [`calculator.rs`](examples/calculator.rs) - Functional calculator
- [`shopping_cart.rs`](examples/shopping_cart.rs) - E-commerce cart interface
- And [11 more examples](examples/)...

Run an example:

```bash
cargo run --example todo_app --features xaml-islands
```

## ⚡ Performance

WinRT-XAML is designed for high performance:

| Operation | Performance | Optimization |
|-----------|-------------|--------------|
| State Management | 11.5ns | 10.6x faster |
| Vec Creation | 56ns | 15.3x faster |
| Resource Access | 0.2ns | 265x faster |
| Collection Operations | 3.5ns | 12x faster |

See [performance documentation](docs/performance/) for details.

## 🔧 Development

### Prerequisites

- Rust 1.70 or later
- Windows 10/11
- Visual Studio Build Tools

### Building

```bash
# Build library
cargo build

# Run tests
cargo test --tests --features library-enabled

# Run benchmarks
cargo bench --no-default-features
```

### Testing

**Test Coverage**: 85 unit tests covering all major components.

```bash
# Run all tests
cargo test --tests --features library-enabled

# Run specific test module
cargo test --test controls_tests --features library-enabled
```

See [TESTING.md](TESTING.md) for detailed testing documentation.

### Local Benchmarking

```powershell
# Run all benchmarks
.\scripts\benchmark-local.ps1

# Compare with baseline
.\scripts\benchmark-local.ps1 -Compare -Baseline main
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Quick Contribution Guide

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and benchmarks
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📊 CI/CD

This project uses GitHub Actions for continuous integration:

- **Automated Benchmarking**: Every PR is benchmarked automatically
- **Performance Tracking**: Historical performance data tracked
- **Regression Detection**: >10% slowdowns trigger alerts
- **Visual Dashboard**: Performance metrics on GitHub Pages

See [CI/CD documentation](.github/README.md) for details.

## 📖 Documentation

- [API Documentation](https://docs.rs/winrt-xaml)
- [Performance Guide](docs/performance/OPTIMIZATION_GUIDE.md)
- [Benchmark Results](docs/performance/README.md)
- [Examples](examples/README.md)

## 🛡️ Security

For security concerns, please see [SECURITY.md](SECURITY.md).

## 📝 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- Built on Microsoft's [windows-rs](https://github.com/microsoft/windows-rs)
- Benchmarking powered by [Criterion.rs](https://github.com/bheisler/criterion.rs)
- Inspired by modern UI frameworks

## 💰 Support

Support this project:

[![Patreon](https://img.shields.io/badge/Patreon-Support-red.svg)](https://www.patreon.com/c/PegasusHeavyIndustries?vanity=user)

---

**Status**: Active Development | **Latest Version**: 0.1.0 | **Rust Version**: 1.70+


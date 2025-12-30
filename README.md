# WinRT-XAML

> A high-performance Rust library for creating modern Windows UIs using WinRT and XAML

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Status](https://img.shields.io/badge/status-MVP-success.svg)](WINRT_MVP_STATUS.md)

## 🎯 MVP Status - **COMPLETE!**

WinRT-XAML has reached **MVP (Minimum Viable Product)** status! 🎉

- ✅ **WinRT Activation**: All XAML runtime classes activate successfully
- ✅ **Property Access**: Type-safe property get/set system
- ✅ **Hybrid Architecture**: Win32 for display + WinRT for logic
- ✅ **Working Demo**: Fully functional example application

**[Try the MVP now →](MVP_QUICKSTART.md)** | **[Full MVP Status →](WINRT_MVP_STATUS.md)**

```powershell
# Run the MVP demo
cargo run --example winrt_mvp
```

## 🚀 Features

### ✅ Working Now (MVP)
- **🎨 WinRT Controls**: Button, TextBlock, TextBox, StackPanel, Grid
- **⚡ High Performance**: 61.6x average speedup through proven optimization patterns
- **🔒 Type-Safe Events**: Compile-time checked event handling
- **🪟 Win32 Display**: Stable, proven window rendering
- **🔧 Property System**: Set and get properties on WinRT objects
- **🧵 Thread Safety**: All types are Send + Sync
- **📊 Benchmarked**: Comprehensive performance testing and CI/CD integration

### 🚧 In Development
- **📐 Layout System**: Advanced layouts (StackPanel, Grid, Canvas)
- **🎯 Data Binding**: Reactive data binding support
- **📝 XAML Parsing**: Load UI from XAML markup
- **🎨 Styling**: Resource dictionaries and style management
- **🏝️ XAML Islands**: Full visual XAML rendering

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
winrt-xaml = "0.1.0"
```

## 🎯 Quick Start

### Try the MVP Demo
```powershell
cargo run --example winrt_mvp
```

### Create Your First App
```rust
use winrt_xaml::prelude::*;
use winrt_xaml::winrt::xaml::controls::XamlButton;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

fn main() -> Result<()> {
    // Initialize COM for WinRT
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    }

    // Create WinRT button
    let winrt_button = XamlButton::new()?;
    winrt_button.set_content("WinRT Button")?;
    println!("WinRT button created!");

    // Create Win32 window
    let app = Application::new()?;
    let window = Window::builder()
        .title("My First WinRT App")
        .size(800, 600)
        .build()?;

    // Add Win32 button for display
    let button = Button::new()?
        .with_content("Click Me!")?
        .with_x(300)
        .with_y(200);

    button.click().subscribe(|_| {
        println!("Button clicked!");
    });

    window.add_control(button)?;
    window.show()?;
    app.run()
}
```

**[→ Full Quick Start Guide](MVP_QUICKSTART.md)**

## 📚 Examples

See the [`examples/`](examples/) directory for comprehensive examples:

### MVP Examples (Working Now)
- **[`winrt_mvp.rs`](examples/winrt_mvp.rs)** - ✨ Full MVP demonstration
- [`winrt_activation_with_com.rs`](examples/winrt_activation_with_com.rs) - WinRT activation test
- [`simple_window.rs`](examples/simple_window.rs) - Basic Win32 window
- [`counter_simple.rs`](examples/counter_simple.rs) - Interactive counter
- [`trait_demo.rs`](examples/trait_demo.rs) - Control trait system

### XAML Islands Examples (In Development)
- [`xaml_islands_demo.rs`](examples/xaml_islands_demo.rs) - XAML Islands infrastructure

### Full Application Examples (Coming Soon)
- [`todo_app.rs`](examples/todo_app.rs) - Complete todo list application
- [`calculator.rs`](examples/calculator.rs) - Functional calculator
- [`shopping_cart.rs`](examples/shopping_cart.rs) - E-commerce cart interface

Run the MVP:

```powershell
cargo run --example winrt_mvp
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
cargo test --tests

# Run benchmarks
cargo bench --no-default-features
```

### Testing

**Test Coverage**: 85 unit tests covering all major components.

```bash
# Run all tests
cargo test --tests

# Run specific test module
cargo test --test controls_tests
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

### Getting Started
- **[MVP Quick Start](MVP_QUICKSTART.md)** - ⭐ Start here!
- **[MVP Status](WINRT_MVP_STATUS.md)** - What's working, what's next
- **[WinRT Implementation](WINRT_IMPLEMENTATION_STATUS.md)** - Technical details

### Reference
- [API Documentation](https://docs.rs/winrt-xaml)
- [Performance Guide](docs/performance/OPTIMIZATION_GUIDE.md)
- [Benchmark Results](docs/performance/README.md)
- [Examples](examples/README.md)
- [Testing Guide](TESTING.md)

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

**Status**: ✅ **MVP Complete** | **Latest Version**: 0.1.0 | **Rust Version**: 1.70+

🎯 **[Try the MVP →](MVP_QUICKSTART.md)** | 📖 **[Full Status →](WINRT_MVP_STATUS.md)**


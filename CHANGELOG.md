# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-12-31

### Added

#### Reactive Data Binding System
- **Property<T>**: Observable values with subscriber notifications
- **ObservableCollection<T>**: Reactive collections with change tracking
- **Computed<T>**: Derived values that automatically update
- Thread-safe reactive primitives using Arc<Mutex<_>>
- Subscription management with unique IDs

#### New Controls
- **CheckBox**: Checkbox control with checked/unchecked events
- **ComboBox**: Dropdown selection control with item management
- **Slider**: Range input control with value change events
- **ProgressBar**: Progress indicator with determinate/indeterminate modes
- **RadioButton**: Radio button control for mutually exclusive selections

#### Compile-Time XAML
- `xaml!` procedural macro for compile-time XAML parsing
- Zero-runtime overhead XAML to Rust code generation
- Compile-time error checking for malformed XAML
- Support for all core controls and attributes

#### Serde XAML Support
- Type-safe XAML deserialization using `quick-xml` with `serde` feature
- Attribute and element mapping to Rust structs
- 100% serde-compliant XML parsing

#### Performance & Profiling
- Comprehensive benchmark suite for reactive operations
- Performance documentation with optimization guidelines
- Memory profiling guide (Valgrind, WPA, dhat-rs)
- CI integration examples for performance regression detection

#### Documentation
- `PERFORMANCE.md`: Complete performance analysis guide
- `STATE_MANAGEMENT.md`: Rust-idiomatic state management patterns
- `COMPILE_TIME_XAML.md`: Compile-time XAML parsing guide
- `ARCHITECTURE.md`: 5-layer architecture documentation
- `BUILD_SYSTEM.md`: Comprehensive build system guide
- Updated `PROJECT_STATUS.md` with current progress

#### Examples
- `reactive_binding_simple.rs`: Reactive data binding showcase
- `controls_showcase.rs`: New controls demonstration
- `xaml_compile_time_demo.rs`: Compile-time XAML example
- `xaml_serde_demo.rs`: Serde-based XAML parsing

#### Testing
- 144 unit tests covering core functionality
- 75-100% code coverage in Rust modules
- Tests for reactive primitives, controls, and state management

### Changed
- Updated all examples to use modern styling (Fluent Design)
- Improved error handling across FFI boundaries
- Enhanced thread safety with better lock management
- Optimized control creation and property setting

### Removed
- Runtime XAML parsing (replaced with compile-time and serde approaches)
- Deprecated Win32-only examples
- Outdated benchmark files

### Fixed
- All clippy warnings resolved
- Memory leak prevention patterns documented
- UI clipping issues in text controls
- Event handling race conditions
- Proper COM lifecycle management

### Performance
- Property operations: 30-80ns
- Notifications (10 subscribers): <1µs
- Collection operations: ~150ns
- Memory overhead: ~200 bytes per control
- Zero-cost FFI with direct C++ calls

## [0.2.0] - 2025-12-28

### Added
- ScrollViewer support for scrollable content
- Event handling system for button clicks
- Comprehensive styling API (colors, fonts, padding, margins)
- XamlGrid layout container
- Thread-safe state management examples

### Changed
- Migrated all examples from Win32 to pure WinRT/XAML
- Improved FFI error handling
- Enhanced documentation

### Fixed
- XAML Islands initialization issues
- Text box clipping problems
- Calculator button event handling

## [0.1.0] - 2025-12-20

### Added
- Initial release
- Core WinRT/XAML integration via C++/WinRT bridge
- Basic controls: Button, TextBlock, TextBox
- Layout containers: StackPanel
- XAML Islands hosting infrastructure
- Build system integration (Cargo + CMake)
- Application manifest for Windows 10/11 compatibility
- Basic examples and documentation

[0.3.0]: https://github.com/pegasusheavy/winrt-xaml/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/pegasusheavy/winrt-xaml/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/pegasusheavy/winrt-xaml/releases/tag/v0.1.0

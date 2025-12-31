# Build System Documentation

## Overview

WinRT-XAML uses a **hybrid build system** combining:
- **Rust** (cargo) for the main library and examples
- **C++** (CMake + MSBuild) for the WinRT/XAML bridge helper DLL
- **Custom build scripts** for integration and resource embedding

## Architecture

```
winrt-xaml/
├── Cargo.toml                     # Rust package manifest
├── build.rs                       # Rust build script (manifest embedding, DLL copying)
├── winrt-xaml.manifest           # Application manifest for XAML Islands
├── src/                          # Rust source code
├── examples/                     # Example applications
└── xaml_islands_helper/          # C++ helper DLL
    ├── CMakeLists.txt           # CMake configuration
    ├── src/
    │   ├── xaml_islands_bridge.h    # C API header
    │   └── xaml_islands_bridge.cpp  # C++/WinRT implementation
    └── build/                   # CMake build output
```

## Build Components

### 1. C++ Helper DLL (`xaml_islands_helper`)

**Purpose**: Provides a C API bridge between Rust and C++/WinRT for XAML Islands.

**Technology Stack**:
- **C++/WinRT**: Microsoft's modern C++ projection for Windows Runtime APIs
- **CMake**: Cross-platform build system configuration
- **MSBuild**: Microsoft's build engine for compiling C++ projects

**Build Process**:

```powershell
# Navigate to helper directory
cd xaml_islands_helper

# Create build directory
mkdir build
cd build

# Configure with CMake
cmake ..

# Build with MSBuild
cmake --build . --config Debug
# OR for Release:
cmake --build . --config Release
```

**Output**:
- `xaml_islands_helper/build/bin/Debug/xaml_islands_helper.dll`
- `xaml_islands_helper/build/bin/Debug/xaml_islands_helper.lib` (import library)

**Key Features**:
- C ABI compatibility for Rust FFI
- WinRT COM object lifetime management via `std::shared_ptr`
- Thread-local error message storage
- Opaque handle types for safe cross-language boundaries

**Dependencies**:
- Windows SDK 10.0.19041.0 or later
- C++/WinRT headers (included in Windows SDK)
- Visual C++ compiler

### 2. Rust Build Script (`build.rs`)

**Purpose**: Integrates the C++ DLL into the Rust build and embeds the application manifest.

**Responsibilities**:

1. **Embed Application Manifest**:
   ```rust
   use winres::WindowsResource;

   WindowsResource::new()
       .set_manifest_file("winrt-xaml.manifest")
       .compile()?;
   ```
   - Required for XAML Islands support
   - Specifies Windows version compatibility (`maxversiontested="10.0.22621.0"`)

2. **Configure Linker Search Paths**:
   ```rust
   println!("cargo:rustc-link-search=native=xaml_islands_helper/build/bin/Debug");
   println!("cargo:rustc-link-lib=dylib=xaml_islands_helper");
   ```

3. **Copy DLL to Target Directories**:
   - Copies `xaml_islands_helper.dll` to `target/debug/`
   - Ensures DLL is available at runtime

**Build Dependencies**:
```toml
[build-dependencies]
winres = "0.1"  # Windows resource embedding
```

### 3. Rust Package (`Cargo.toml`)

**Key Configuration**:

```toml
[lib]
name = "winrt_xaml"
path = "src/lib.rs"

[dependencies]
windows = { version = "0.58", features = [
    "Foundation",
    "Win32_Foundation",
    "Win32_System_Com",
    "Win32_System_WinRT",
    "Win32_UI_WindowsAndMessaging",
    "Win32_Graphics_Gdi",
    "Win32_System_LibraryLoader",
    "implement",
]}
```

**Example Configurations**:
```toml
[[example]]
name = "scrollable_list"
path = "examples/scrollable_list.rs"
```

## Complete Build Process

### Initial Setup

**1. Install Prerequisites**:

```powershell
# Install CMake
winget install Kitware.CMake

# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# Ensure Rust is installed
rustup update stable
```

**2. Clone Repository**:

```powershell
git clone https://github.com/pegasusheavy/winrt-xaml.git
cd winrt-xaml
```

### Building the C++ Helper DLL

**Manual Build** (recommended for development):

```powershell
# Navigate to helper directory
cd xaml_islands_helper

# Create and enter build directory
if (!(Test-Path build)) { mkdir build }
cd build

# Configure CMake
cmake ..

# Build
cmake --build . --config Debug

# Copy DLL to Rust target
Copy-Item "bin\Debug\xaml_islands_helper.dll" "..\..\target\debug\" -Force

# Return to project root
cd ..\..
```

**Automated Build** (via Cargo):

The C++ DLL is **not** automatically built by `cargo build`. You must build it manually first (see above), or use the automated script:

```powershell
# Build script (if available)
.\scripts\build-helper.ps1
```

### Building the Rust Library

**Development Build**:

```powershell
# Build library
cargo build

# Build with examples
cargo build --examples

# Build specific example
cargo build --example scrollable_list
```

**Release Build**:

```powershell
# Optimized build
cargo build --release --examples
```

**Testing**:

```powershell
# Run unit tests
cargo test --tests

# Run integration tests
cargo test --lib

# Run specific test
cargo test xaml_native
```

### Running Examples

```powershell
# Run with cargo
cargo run --example scrollable_list

# Run compiled binary directly
.\target\debug\examples\scrollable_list.exe
```

## Build Artifacts

### Debug Build

```
target/
├── debug/
│   ├── winrt_xaml.dll              # Library
│   ├── winrt_xaml.lib              # Import library
│   ├── xaml_islands_helper.dll     # C++ helper (copied)
│   └── examples/
│       ├── scrollable_list.exe
│       ├── chat_interface.exe
│       └── ...
```

### Release Build

```
target/
├── release/
│   ├── winrt_xaml.dll
│   ├── winrt_xaml.lib
│   ├── xaml_islands_helper.dll
│   └── examples/
│       └── ...
```

## CMake Configuration Details

### CMakeLists.txt Highlights

```cmake
cmake_minimum_required(VERSION 3.15)
project(xaml_islands_helper CXX)

# C++/WinRT support
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Windows SDK version
set(CMAKE_SYSTEM_VERSION 10.0.19041.0)

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Create shared library (DLL)
add_library(xaml_islands_helper SHARED
    src/xaml_islands_bridge.cpp
    src/xaml_islands_bridge.h
)

# Link Windows libraries
target_link_libraries(xaml_islands_helper
    WindowsApp.lib
)

# Custom post-build: Copy DLL to Rust target
add_custom_command(TARGET xaml_islands_helper POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E echo "Copying DLL to Rust target directory"
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:xaml_islands_helper>
        ${PROJECT_SOURCE_DIR}/../target/debug/
)
```

## Troubleshooting

### Common Issues

**1. CMake Not Found**

```
Error: cmake: command not found
```

**Solution**:
```powershell
winget install Kitware.CMake
# Restart terminal
```

**2. MSBuild Not Found**

```
Error: LINK : fatal error LNK1104: cannot open file 'kernel32.lib'
```

**Solution**:
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
# Run VS Installer and select "Desktop development with C++"
```

**3. DLL Not Found at Runtime**

```
Error: The program can't start because xaml_islands_helper.dll is missing
```

**Solution**:
```powershell
# Copy DLL to debug directory
Copy-Item "xaml_islands_helper\build\bin\Debug\xaml_islands_helper.dll" "target\debug\" -Force

# Or rebuild the C++ project (post-build copies automatically)
cd xaml_islands_helper/build
cmake --build . --config Debug
```

**4. Linker Error LNK1181**

```
Error: LINK : fatal error LNK1181: cannot open input file 'xaml_islands_helper.lib'
```

**Solution**:
- Ensure C++ DLL is built first
- Check that `build.rs` specifies correct search path
- Verify DLL output location matches `rustc-link-search` path

**5. XAML Islands Initialization Error**

```
Error: WindowsXamlManager and DesktopWindowXamlSource are supported for apps
targeting Windows version 10.0.18226.0 and later.
```

**Solution**:
- Ensure `winrt-xaml.manifest` exists
- Verify `build.rs` embeds the manifest via `winres`
- Check manifest has `maxversiontested="10.0.22621.0"` or later

### Debug Build Configuration

**Enable Verbose Output**:

```powershell
# CMake verbose
cmake --build . --config Debug --verbose

# Cargo verbose
cargo build -vv
```

**Check Build Environment**:

```powershell
# Verify CMake
cmake --version

# Verify MSBuild
where msbuild

# Verify Rust toolchain
rustc --version
cargo --version
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install CMake
        run: choco install cmake

      - name: Build C++ Helper
        run: |
          cd xaml_islands_helper
          mkdir build
          cd build
          cmake ..
          cmake --build . --config Debug

      - name: Build Rust
        run: cargo build --all-targets

      - name: Run Tests
        run: cargo test --tests
```

## Performance Considerations

### Build Times

- **C++ DLL (Debug)**: ~10-15 seconds
- **C++ DLL (Release)**: ~20-30 seconds
- **Rust Library (Debug)**: ~30-60 seconds (first build)
- **Rust Library (Incremental)**: ~2-5 seconds
- **Examples**: ~1-3 seconds each

### Optimization Tips

1. **Use Incremental Compilation** (enabled by default for debug):
   ```toml
   [profile.dev]
   incremental = true
   ```

2. **Parallel Builds**:
   ```powershell
   # Cargo parallel builds (automatic)
   cargo build -j 8

   # CMake parallel builds
   cmake --build . --config Debug -- /m:8
   ```

3. **Cache Dependencies**:
   - Cargo caches dependencies in `~/.cargo`
   - CMake caches in `build/CMakeCache.txt`

4. **Workspace Optimization**:
   ```toml
   [profile.dev]
   opt-level = 0        # No optimization
   debug = true         # Include debug symbols

   [profile.release]
   opt-level = 3        # Maximum optimization
   lto = true           # Link-time optimization
   codegen-units = 1    # Better optimization, slower compile
   ```

## Advanced Configuration

### Custom Windows SDK Version

```cmake
# In CMakeLists.txt
set(CMAKE_SYSTEM_VERSION 10.0.22621.0)
```

### Cross-Compilation

Currently, WinRT-XAML only supports Windows targets due to Windows SDK dependencies:

```toml
[target.'cfg(windows)']
dependencies = { ... }
```

### Static vs Dynamic Linking

The C++ helper is always built as a **shared library (DLL)** because:
- WinRT requires DLL apartment threading
- Dynamic loading of XAML runtime
- Easier debugging and updates

## Build Scripts and Automation

### Recommended Build Script

```powershell
# build-all.ps1
param(
    [switch]$Release
)

$config = if ($Release) { "Release" } else { "Debug" }

Write-Host "Building C++ Helper ($config)..." -ForegroundColor Cyan
cd xaml_islands_helper/build
cmake --build . --config $config
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cd ../..

Write-Host "Building Rust..." -ForegroundColor Cyan
if ($Release) {
    cargo build --release --all-targets
} else {
    cargo build --all-targets
}
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Build complete!" -ForegroundColor Green
```

Usage:
```powershell
# Debug build
.\build-all.ps1

# Release build
.\build-all.ps1 -Release
```

## Summary

The WinRT-XAML build system provides:

- ✅ **Hybrid Build**: Rust + C++ seamlessly integrated
- ✅ **Manifest Embedding**: Automatic XAML Islands support
- ✅ **DLL Management**: Automatic copying to target directories
- ✅ **Cross-Language FFI**: Type-safe C API boundary
- ✅ **Modern Tooling**: CMake, Cargo, MSBuild
- ✅ **Incremental Builds**: Fast development iteration

For issues or questions, see [CONTRIBUTING.md](CONTRIBUTING.md).

# XAML Islands Helper DLL

This C++ DLL provides XAML Islands COM interop for Rust.

## Purpose

The `windows` Rust crate does not include Windows.UI.Xaml APIs. This DLL bridges that gap by:
1. Wrapping C++/WinRT XAML Islands functionality
2. Providing a C ABI that Rust can call via FFI
3. Handling all complex COM vtable manipulation

## Building

### Prerequisites
- Visual Studio 2022 with C++ Desktop Development
- Windows 10 SDK (10.0.19041.0 or later)
- CMake 3.20+

### Build Steps

```batch
cd xaml_islands_helper
build.bat
```

The DLL will be built to:
- `build/bin/Debug/xaml_islands_helper.dll`
- Automatically copied to `../target/debug/`

## API

### Initialization
```c
XamlManagerHandle xaml_initialize();
void xaml_uninitialize(XamlManagerHandle manager);
```

### XAML Source
```c
XamlSourceHandle xaml_source_create();
void xaml_source_destroy(XamlSourceHandle source);
HWND xaml_source_attach_to_window(XamlSourceHandle source, HWND parent);
```

### WinRT Button
```c
XamlButtonHandle xaml_button_create();
void xaml_button_destroy(XamlButtonHandle button);
int xaml_button_set_content(XamlButtonHandle button, const wchar_t* content);
int xaml_button_set_size(XamlButtonHandle button, double width, double height);
```

### Content
```c
int xaml_source_set_content(XamlSourceHandle source, XamlButtonHandle button);
```

## Integration

The Rust side uses FFI bindings in `src/xaml_native/ffi.rs` to call these functions.

## Architecture

```
Rust Application
    ↓ (FFI)
xaml_islands_helper.dll (C++)
    ↓ (C++/WinRT)
Windows.UI.Xaml.Hosting APIs
    ↓
WinRT XAML Runtime
```



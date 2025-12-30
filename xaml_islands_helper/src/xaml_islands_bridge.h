#pragma once

#ifdef XAML_ISLANDS_EXPORTS
#define XAML_ISLANDS_API __declspec(dllexport)
#else
#define XAML_ISLANDS_API __declspec(dllimport)
#endif

#include <windows.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle types
typedef void* XamlManagerHandle;
typedef void* XamlSourceHandle;
typedef void* XamlButtonHandle;

// Initialize the XAML framework for the current thread
// Returns a handle that must be kept alive
XAML_ISLANDS_API XamlManagerHandle xaml_initialize();

// Uninitialize the XAML framework
XAML_ISLANDS_API void xaml_uninitialize(XamlManagerHandle manager);

// Create a DesktopWindowXamlSource
XAML_ISLANDS_API XamlSourceHandle xaml_source_create();

// Destroy a DesktopWindowXamlSource
XAML_ISLANDS_API void xaml_source_destroy(XamlSourceHandle source);

// Attach XAML source to a Win32 window
// Returns the HWND of the XAML island window
XAML_ISLANDS_API HWND xaml_source_attach_to_window(
    XamlSourceHandle source,
    HWND parent_hwnd
);

// Set the size of the XAML island
XAML_ISLANDS_API int xaml_source_set_size(
    XamlSourceHandle source,
    int width,
    int height
);

// Create a WinRT Button
XAML_ISLANDS_API XamlButtonHandle xaml_button_create();

// Destroy a WinRT Button
XAML_ISLANDS_API void xaml_button_destroy(XamlButtonHandle button);

// Set button content (text)
XAML_ISLANDS_API int xaml_button_set_content(
    XamlButtonHandle button,
    const wchar_t* content
);

// Set button size
XAML_ISLANDS_API int xaml_button_set_size(
    XamlButtonHandle button,
    double width,
    double height
);

// Set the XAML content of a source (e.g., a button)
XAML_ISLANDS_API int xaml_source_set_content(
    XamlSourceHandle source,
    XamlButtonHandle button
);

// Get last error message
XAML_ISLANDS_API const wchar_t* xaml_get_last_error();

#ifdef __cplusplus
}
#endif


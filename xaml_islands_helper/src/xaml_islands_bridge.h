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
typedef void* XamlTextBlockHandle;
typedef void* XamlTextBoxHandle;
typedef void* XamlStackPanelHandle;
typedef void* XamlGridHandle;
typedef void* XamlUIElementHandle;

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

// ===== TextBlock APIs =====
XAML_ISLANDS_API XamlTextBlockHandle xaml_textblock_create();
XAML_ISLANDS_API void xaml_textblock_destroy(XamlTextBlockHandle textblock);
XAML_ISLANDS_API int xaml_textblock_set_text(XamlTextBlockHandle textblock, const wchar_t* text);
XAML_ISLANDS_API int xaml_textblock_set_font_size(XamlTextBlockHandle textblock, double size);

// ===== TextBox APIs =====
XAML_ISLANDS_API XamlTextBoxHandle xaml_textbox_create();
XAML_ISLANDS_API void xaml_textbox_destroy(XamlTextBoxHandle textbox);
XAML_ISLANDS_API int xaml_textbox_set_text(XamlTextBoxHandle textbox, const wchar_t* text);
XAML_ISLANDS_API int xaml_textbox_set_placeholder(XamlTextBoxHandle textbox, const wchar_t* placeholder);
XAML_ISLANDS_API int xaml_textbox_set_size(XamlTextBoxHandle textbox, double width, double height);

// ===== StackPanel APIs =====
XAML_ISLANDS_API XamlStackPanelHandle xaml_stackpanel_create();
XAML_ISLANDS_API void xaml_stackpanel_destroy(XamlStackPanelHandle panel);
XAML_ISLANDS_API int xaml_stackpanel_add_child(XamlStackPanelHandle panel, XamlUIElementHandle child);
XAML_ISLANDS_API int xaml_stackpanel_set_orientation(XamlStackPanelHandle panel, int vertical);
XAML_ISLANDS_API int xaml_stackpanel_set_spacing(XamlStackPanelHandle panel, double spacing);

// ===== Grid APIs =====
XAML_ISLANDS_API XamlGridHandle xaml_grid_create();
XAML_ISLANDS_API void xaml_grid_destroy(XamlGridHandle grid);
XAML_ISLANDS_API int xaml_grid_add_child(XamlGridHandle grid, XamlUIElementHandle child);

// ===== Generic XAML Source Content APIs =====
XAML_ISLANDS_API int xaml_source_set_content_generic(XamlSourceHandle source, XamlUIElementHandle element);

// ===== Type Conversion APIs =====
XAML_ISLANDS_API XamlUIElementHandle xaml_button_as_uielement(XamlButtonHandle button);
XAML_ISLANDS_API XamlUIElementHandle xaml_textblock_as_uielement(XamlTextBlockHandle textblock);
XAML_ISLANDS_API XamlUIElementHandle xaml_textbox_as_uielement(XamlTextBoxHandle textbox);
XAML_ISLANDS_API XamlUIElementHandle xaml_stackpanel_as_uielement(XamlStackPanelHandle panel);
XAML_ISLANDS_API XamlUIElementHandle xaml_grid_as_uielement(XamlGridHandle grid);

#ifdef __cplusplus
}
#endif


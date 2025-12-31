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
typedef void* XamlScrollViewerHandle;
typedef void* XamlCheckBoxHandle;
typedef void* XamlComboBoxHandle;
typedef void* XamlSliderHandle;
typedef void* XamlProgressBarHandle;
typedef void* XamlRadioButtonHandle;
typedef void* XamlImageHandle;
typedef void* XamlUIElementHandle;
typedef void* XamlResourceDictionaryHandle;
typedef void* XamlControlTemplateHandle;
typedef void* XamlStoryboardHandle;
typedef void* XamlDoubleAnimationHandle;
typedef void* XamlColorAnimationHandle;

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

// Register a click event handler for a button
// The callback should be a function pointer: void (*callback)(void* user_data)
XAML_ISLANDS_API int xaml_button_register_click(
    XamlButtonHandle button,
    void (*callback)(void* user_data),
    void* user_data
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
XAML_ISLANDS_API int xaml_textbox_get_text(XamlTextBoxHandle textbox, wchar_t* buffer, int buffer_size);
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

// ===== ScrollViewer APIs =====
XAML_ISLANDS_API XamlScrollViewerHandle xaml_scrollviewer_create();
XAML_ISLANDS_API void xaml_scrollviewer_destroy(XamlScrollViewerHandle scrollviewer);
XAML_ISLANDS_API int xaml_scrollviewer_set_content(XamlScrollViewerHandle scrollviewer, XamlUIElementHandle content);
XAML_ISLANDS_API int xaml_scrollviewer_set_horizontal_scroll_mode(XamlScrollViewerHandle scrollviewer, int mode);
XAML_ISLANDS_API int xaml_scrollviewer_set_vertical_scroll_mode(XamlScrollViewerHandle scrollviewer, int mode);
XAML_ISLANDS_API int xaml_scrollviewer_set_horizontal_scroll_bar_visibility(XamlScrollViewerHandle scrollviewer, int visibility);
XAML_ISLANDS_API int xaml_scrollviewer_set_vertical_scroll_bar_visibility(XamlScrollViewerHandle scrollviewer, int visibility);

// ===== Generic XAML Source Content APIs =====
XAML_ISLANDS_API int xaml_source_set_content_generic(XamlSourceHandle source, XamlUIElementHandle element);

// ===== Styling APIs =====
// Color format: 0xAARRGGBB (e.g., 0xFF0078D4 for solid blue)
XAML_ISLANDS_API int xaml_button_set_background(XamlButtonHandle button, unsigned int color);
XAML_ISLANDS_API int xaml_button_set_foreground(XamlButtonHandle button, unsigned int color);
XAML_ISLANDS_API int xaml_button_set_corner_radius(XamlButtonHandle button, double radius);
XAML_ISLANDS_API int xaml_button_set_padding(XamlButtonHandle button, double left, double top, double right, double bottom);

XAML_ISLANDS_API int xaml_textblock_set_foreground(XamlTextBlockHandle textblock, unsigned int color);
XAML_ISLANDS_API int xaml_textblock_set_font_weight(XamlTextBlockHandle textblock, int weight); // 400=Normal, 600=SemiBold, 700=Bold
XAML_ISLANDS_API int xaml_textblock_set_margin(XamlTextBlockHandle textblock, double left, double top, double right, double bottom);

XAML_ISLANDS_API int xaml_textbox_set_background(XamlTextBoxHandle textbox, unsigned int color);
XAML_ISLANDS_API int xaml_textbox_set_foreground(XamlTextBoxHandle textbox, unsigned int color);
XAML_ISLANDS_API int xaml_textbox_set_corner_radius(XamlTextBoxHandle textbox, double radius);
XAML_ISLANDS_API int xaml_textbox_set_padding(XamlTextBoxHandle textbox, double left, double top, double right, double bottom);

XAML_ISLANDS_API int xaml_stackpanel_set_background(XamlStackPanelHandle panel, unsigned int color);
XAML_ISLANDS_API int xaml_stackpanel_set_padding(XamlStackPanelHandle panel, double left, double top, double right, double bottom);
XAML_ISLANDS_API int xaml_stackpanel_set_corner_radius(XamlStackPanelHandle panel, double radius);

XAML_ISLANDS_API int xaml_grid_set_background(XamlGridHandle grid, unsigned int color);
XAML_ISLANDS_API int xaml_grid_set_padding(XamlGridHandle grid, double left, double top, double right, double bottom);
XAML_ISLANDS_API int xaml_grid_set_corner_radius(XamlGridHandle grid, double radius);

// ===== CheckBox APIs =====
XAML_ISLANDS_API XamlCheckBoxHandle xaml_checkbox_create();
XAML_ISLANDS_API int xaml_checkbox_set_content(XamlCheckBoxHandle handle, const wchar_t* content);
XAML_ISLANDS_API int xaml_checkbox_set_is_checked(XamlCheckBoxHandle handle, bool is_checked);
XAML_ISLANDS_API bool xaml_checkbox_get_is_checked(XamlCheckBoxHandle handle);

// ===== ComboBox APIs =====
XAML_ISLANDS_API XamlComboBoxHandle xaml_combobox_create();
XAML_ISLANDS_API int xaml_combobox_add_item(XamlComboBoxHandle handle, const wchar_t* item);
XAML_ISLANDS_API int xaml_combobox_set_selected_index(XamlComboBoxHandle handle, int index);
XAML_ISLANDS_API int xaml_combobox_get_selected_index(XamlComboBoxHandle handle);

// ===== Slider APIs =====
XAML_ISLANDS_API XamlSliderHandle xaml_slider_create();
XAML_ISLANDS_API int xaml_slider_set_minimum(XamlSliderHandle handle, double minimum);
XAML_ISLANDS_API int xaml_slider_set_maximum(XamlSliderHandle handle, double maximum);
XAML_ISLANDS_API int xaml_slider_set_value(XamlSliderHandle handle, double value);
XAML_ISLANDS_API double xaml_slider_get_value(XamlSliderHandle handle);

// ===== ProgressBar APIs =====
XAML_ISLANDS_API XamlProgressBarHandle xaml_progressbar_create();
XAML_ISLANDS_API int xaml_progressbar_set_minimum(XamlProgressBarHandle handle, double minimum);
XAML_ISLANDS_API int xaml_progressbar_set_maximum(XamlProgressBarHandle handle, double maximum);
XAML_ISLANDS_API int xaml_progressbar_set_value(XamlProgressBarHandle handle, double value);
XAML_ISLANDS_API int xaml_progressbar_set_is_indeterminate(XamlProgressBarHandle handle, bool is_indeterminate);

// ===== RadioButton APIs =====
XAML_ISLANDS_API XamlRadioButtonHandle xaml_radiobutton_create();
XAML_ISLANDS_API void xaml_radiobutton_destroy(XamlRadioButtonHandle radiobutton);
XAML_ISLANDS_API int xaml_radiobutton_set_content(XamlRadioButtonHandle radiobutton, const wchar_t* content);
XAML_ISLANDS_API int xaml_radiobutton_set_is_checked(XamlRadioButtonHandle radiobutton, int is_checked);
XAML_ISLANDS_API int xaml_radiobutton_get_is_checked(XamlRadioButtonHandle radiobutton);
XAML_ISLANDS_API int xaml_radiobutton_set_group_name(XamlRadioButtonHandle radiobutton, const wchar_t* group_name);
XAML_ISLANDS_API void xaml_radiobutton_on_checked(XamlRadioButtonHandle radiobutton, void* callback_ptr);
XAML_ISLANDS_API void xaml_radiobutton_on_unchecked(XamlRadioButtonHandle radiobutton, void* callback_ptr);

// ===== Image APIs =====
XAML_ISLANDS_API XamlImageHandle xaml_image_create();
XAML_ISLANDS_API void xaml_image_destroy(XamlImageHandle image);
XAML_ISLANDS_API int xaml_image_set_source(XamlImageHandle image, const wchar_t* uri);
XAML_ISLANDS_API int xaml_image_set_stretch(XamlImageHandle image, int stretch_mode);
XAML_ISLANDS_API int xaml_image_set_size(XamlImageHandle image, double width, double height);

// ===== Grid Row/Column Definition APIs =====
XAML_ISLANDS_API int xaml_grid_add_row_definition(XamlGridHandle grid, double height, int is_auto, int is_star);
XAML_ISLANDS_API int xaml_grid_add_column_definition(XamlGridHandle grid, double width, int is_auto, int is_star);
XAML_ISLANDS_API int xaml_grid_set_child_row(XamlUIElementHandle child, int row);
XAML_ISLANDS_API int xaml_grid_set_child_column(XamlUIElementHandle child, int column);
XAML_ISLANDS_API int xaml_grid_set_child_row_span(XamlUIElementHandle child, int row_span);
XAML_ISLANDS_API int xaml_grid_set_child_column_span(XamlUIElementHandle child, int column_span);

// ===== TextBox TextChanged Event =====
XAML_ISLANDS_API void xaml_textbox_on_text_changed(XamlTextBoxHandle textbox, void* callback_ptr);

// ===== Type Conversion APIs =====
XAML_ISLANDS_API XamlUIElementHandle xaml_button_as_uielement(XamlButtonHandle button);
XAML_ISLANDS_API XamlUIElementHandle xaml_textblock_as_uielement(XamlTextBlockHandle textblock);
XAML_ISLANDS_API XamlUIElementHandle xaml_textbox_as_uielement(XamlTextBoxHandle textbox);
XAML_ISLANDS_API XamlUIElementHandle xaml_stackpanel_as_uielement(XamlStackPanelHandle panel);
XAML_ISLANDS_API XamlUIElementHandle xaml_grid_as_uielement(XamlGridHandle grid);
XAML_ISLANDS_API XamlUIElementHandle xaml_scrollviewer_as_uielement(XamlScrollViewerHandle scrollviewer);
XAML_ISLANDS_API XamlUIElementHandle xaml_checkbox_as_uielement(XamlCheckBoxHandle checkbox);
XAML_ISLANDS_API XamlUIElementHandle xaml_combobox_as_uielement(XamlComboBoxHandle combobox);
XAML_ISLANDS_API XamlUIElementHandle xaml_slider_as_uielement(XamlSliderHandle slider);
XAML_ISLANDS_API XamlUIElementHandle xaml_progressbar_as_uielement(XamlProgressBarHandle progressbar);
XAML_ISLANDS_API XamlUIElementHandle xaml_radiobutton_as_uielement(XamlRadioButtonHandle radiobutton);
XAML_ISLANDS_API XamlUIElementHandle xaml_image_as_uielement(XamlImageHandle image);

// ============================================================================
// Resource Dictionary APIs
// ============================================================================

XAML_ISLANDS_API XamlResourceDictionaryHandle xaml_resource_dictionary_create();
XAML_ISLANDS_API void xaml_resource_dictionary_destroy(XamlResourceDictionaryHandle dict);
XAML_ISLANDS_API int xaml_resource_dictionary_insert_color(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key,
    unsigned int color
);
XAML_ISLANDS_API int xaml_resource_dictionary_insert_double(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key,
    double value
);
XAML_ISLANDS_API int xaml_resource_dictionary_insert_string(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key,
    const wchar_t* value
);
XAML_ISLANDS_API int xaml_resource_dictionary_has_key(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
);
XAML_ISLANDS_API unsigned int xaml_resource_dictionary_get_color(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
);
XAML_ISLANDS_API double xaml_resource_dictionary_get_double(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
);
XAML_ISLANDS_API int xaml_resource_dictionary_remove(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
);
XAML_ISLANDS_API void xaml_resource_dictionary_clear(XamlResourceDictionaryHandle dict);

// Apply resource dictionary to a UI element
XAML_ISLANDS_API int xaml_uielement_set_resources(
    XamlUIElementHandle element,
    XamlResourceDictionaryHandle dict
);

// ============================================================================
// Control Template APIs
// ============================================================================

XAML_ISLANDS_API XamlControlTemplateHandle xaml_control_template_create();
XAML_ISLANDS_API void xaml_control_template_destroy(XamlControlTemplateHandle template_handle);
XAML_ISLANDS_API int xaml_control_template_set_content(
    XamlControlTemplateHandle template_handle,
    XamlUIElementHandle content
);
XAML_ISLANDS_API int xaml_button_set_template(
    XamlButtonHandle button,
    XamlControlTemplateHandle template_handle
);

// ============================================================================
// Animation APIs
// ============================================================================

// Storyboard APIs
XAML_ISLANDS_API XamlStoryboardHandle xaml_storyboard_create();
XAML_ISLANDS_API void xaml_storyboard_destroy(XamlStoryboardHandle storyboard);
XAML_ISLANDS_API int xaml_storyboard_add_animation(
    XamlStoryboardHandle storyboard,
    XamlDoubleAnimationHandle animation
);
XAML_ISLANDS_API int xaml_storyboard_add_color_animation(
    XamlStoryboardHandle storyboard,
    XamlColorAnimationHandle animation
);
XAML_ISLANDS_API int xaml_storyboard_begin(XamlStoryboardHandle storyboard);
XAML_ISLANDS_API int xaml_storyboard_stop(XamlStoryboardHandle storyboard);
XAML_ISLANDS_API int xaml_storyboard_pause(XamlStoryboardHandle storyboard);
XAML_ISLANDS_API int xaml_storyboard_resume(XamlStoryboardHandle storyboard);
XAML_ISLANDS_API int xaml_storyboard_set_target(
    XamlStoryboardHandle storyboard,
    XamlUIElementHandle target
);

// DoubleAnimation APIs
XAML_ISLANDS_API XamlDoubleAnimationHandle xaml_double_animation_create();
XAML_ISLANDS_API void xaml_double_animation_destroy(XamlDoubleAnimationHandle animation);
XAML_ISLANDS_API int xaml_double_animation_set_from(XamlDoubleAnimationHandle animation, double from);
XAML_ISLANDS_API int xaml_double_animation_set_to(XamlDoubleAnimationHandle animation, double to);
XAML_ISLANDS_API int xaml_double_animation_set_duration(XamlDoubleAnimationHandle animation, int milliseconds);
XAML_ISLANDS_API int xaml_double_animation_set_target_property(
    XamlDoubleAnimationHandle animation,
    XamlUIElementHandle target,
    const wchar_t* property_path
);

// ColorAnimation APIs
XAML_ISLANDS_API XamlColorAnimationHandle xaml_color_animation_create();
XAML_ISLANDS_API void xaml_color_animation_destroy(XamlColorAnimationHandle animation);
XAML_ISLANDS_API int xaml_color_animation_set_from(XamlColorAnimationHandle animation, unsigned int from);
XAML_ISLANDS_API int xaml_color_animation_set_to(XamlColorAnimationHandle animation, unsigned int to);
XAML_ISLANDS_API int xaml_color_animation_set_duration(XamlColorAnimationHandle animation, int milliseconds);
XAML_ISLANDS_API int xaml_color_animation_set_target_property(
    XamlColorAnimationHandle animation,
    XamlUIElementHandle target,
    const wchar_t* property_path
);

#ifdef __cplusplus
}
#endif


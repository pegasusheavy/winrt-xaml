//! FFI bindings to the C++ XAML Islands helper DLL.

use std::ffi::c_void;
use windows::Win32::Foundation::HWND;

// Opaque handle types from C++
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlManagerHandle(pub *mut c_void);
unsafe impl Send for XamlManagerHandle {}
unsafe impl Sync for XamlManagerHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlSourceHandle(pub *mut c_void);
unsafe impl Send for XamlSourceHandle {}
unsafe impl Sync for XamlSourceHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlButtonHandle(pub *mut c_void);
unsafe impl Send for XamlButtonHandle {}
unsafe impl Sync for XamlButtonHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlTextBlockHandle(pub *mut c_void);
unsafe impl Send for XamlTextBlockHandle {}
unsafe impl Sync for XamlTextBlockHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlTextBoxHandle(pub *mut c_void);
unsafe impl Send for XamlTextBoxHandle {}
unsafe impl Sync for XamlTextBoxHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlStackPanelHandle(pub *mut c_void);
unsafe impl Send for XamlStackPanelHandle {}
unsafe impl Sync for XamlStackPanelHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlGridHandle(pub *mut c_void);
unsafe impl Send for XamlGridHandle {}
unsafe impl Sync for XamlGridHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlScrollViewerHandle(pub *mut c_void);
unsafe impl Send for XamlScrollViewerHandle {}
unsafe impl Sync for XamlScrollViewerHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlCheckBoxHandle(pub *mut c_void);
unsafe impl Send for XamlCheckBoxHandle {}
unsafe impl Sync for XamlCheckBoxHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlComboBoxHandle(pub *mut c_void);
unsafe impl Send for XamlComboBoxHandle {}
unsafe impl Sync for XamlComboBoxHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlSliderHandle(pub *mut c_void);
unsafe impl Send for XamlSliderHandle {}
unsafe impl Sync for XamlSliderHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlProgressBarHandle(pub *mut c_void);
unsafe impl Send for XamlProgressBarHandle {}
unsafe impl Sync for XamlProgressBarHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlRadioButtonHandle(pub *mut c_void);
unsafe impl Send for XamlRadioButtonHandle {}
unsafe impl Sync for XamlRadioButtonHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlImageHandle(pub *mut c_void);
unsafe impl Send for XamlImageHandle {}
unsafe impl Sync for XamlImageHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlListViewHandle(pub *mut c_void);
unsafe impl Send for XamlListViewHandle {}
unsafe impl Sync for XamlListViewHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlUIElementHandle(pub *mut c_void);
unsafe impl Send for XamlUIElementHandle {}
unsafe impl Sync for XamlUIElementHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlResourceDictionaryHandle(pub *mut c_void);
unsafe impl Send for XamlResourceDictionaryHandle {}
unsafe impl Sync for XamlResourceDictionaryHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlControlTemplateHandle(pub *mut c_void);
unsafe impl Send for XamlControlTemplateHandle {}
unsafe impl Sync for XamlControlTemplateHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlStoryboardHandle(pub *mut c_void);
unsafe impl Send for XamlStoryboardHandle {}
unsafe impl Sync for XamlStoryboardHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlDoubleAnimationHandle(pub *mut c_void);
unsafe impl Send for XamlDoubleAnimationHandle {}
unsafe impl Sync for XamlDoubleAnimationHandle {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XamlColorAnimationHandle(pub *mut c_void);
unsafe impl Send for XamlColorAnimationHandle {}
unsafe impl Sync for XamlColorAnimationHandle {}

// Raw FFI functions
#[link(name = "xaml_islands_helper", kind = "dylib")]
extern "C" {
    pub fn xaml_initialize() -> XamlManagerHandle;
    pub fn xaml_uninitialize(manager: XamlManagerHandle);

    pub fn xaml_source_create() -> XamlSourceHandle;
    pub fn xaml_source_destroy(source: XamlSourceHandle);
    pub fn xaml_source_attach_to_window(source: XamlSourceHandle, parent_hwnd: HWND) -> HWND;
    pub fn xaml_source_set_size(source: XamlSourceHandle, width: i32, height: i32) -> i32;
    pub fn xaml_source_set_content(source: XamlSourceHandle, button: XamlButtonHandle) -> i32;
    pub fn xaml_source_set_content_generic(source: XamlSourceHandle, element: XamlUIElementHandle) -> i32;

    pub fn xaml_button_create() -> XamlButtonHandle;
    pub fn xaml_button_destroy(button: XamlButtonHandle);
    pub fn xaml_button_set_content(button: XamlButtonHandle, content: *const u16) -> i32;
    pub fn xaml_button_set_size(button: XamlButtonHandle, width: f64, height: f64) -> i32;
    pub fn xaml_button_register_click(button: XamlButtonHandle, callback: extern "C" fn(*mut c_void), user_data: *mut c_void) -> i32;

    pub fn xaml_textblock_create() -> XamlTextBlockHandle;
    pub fn xaml_textblock_destroy(textblock: XamlTextBlockHandle);
    pub fn xaml_textblock_set_text(textblock: XamlTextBlockHandle, text: *const u16) -> i32;
    pub fn xaml_textblock_set_font_size(textblock: XamlTextBlockHandle, size: f64) -> i32;

    pub fn xaml_textbox_create() -> XamlTextBoxHandle;
    pub fn xaml_textbox_destroy(textbox: XamlTextBoxHandle);
    pub fn xaml_textbox_set_text(textbox: XamlTextBoxHandle, text: *const u16) -> i32;
    pub fn xaml_textbox_get_text(textbox: XamlTextBoxHandle, buffer: *mut u16, buffer_size: i32) -> i32;
    pub fn xaml_textbox_set_placeholder(textbox: XamlTextBoxHandle, placeholder: *const u16) -> i32;
    pub fn xaml_textbox_set_size(textbox: XamlTextBoxHandle, width: f64, height: f64) -> i32;

    pub fn xaml_stackpanel_create() -> XamlStackPanelHandle;
    pub fn xaml_stackpanel_destroy(panel: XamlStackPanelHandle);
    pub fn xaml_stackpanel_add_child(panel: XamlStackPanelHandle, child: XamlUIElementHandle) -> i32;
    pub fn xaml_stackpanel_set_orientation(panel: XamlStackPanelHandle, vertical: i32) -> i32;
    pub fn xaml_stackpanel_set_spacing(panel: XamlStackPanelHandle, spacing: f64) -> i32;

    pub fn xaml_grid_create() -> XamlGridHandle;
    pub fn xaml_grid_destroy(grid: XamlGridHandle);
    pub fn xaml_grid_add_child(grid: XamlGridHandle, child: XamlUIElementHandle) -> i32;

    pub fn xaml_scrollviewer_create() -> XamlScrollViewerHandle;
    pub fn xaml_scrollviewer_destroy(scrollviewer: XamlScrollViewerHandle);
    pub fn xaml_scrollviewer_set_content(scrollviewer: XamlScrollViewerHandle, content: XamlUIElementHandle) -> i32;
    pub fn xaml_scrollviewer_set_horizontal_scroll_mode(scrollviewer: XamlScrollViewerHandle, mode: i32) -> i32;
    pub fn xaml_scrollviewer_set_vertical_scroll_mode(scrollviewer: XamlScrollViewerHandle, mode: i32) -> i32;
    pub fn xaml_scrollviewer_set_horizontal_scroll_bar_visibility(scrollviewer: XamlScrollViewerHandle, visibility: i32) -> i32;
    pub fn xaml_scrollviewer_set_vertical_scroll_bar_visibility(scrollviewer: XamlScrollViewerHandle, visibility: i32) -> i32;

    // Styling APIs
    pub fn xaml_button_set_background(button: XamlButtonHandle, color: u32) -> i32;
    pub fn xaml_button_set_foreground(button: XamlButtonHandle, color: u32) -> i32;
    pub fn xaml_button_set_corner_radius(button: XamlButtonHandle, radius: f64) -> i32;
    pub fn xaml_button_set_padding(button: XamlButtonHandle, left: f64, top: f64, right: f64, bottom: f64) -> i32;

    pub fn xaml_textblock_set_foreground(textblock: XamlTextBlockHandle, color: u32) -> i32;
    pub fn xaml_textblock_set_font_weight(textblock: XamlTextBlockHandle, weight: i32) -> i32;
    pub fn xaml_textblock_set_margin(textblock: XamlTextBlockHandle, left: f64, top: f64, right: f64, bottom: f64) -> i32;

    pub fn xaml_textbox_set_background(textbox: XamlTextBoxHandle, color: u32) -> i32;
    pub fn xaml_textbox_set_foreground(textbox: XamlTextBoxHandle, color: u32) -> i32;
    pub fn xaml_textbox_set_corner_radius(textbox: XamlTextBoxHandle, radius: f64) -> i32;
    pub fn xaml_textbox_set_padding(textbox: XamlTextBoxHandle, left: f64, top: f64, right: f64, bottom: f64) -> i32;

    pub fn xaml_stackpanel_set_background(panel: XamlStackPanelHandle, color: u32) -> i32;
    pub fn xaml_stackpanel_set_padding(panel: XamlStackPanelHandle, left: f64, top: f64, right: f64, bottom: f64) -> i32;
    pub fn xaml_stackpanel_set_corner_radius(panel: XamlStackPanelHandle, radius: f64) -> i32;

    pub fn xaml_grid_set_background(grid: XamlGridHandle, color: u32) -> i32;
    pub fn xaml_grid_set_padding(grid: XamlGridHandle, left: f64, top: f64, right: f64, bottom: f64) -> i32;
    pub fn xaml_grid_set_corner_radius(grid: XamlGridHandle, radius: f64) -> i32;

    // CheckBox APIs
    pub fn xaml_checkbox_create() -> XamlCheckBoxHandle;
    pub fn xaml_checkbox_set_content(checkbox: XamlCheckBoxHandle, content: *const u16) -> i32;
    pub fn xaml_checkbox_set_is_checked(checkbox: XamlCheckBoxHandle, is_checked: bool) -> i32;
    pub fn xaml_checkbox_get_is_checked(checkbox: XamlCheckBoxHandle) -> bool;

    // ComboBox APIs
    pub fn xaml_combobox_create() -> XamlComboBoxHandle;
    pub fn xaml_combobox_add_item(combobox: XamlComboBoxHandle, item: *const u16) -> i32;
    pub fn xaml_combobox_set_selected_index(combobox: XamlComboBoxHandle, index: i32) -> i32;
    pub fn xaml_combobox_get_selected_index(combobox: XamlComboBoxHandle) -> i32;

    // Slider APIs
    pub fn xaml_slider_create() -> XamlSliderHandle;
    pub fn xaml_slider_set_minimum(slider: XamlSliderHandle, minimum: f64) -> i32;
    pub fn xaml_slider_set_maximum(slider: XamlSliderHandle, maximum: f64) -> i32;
    pub fn xaml_slider_set_value(slider: XamlSliderHandle, value: f64) -> i32;
    pub fn xaml_slider_get_value(slider: XamlSliderHandle) -> f64;

    // ProgressBar APIs
    pub fn xaml_progressbar_create() -> XamlProgressBarHandle;
    pub fn xaml_progressbar_set_minimum(progressbar: XamlProgressBarHandle, minimum: f64) -> i32;
    pub fn xaml_progressbar_set_maximum(progressbar: XamlProgressBarHandle, maximum: f64) -> i32;
    pub fn xaml_progressbar_set_value(progressbar: XamlProgressBarHandle, value: f64) -> i32;
    pub fn xaml_progressbar_set_is_indeterminate(progressbar: XamlProgressBarHandle, is_indeterminate: bool) -> i32;

    // Type conversion
    pub fn xaml_button_as_uielement(button: XamlButtonHandle) -> XamlUIElementHandle;
    pub fn xaml_textblock_as_uielement(textblock: XamlTextBlockHandle) -> XamlUIElementHandle;
    pub fn xaml_textbox_as_uielement(textbox: XamlTextBoxHandle) -> XamlUIElementHandle;
    pub fn xaml_stackpanel_as_uielement(panel: XamlStackPanelHandle) -> XamlUIElementHandle;
    pub fn xaml_grid_as_uielement(grid: XamlGridHandle) -> XamlUIElementHandle;
    pub fn xaml_scrollviewer_as_uielement(scrollviewer: XamlScrollViewerHandle) -> XamlUIElementHandle;
    pub fn xaml_checkbox_as_uielement(checkbox: XamlCheckBoxHandle) -> XamlUIElementHandle;
    pub fn xaml_combobox_as_uielement(combobox: XamlComboBoxHandle) -> XamlUIElementHandle;
    pub fn xaml_slider_as_uielement(slider: XamlSliderHandle) -> XamlUIElementHandle;
    pub fn xaml_progressbar_as_uielement(progressbar: XamlProgressBarHandle) -> XamlUIElementHandle;

    // RadioButton APIs
    pub fn xaml_radiobutton_create() -> XamlRadioButtonHandle;
    pub fn xaml_radiobutton_destroy(radiobutton: XamlRadioButtonHandle);
    pub fn xaml_radiobutton_set_content(radiobutton: XamlRadioButtonHandle, content: *const u16) -> i32;
    pub fn xaml_radiobutton_set_is_checked(radiobutton: XamlRadioButtonHandle, is_checked: i32) -> i32;
    pub fn xaml_radiobutton_get_is_checked(radiobutton: XamlRadioButtonHandle) -> i32;
    pub fn xaml_radiobutton_set_group_name(radiobutton: XamlRadioButtonHandle, group_name: *const u16) -> i32;
    pub fn xaml_radiobutton_on_checked(radiobutton: XamlRadioButtonHandle, callback_ptr: *mut c_void);
    pub fn xaml_radiobutton_on_unchecked(radiobutton: XamlRadioButtonHandle, callback_ptr: *mut c_void);
    pub fn xaml_radiobutton_as_uielement(radiobutton: XamlRadioButtonHandle) -> XamlUIElementHandle;

    // Image APIs
    pub fn xaml_image_create() -> XamlImageHandle;
    pub fn xaml_image_destroy(image: XamlImageHandle);
    pub fn xaml_image_set_source(image: XamlImageHandle, uri: *const u16) -> i32;
    pub fn xaml_image_set_stretch(image: XamlImageHandle, stretch_mode: i32) -> i32;
    pub fn xaml_image_set_size(image: XamlImageHandle, width: f64, height: f64) -> i32;
    pub fn xaml_image_as_uielement(image: XamlImageHandle) -> XamlUIElementHandle;

    // Grid row/column definition APIs
    pub fn xaml_grid_add_row_definition(grid: XamlGridHandle, height: f64, is_auto: i32, is_star: i32) -> i32;
    pub fn xaml_grid_add_column_definition(grid: XamlGridHandle, width: f64, is_auto: i32, is_star: i32) -> i32;
    pub fn xaml_grid_set_child_row(child: XamlUIElementHandle, row: i32) -> i32;
    pub fn xaml_grid_set_child_column(child: XamlUIElementHandle, column: i32) -> i32;
    pub fn xaml_grid_set_child_row_span(child: XamlUIElementHandle, row_span: i32) -> i32;
    pub fn xaml_grid_set_child_column_span(child: XamlUIElementHandle, column_span: i32) -> i32;

    // TextBox TextChanged event
    pub fn xaml_textbox_on_text_changed(textbox: XamlTextBoxHandle, callback_ptr: *mut c_void);

    // ListView APIs
    pub fn xaml_listview_create() -> XamlListViewHandle;
    pub fn xaml_listview_destroy(listview: XamlListViewHandle);
    pub fn xaml_listview_add_item(listview: XamlListViewHandle, item: *const u16) -> i32;
    pub fn xaml_listview_remove_item(listview: XamlListViewHandle, index: i32) -> i32;
    pub fn xaml_listview_clear_items(listview: XamlListViewHandle) -> i32;
    pub fn xaml_listview_get_item_count(listview: XamlListViewHandle) -> i32;
    pub fn xaml_listview_get_selected_index(listview: XamlListViewHandle) -> i32;
    pub fn xaml_listview_set_selected_index(listview: XamlListViewHandle, index: i32) -> i32;
    pub fn xaml_listview_get_item(listview: XamlListViewHandle, index: i32, buffer: *mut u16, buffer_size: i32) -> i32;
    pub fn xaml_listview_on_selection_changed(listview: XamlListViewHandle, callback_ptr: *mut c_void);
    pub fn xaml_listview_set_selection_mode(listview: XamlListViewHandle, mode: i32) -> i32;
    pub fn xaml_listview_as_uielement(listview: XamlListViewHandle) -> XamlUIElementHandle;

    // Resource Dictionary APIs
    pub fn xaml_resource_dictionary_create() -> XamlResourceDictionaryHandle;
    pub fn xaml_resource_dictionary_destroy(dict: XamlResourceDictionaryHandle);
    pub fn xaml_resource_dictionary_insert_color(dict: XamlResourceDictionaryHandle, key: *const u16, color: u32) -> i32;
    pub fn xaml_resource_dictionary_insert_double(dict: XamlResourceDictionaryHandle, key: *const u16, value: f64) -> i32;
    pub fn xaml_resource_dictionary_insert_string(dict: XamlResourceDictionaryHandle, key: *const u16, value: *const u16) -> i32;
    pub fn xaml_resource_dictionary_has_key(dict: XamlResourceDictionaryHandle, key: *const u16) -> i32;
    pub fn xaml_resource_dictionary_get_color(dict: XamlResourceDictionaryHandle, key: *const u16) -> u32;
    pub fn xaml_resource_dictionary_get_double(dict: XamlResourceDictionaryHandle, key: *const u16) -> f64;
    pub fn xaml_resource_dictionary_remove(dict: XamlResourceDictionaryHandle, key: *const u16) -> i32;
    pub fn xaml_resource_dictionary_clear(dict: XamlResourceDictionaryHandle);
    pub fn xaml_uielement_set_resources(element: XamlUIElementHandle, dict: XamlResourceDictionaryHandle) -> i32;

    // Control Template APIs
    pub fn xaml_control_template_create() -> XamlControlTemplateHandle;
    pub fn xaml_control_template_destroy(template_handle: XamlControlTemplateHandle);
    pub fn xaml_control_template_set_content(template_handle: XamlControlTemplateHandle, content: XamlUIElementHandle) -> i32;
    pub fn xaml_button_set_template(button: XamlButtonHandle, template_handle: XamlControlTemplateHandle) -> i32;

    // Animation APIs - Storyboard
    pub fn xaml_storyboard_create() -> XamlStoryboardHandle;
    pub fn xaml_storyboard_destroy(storyboard: XamlStoryboardHandle);
    pub fn xaml_storyboard_add_animation(storyboard: XamlStoryboardHandle, animation: XamlDoubleAnimationHandle) -> i32;
    pub fn xaml_storyboard_add_color_animation(storyboard: XamlStoryboardHandle, animation: XamlColorAnimationHandle) -> i32;
    pub fn xaml_storyboard_begin(storyboard: XamlStoryboardHandle) -> i32;
    pub fn xaml_storyboard_stop(storyboard: XamlStoryboardHandle) -> i32;
    pub fn xaml_storyboard_pause(storyboard: XamlStoryboardHandle) -> i32;
    pub fn xaml_storyboard_resume(storyboard: XamlStoryboardHandle) -> i32;
    pub fn xaml_storyboard_set_target(storyboard: XamlStoryboardHandle, target: XamlUIElementHandle) -> i32;

    // Animation APIs - DoubleAnimation
    pub fn xaml_double_animation_create() -> XamlDoubleAnimationHandle;
    pub fn xaml_double_animation_destroy(animation: XamlDoubleAnimationHandle);
    pub fn xaml_double_animation_set_from(animation: XamlDoubleAnimationHandle, from: f64) -> i32;
    pub fn xaml_double_animation_set_to(animation: XamlDoubleAnimationHandle, to: f64) -> i32;
    pub fn xaml_double_animation_set_duration(animation: XamlDoubleAnimationHandle, milliseconds: i32) -> i32;
    pub fn xaml_double_animation_set_target_property(animation: XamlDoubleAnimationHandle, target: XamlUIElementHandle, property_path: *const u16) -> i32;

    // Animation APIs - ColorAnimation
    pub fn xaml_color_animation_create() -> XamlColorAnimationHandle;
    pub fn xaml_color_animation_destroy(animation: XamlColorAnimationHandle);
    pub fn xaml_color_animation_set_from(animation: XamlColorAnimationHandle, from: u32) -> i32;
    pub fn xaml_color_animation_set_to(animation: XamlColorAnimationHandle, to: u32) -> i32;
    pub fn xaml_color_animation_set_duration(animation: XamlColorAnimationHandle, milliseconds: i32) -> i32;
    pub fn xaml_color_animation_set_target_property(animation: XamlColorAnimationHandle, target: XamlUIElementHandle, property_path: *const u16) -> i32;

    pub fn xaml_get_last_error() -> *const u16;
}

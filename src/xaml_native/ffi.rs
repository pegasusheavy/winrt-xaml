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
pub struct XamlUIElementHandle(pub *mut c_void);
unsafe impl Send for XamlUIElementHandle {}
unsafe impl Sync for XamlUIElementHandle {}

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

    pub fn xaml_button_as_uielement(button: XamlButtonHandle) -> XamlUIElementHandle;
    pub fn xaml_textblock_as_uielement(textblock: XamlTextBlockHandle) -> XamlUIElementHandle;
    pub fn xaml_textbox_as_uielement(textbox: XamlTextBoxHandle) -> XamlUIElementHandle;
    pub fn xaml_stackpanel_as_uielement(panel: XamlStackPanelHandle) -> XamlUIElementHandle;
    pub fn xaml_grid_as_uielement(grid: XamlGridHandle) -> XamlUIElementHandle;

    pub fn xaml_get_last_error() -> *const u16;
}

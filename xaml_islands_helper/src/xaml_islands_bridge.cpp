#include "xaml_islands_bridge.h"
#include <winrt/Windows.Foundation.h>
#include <winrt/Windows.Foundation.Collections.h>
#include <winrt/Windows.UI.Xaml.h>
#include <winrt/Windows.UI.Xaml.Controls.h>
#include <winrt/Windows.UI.Xaml.Controls.Primitives.h>
#include <winrt/Windows.UI.Xaml.Hosting.h>
#include <winrt/Windows.UI.Xaml.Media.h>
#include <winrt/Windows.UI.Xaml.Media.Animation.h>
#include <Windows.UI.Xaml.Hosting.DesktopWindowXamlSource.h>
#include <string>
#include <memory>

using namespace winrt;
using namespace Windows::Foundation;
using namespace Windows::UI;
using namespace Windows::UI::Xaml;
using namespace Windows::UI::Xaml::Controls;
using namespace Windows::UI::Xaml::Hosting;
using namespace Windows::UI::Xaml::Media;
using namespace Windows::UI::Xaml::Media::Animation;

// Thread-local error message
thread_local std::wstring g_last_error;

void set_last_error(const wchar_t* message) {
    g_last_error = message;
}

void set_last_error(const std::wstring& message) {
    g_last_error = message;
}

// Initialize the XAML framework
XamlManagerHandle xaml_initialize() {
    try {
        init_apartment(apartment_type::single_threaded);

        auto manager = WindowsXamlManager::InitializeForCurrentThread();

        // Store in a shared_ptr for proper lifetime management
        auto* handle = new std::shared_ptr<WindowsXamlManager>(
            std::make_shared<WindowsXamlManager>(manager)
        );
        return reinterpret_cast<XamlManagerHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_initialize");
        return nullptr;
    }
}

// Uninitialize the XAML framework
void xaml_uninitialize(XamlManagerHandle manager) {
    if (manager) {
        auto* mgr = reinterpret_cast<std::shared_ptr<WindowsXamlManager>*>(manager);
        delete mgr;
    }
}

// Create a DesktopWindowXamlSource
XamlSourceHandle xaml_source_create() {
    try {
        auto source = DesktopWindowXamlSource();
        auto* handle = new std::shared_ptr<DesktopWindowXamlSource>(
            std::make_shared<DesktopWindowXamlSource>(source)
        );
        return reinterpret_cast<XamlSourceHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_source_create");
        return nullptr;
    }
}

// Destroy a DesktopWindowXamlSource
void xaml_source_destroy(XamlSourceHandle source) {
    if (source) {
        auto* src = reinterpret_cast<std::shared_ptr<DesktopWindowXamlSource>*>(source);
        delete src;
    }
}

// Attach XAML source to a Win32 window
HWND xaml_source_attach_to_window(XamlSourceHandle source, HWND parent_hwnd) {
    if (!source || !parent_hwnd) {
        set_last_error(L"Invalid source or parent HWND");
        return nullptr;
    }

    try {
        auto* src = reinterpret_cast<std::shared_ptr<DesktopWindowXamlSource>*>(source);

        // Get the IDesktopWindowXamlSourceNative interface
        auto interop = (*src)->as<IDesktopWindowXamlSourceNative>();

        // Attach to the window
        HWND island_hwnd = nullptr;
        check_hresult(interop->AttachToWindow(parent_hwnd));
        check_hresult(interop->get_WindowHandle(&island_hwnd));

        return island_hwnd;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_source_attach_to_window");
        return nullptr;
    }
}

// Set the size of the XAML island
int xaml_source_set_size(XamlSourceHandle source, int width, int height) {
    // Size is managed by the parent window, no-op for now
    return 0;
}

// Create a WinRT Button
XamlButtonHandle xaml_button_create() {
    try {
        auto button = Button();
        auto* handle = new std::shared_ptr<Button>(
            std::make_shared<Button>(button)
        );
        return reinterpret_cast<XamlButtonHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_create");
        return nullptr;
    }
}

// Destroy a WinRT Button
void xaml_button_destroy(XamlButtonHandle button) {
    if (button) {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        delete btn;
    }
}

// Set button content
int xaml_button_set_content(XamlButtonHandle button, const wchar_t* content) {
    if (!button || !content) {
        set_last_error(L"Invalid button or content");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        (*btn)->Content(box_value(hstring(content)));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_content");
        return -1;
    }
}

// Set button size
int xaml_button_set_size(XamlButtonHandle button, double width, double height) {
    if (!button) {
        set_last_error(L"Invalid button");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        (*btn)->Width(width);
        (*btn)->Height(height);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_size");
        return -1;
    }
}

// Register a click event handler for a button
int xaml_button_register_click(XamlButtonHandle button, void (*callback)(void* user_data), void* user_data) {
    if (!button || !callback) {
        set_last_error(L"Invalid button or callback");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);

        // Register the Click event handler
        // We explicitly specify event_token to help the compiler
        winrt::event_token token = (*btn)->Click([callback, user_data](IInspectable const& sender, RoutedEventArgs const& args) {
            // Call the C callback from Rust
            callback(user_data);
        });

        // Note: In a production app, you'd want to store this token to unregister later
        // For now, the event will remain registered for the button's lifetime
        (void)token; // Suppress unused variable warning

        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_register_click");
        return -1;
    }
}

// Set the XAML content
int xaml_source_set_content(XamlSourceHandle source, XamlButtonHandle button) {
    if (!source || !button) {
        set_last_error(L"Invalid source or button");
        return -1;
    }

    try {
        auto* src = reinterpret_cast<std::shared_ptr<DesktopWindowXamlSource>*>(source);
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);

        (*src)->Content(**btn);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_source_set_content");
        return -1;
    }
}

// Get last error
const wchar_t* xaml_get_last_error() {
    return g_last_error.c_str();
}

// ===== TextBlock Implementation =====
XamlTextBlockHandle xaml_textblock_create() {
    try {
        auto textblock = TextBlock();
        auto* handle = new std::shared_ptr<TextBlock>(
            std::make_shared<TextBlock>(textblock)
        );
        return reinterpret_cast<XamlTextBlockHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textblock_create");
        return nullptr;
    }
}

void xaml_textblock_destroy(XamlTextBlockHandle textblock) {
    if (textblock) {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        delete tb;
    }
}

int xaml_textblock_set_text(XamlTextBlockHandle textblock, const wchar_t* text) {
    if (!textblock || !text) {
        set_last_error(L"Invalid textblock or text");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        (*tb)->Text(text);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textblock_set_text");
        return -1;
    }
}

int xaml_textblock_set_font_size(XamlTextBlockHandle textblock, double size) {
    if (!textblock) {
        set_last_error(L"Invalid textblock");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        (*tb)->FontSize(size);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textblock_set_font_size");
        return -1;
    }
}

// ===== TextBox Implementation =====
XamlTextBoxHandle xaml_textbox_create() {
    try {
        auto textbox = TextBox();
        auto* handle = new std::shared_ptr<TextBox>(
            std::make_shared<TextBox>(textbox)
        );
        return reinterpret_cast<XamlTextBoxHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_create");
        return nullptr;
    }
}

void xaml_textbox_destroy(XamlTextBoxHandle textbox) {
    if (textbox) {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        delete tb;
    }
}

int xaml_textbox_set_text(XamlTextBoxHandle textbox, const wchar_t* text) {
    if (!textbox || !text) {
        set_last_error(L"Invalid textbox or text");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->Text(text);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_text");
        return -1;
    }
}

// Get the text content from a TextBox
int xaml_textbox_get_text(XamlTextBoxHandle textbox, wchar_t* buffer, int buffer_size) {
    if (!textbox || !buffer || buffer_size <= 0) {
        set_last_error(L"Invalid textbox, buffer, or buffer size");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        auto text = (*tb)->Text();

        // Copy the text to the buffer
        int len = static_cast<int>(text.size());
        if (len >= buffer_size) {
            len = buffer_size - 1; // Leave room for null terminator
        }

        wcsncpy_s(buffer, buffer_size, text.c_str(), len);
        buffer[len] = L'\0';

        return len; // Return the number of characters copied
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_get_text");
        return -1;
    }
}

int xaml_textbox_set_placeholder(XamlTextBoxHandle textbox, const wchar_t* placeholder) {
    if (!textbox || !placeholder) {
        set_last_error(L"Invalid textbox or placeholder");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->PlaceholderText(placeholder);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_placeholder");
        return -1;
    }
}

int xaml_textbox_set_size(XamlTextBoxHandle textbox, double width, double height) {
    if (!textbox) {
        set_last_error(L"Invalid textbox");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->Width(width);
        (*tb)->Height(height);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_size");
        return -1;
    }
}

// ===== StackPanel Implementation =====
XamlStackPanelHandle xaml_stackpanel_create() {
    try {
        auto panel = StackPanel();
        auto* handle = new std::shared_ptr<StackPanel>(
            std::make_shared<StackPanel>(panel)
        );
        return reinterpret_cast<XamlStackPanelHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_create");
        return nullptr;
    }
}

void xaml_stackpanel_destroy(XamlStackPanelHandle panel) {
    if (panel) {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        delete sp;
    }
}

int xaml_stackpanel_add_child(XamlStackPanelHandle panel, XamlUIElementHandle child) {
    if (!panel || !child) {
        set_last_error(L"Invalid panel or child");
        return -1;
    }

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        auto* element = reinterpret_cast<std::shared_ptr<UIElement>*>(child);

        // Cast to Panel to access Children collection
        auto panel_obj = (*sp)->as<Panel>();
        auto children = panel_obj.Children();
        children.Append(**element);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_add_child");
        return -1;
    }
}

int xaml_stackpanel_set_orientation(XamlStackPanelHandle panel, int vertical) {
    if (!panel) {
        set_last_error(L"Invalid panel");
        return -1;
    }

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        (*sp)->Orientation(vertical ? Orientation::Vertical : Orientation::Horizontal);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_set_orientation");
        return -1;
    }
}

int xaml_stackpanel_set_spacing(XamlStackPanelHandle panel, double spacing) {
    if (!panel) {
        set_last_error(L"Invalid panel");
        return -1;
    }

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        (*sp)->Spacing(spacing);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_set_spacing");
        return -1;
    }
}

// ===== Grid Implementation =====
XamlGridHandle xaml_grid_create() {
    try {
        auto grid = Grid();
        auto* handle = new std::shared_ptr<Grid>(
            std::make_shared<Grid>(grid)
        );
        return reinterpret_cast<XamlGridHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_grid_create");
        return nullptr;
    }
}

void xaml_grid_destroy(XamlGridHandle grid) {
    if (grid) {
        auto* g = reinterpret_cast<std::shared_ptr<Grid>*>(grid);
        delete g;
    }
}

int xaml_grid_add_child(XamlGridHandle grid, XamlUIElementHandle child) {
    if (!grid || !child) {
        set_last_error(L"Invalid grid or child");
        return -1;
    }

    try {
        auto* g = reinterpret_cast<std::shared_ptr<Grid>*>(grid);
        auto* element = reinterpret_cast<std::shared_ptr<UIElement>*>(child);

        // Cast to Panel to access Children collection
        auto panel_obj = (*g)->as<Panel>();
        auto children = panel_obj.Children();
        children.Append(**element);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_grid_add_child");
        return -1;
    }
}

// ===== ScrollViewer APIs =====
XamlScrollViewerHandle xaml_scrollviewer_create() {
    try {
        auto scrollviewer = ScrollViewer();
        auto* handle = new std::shared_ptr<ScrollViewer>(
            std::make_shared<ScrollViewer>(scrollviewer)
        );
        return reinterpret_cast<XamlScrollViewerHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_scrollviewer_create");
        return nullptr;
    }
}

void xaml_scrollviewer_destroy(XamlScrollViewerHandle scrollviewer) {
    if (scrollviewer) {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        delete sv;
    }
}

int xaml_scrollviewer_set_content(XamlScrollViewerHandle scrollviewer, XamlUIElementHandle content) {
    if (!scrollviewer || !content) {
        set_last_error(L"Invalid scrollviewer or content");
        return -1;
    }

    try {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        auto* element = reinterpret_cast<std::shared_ptr<UIElement>*>(content);
        (*sv)->Content(**element);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_scrollviewer_set_content");
        return -1;
    }
}

int xaml_scrollviewer_set_horizontal_scroll_mode(XamlScrollViewerHandle scrollviewer, int mode) {
    if (!scrollviewer) {
        set_last_error(L"Invalid scrollviewer handle");
        return -1;
    }

    try {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        // Mode: 0 = Disabled, 1 = Enabled, 2 = Auto
        auto scroll_mode = static_cast<ScrollMode>(mode);
        (*sv)->HorizontalScrollMode(scroll_mode);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_scrollviewer_set_horizontal_scroll_mode");
        return -1;
    }
}

int xaml_scrollviewer_set_vertical_scroll_mode(XamlScrollViewerHandle scrollviewer, int mode) {
    if (!scrollviewer) {
        set_last_error(L"Invalid scrollviewer handle");
        return -1;
    }

    try {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        // Mode: 0 = Disabled, 1 = Enabled, 2 = Auto
        auto scroll_mode = static_cast<ScrollMode>(mode);
        (*sv)->VerticalScrollMode(scroll_mode);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_scrollviewer_set_vertical_scroll_mode");
        return -1;
    }
}

int xaml_scrollviewer_set_horizontal_scroll_bar_visibility(XamlScrollViewerHandle scrollviewer, int visibility) {
    if (!scrollviewer) {
        set_last_error(L"Invalid scrollviewer handle");
        return -1;
    }

    try {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        // Visibility: 0 = Disabled, 1 = Auto, 2 = Hidden, 3 = Visible
        auto vis = static_cast<ScrollBarVisibility>(visibility);
        (*sv)->HorizontalScrollBarVisibility(vis);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_scrollviewer_set_horizontal_scroll_bar_visibility");
        return -1;
    }
}

int xaml_scrollviewer_set_vertical_scroll_bar_visibility(XamlScrollViewerHandle scrollviewer, int visibility) {
    if (!scrollviewer) {
        set_last_error(L"Invalid scrollviewer handle");
        return -1;
    }

    try {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        // Visibility: 0 = Disabled, 1 = Auto, 2 = Hidden, 3 = Visible
        auto vis = static_cast<ScrollBarVisibility>(visibility);
        (*sv)->VerticalScrollBarVisibility(vis);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_scrollviewer_set_vertical_scroll_bar_visibility");
        return -1;
    }
}

// ===== Generic Content API =====
int xaml_source_set_content_generic(XamlSourceHandle source, XamlUIElementHandle element) {
    if (!source || !element) {
        set_last_error(L"Invalid source or element");
        return -1;
    }

    try {
        auto* src = reinterpret_cast<std::shared_ptr<DesktopWindowXamlSource>*>(source);
        auto* elem = reinterpret_cast<std::shared_ptr<UIElement>*>(element);
        (*src)->Content(**elem);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_source_set_content_generic");
        return -1;
    }
}

// ===== Type Conversion APIs =====
XamlUIElementHandle xaml_button_as_uielement(XamlButtonHandle button) {
    if (!button) return nullptr;

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        auto* handle = new std::shared_ptr<UIElement>(
            std::make_shared<UIElement>((*btn)->as<UIElement>())
        );
        return reinterpret_cast<XamlUIElementHandle>(handle);
    }
    catch (...) {
        set_last_error(L"Error converting button to UIElement");
        return nullptr;
    }
}

XamlUIElementHandle xaml_textblock_as_uielement(XamlTextBlockHandle textblock) {
    if (!textblock) return nullptr;

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        auto* handle = new std::shared_ptr<UIElement>(
            std::make_shared<UIElement>((*tb)->as<UIElement>())
        );
        return reinterpret_cast<XamlUIElementHandle>(handle);
    }
    catch (...) {
        set_last_error(L"Error converting textblock to UIElement");
        return nullptr;
    }
}

XamlUIElementHandle xaml_textbox_as_uielement(XamlTextBoxHandle textbox) {
    if (!textbox) return nullptr;

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        auto* handle = new std::shared_ptr<UIElement>(
            std::make_shared<UIElement>((*tb)->as<UIElement>())
        );
        return reinterpret_cast<XamlUIElementHandle>(handle);
    }
    catch (...) {
        set_last_error(L"Error converting textbox to UIElement");
        return nullptr;
    }
}

XamlUIElementHandle xaml_stackpanel_as_uielement(XamlStackPanelHandle panel) {
    if (!panel) return nullptr;

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        auto* handle = new std::shared_ptr<UIElement>(
            std::make_shared<UIElement>((*sp)->as<UIElement>())
        );
        return reinterpret_cast<XamlUIElementHandle>(handle);
    }
    catch (...) {
        set_last_error(L"Error converting stackpanel to UIElement");
        return nullptr;
    }
}

XamlUIElementHandle xaml_grid_as_uielement(XamlGridHandle grid) {
    if (!grid) return nullptr;

    try {
        auto* g = reinterpret_cast<std::shared_ptr<Grid>*>(grid);
        auto* handle = new std::shared_ptr<UIElement>(
            std::make_shared<UIElement>((*g)->as<UIElement>())
        );
        return reinterpret_cast<XamlUIElementHandle>(handle);
    }
    catch (...) {
        set_last_error(L"Error converting grid to UIElement");
        return nullptr;
    }
}

XamlUIElementHandle xaml_scrollviewer_as_uielement(XamlScrollViewerHandle scrollviewer) {
    if (!scrollviewer) return nullptr;

    try {
        auto* sv = reinterpret_cast<std::shared_ptr<ScrollViewer>*>(scrollviewer);
        auto* handle = new std::shared_ptr<UIElement>(
            std::make_shared<UIElement>((*sv)->as<UIElement>())
        );
        return reinterpret_cast<XamlUIElementHandle>(handle);
    }
    catch (...) {
        set_last_error(L"Error converting scrollviewer to UIElement");
        return nullptr;
    }
}

// ===== Styling Implementations =====
#include <winrt/Windows.UI.h>
#include <winrt/Windows.UI.Xaml.Media.h>

using namespace Windows::UI;
using namespace Windows::UI::Xaml::Media;

// Helper to create a SolidColorBrush from ARGB
SolidColorBrush create_solid_brush(unsigned int argb) {
    byte a = (argb >> 24) & 0xFF;
    byte r = (argb >> 16) & 0xFF;
    byte g = (argb >> 8) & 0xFF;
    byte b = argb & 0xFF;
    Color color{ a, r, g, b };
    return SolidColorBrush(color);
}

// Button styling
int xaml_button_set_background(XamlButtonHandle button, unsigned int color) {
    if (!button) {
        set_last_error(L"Invalid button handle");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        (*btn)->Background(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_background");
        return -1;
    }
}

int xaml_button_set_foreground(XamlButtonHandle button, unsigned int color) {
    if (!button) {
        set_last_error(L"Invalid button handle");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        (*btn)->Foreground(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_foreground");
        return -1;
    }
}

int xaml_button_set_corner_radius(XamlButtonHandle button, double radius) {
    if (!button) {
        set_last_error(L"Invalid button handle");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        (*btn)->CornerRadius(CornerRadius{ radius, radius, radius, radius });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_corner_radius");
        return -1;
    }
}

int xaml_button_set_padding(XamlButtonHandle button, double left, double top, double right, double bottom) {
    if (!button) {
        set_last_error(L"Invalid button handle");
        return -1;
    }

    try {
        auto* btn = reinterpret_cast<std::shared_ptr<Button>*>(button);
        (*btn)->Padding(Thickness{ left, top, right, bottom });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_padding");
        return -1;
    }
}

// TextBlock styling
int xaml_textblock_set_foreground(XamlTextBlockHandle textblock, unsigned int color) {
    if (!textblock) {
        set_last_error(L"Invalid textblock handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        (*tb)->Foreground(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textblock_set_foreground");
        return -1;
    }
}

int xaml_textblock_set_font_weight(XamlTextBlockHandle textblock, int weight) {
    if (!textblock) {
        set_last_error(L"Invalid textblock handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        Windows::UI::Text::FontWeight fw;
        fw.Weight = static_cast<uint16_t>(weight);
        (*tb)->FontWeight(fw);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textblock_set_font_weight");
        return -1;
    }
}

int xaml_textblock_set_margin(XamlTextBlockHandle textblock, double left, double top, double right, double bottom) {
    if (!textblock) {
        set_last_error(L"Invalid textblock handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBlock>*>(textblock);
        (*tb)->Margin(Thickness{ left, top, right, bottom });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textblock_set_margin");
        return -1;
    }
}

// TextBox styling
int xaml_textbox_set_background(XamlTextBoxHandle textbox, unsigned int color) {
    if (!textbox) {
        set_last_error(L"Invalid textbox handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->Background(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_background");
        return -1;
    }
}

int xaml_textbox_set_foreground(XamlTextBoxHandle textbox, unsigned int color) {
    if (!textbox) {
        set_last_error(L"Invalid textbox handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->Foreground(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_foreground");
        return -1;
    }
}

int xaml_textbox_set_corner_radius(XamlTextBoxHandle textbox, double radius) {
    if (!textbox) {
        set_last_error(L"Invalid textbox handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->CornerRadius(CornerRadius{ radius, radius, radius, radius });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_corner_radius");
        return -1;
    }
}

int xaml_textbox_set_padding(XamlTextBoxHandle textbox, double left, double top, double right, double bottom) {
    if (!textbox) {
        set_last_error(L"Invalid textbox handle");
        return -1;
    }

    try {
        auto* tb = reinterpret_cast<std::shared_ptr<TextBox>*>(textbox);
        (*tb)->Padding(Thickness{ left, top, right, bottom });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_textbox_set_padding");
        return -1;
    }
}

// StackPanel styling
int xaml_stackpanel_set_background(XamlStackPanelHandle panel, unsigned int color) {
    if (!panel) {
        set_last_error(L"Invalid panel handle");
        return -1;
    }

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        (*sp)->Background(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_set_background");
        return -1;
    }
}

int xaml_stackpanel_set_padding(XamlStackPanelHandle panel, double left, double top, double right, double bottom) {
    if (!panel) {
        set_last_error(L"Invalid panel handle");
        return -1;
    }

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        (*sp)->Padding(Thickness{ left, top, right, bottom });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_set_padding");
        return -1;
    }
}

int xaml_stackpanel_set_corner_radius(XamlStackPanelHandle panel, double radius) {
    if (!panel) {
        set_last_error(L"Invalid panel handle");
        return -1;
    }

    try {
        auto* sp = reinterpret_cast<std::shared_ptr<StackPanel>*>(panel);
        (*sp)->CornerRadius(CornerRadius{ radius, radius, radius, radius });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_stackpanel_set_corner_radius");
        return -1;
    }
}

// Grid styling
int xaml_grid_set_background(XamlGridHandle grid, unsigned int color) {
    if (!grid) {
        set_last_error(L"Invalid grid handle");
        return -1;
    }

    try {
        auto* g = reinterpret_cast<std::shared_ptr<Grid>*>(grid);
        (*g)->Background(create_solid_brush(color));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_grid_set_background");
        return -1;
    }
}

int xaml_grid_set_padding(XamlGridHandle grid, double left, double top, double right, double bottom) {
    if (!grid) {
        set_last_error(L"Invalid grid handle");
        return -1;
    }

    try {
        auto* g = reinterpret_cast<std::shared_ptr<Grid>*>(grid);
        (*g)->Padding(Thickness{ left, top, right, bottom });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_grid_set_padding");
        return -1;
    }
}

int xaml_grid_set_corner_radius(XamlGridHandle grid, double radius) {
    if (!grid) {
        set_last_error(L"Invalid grid handle");
        return -1;
    }

    try {
        auto* g = reinterpret_cast<std::shared_ptr<Grid>*>(grid);
        (*g)->CornerRadius(CornerRadius{ radius, radius, radius, radius });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_grid_set_corner_radius");
        return -1;
    }
}


// ===== CheckBox Implementation =====

XamlCheckBoxHandle xaml_checkbox_create() {
    try {
        auto* checkbox = new std::shared_ptr<CheckBox>(std::make_shared<CheckBox>());
        return checkbox;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return nullptr;
    }
}

int xaml_checkbox_set_content(XamlCheckBoxHandle handle, const wchar_t* content) {
    if (!handle || !content) return -1;
    try {
        auto* checkbox = reinterpret_cast<std::shared_ptr<CheckBox>*>(handle);
        (*checkbox)->Content(box_value(content));
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_checkbox_set_is_checked(XamlCheckBoxHandle handle, bool is_checked) {
    if (!handle) return -1;
    try {
        auto* checkbox = reinterpret_cast<std::shared_ptr<CheckBox>*>(handle);
        (*checkbox)->IsChecked(is_checked);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

bool xaml_checkbox_get_is_checked(XamlCheckBoxHandle handle) {
    if (!handle) return false;
    try {
        auto* checkbox = reinterpret_cast<std::shared_ptr<CheckBox>*>(handle);
        auto checked = (*checkbox)->IsChecked();
        return checked ? checked.Value() : false;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return false;
    }
}

// ===== ComboBox Implementation =====

XamlComboBoxHandle xaml_combobox_create() {
    try {
        auto* combobox = new std::shared_ptr<ComboBox>(std::make_shared<ComboBox>());
        return combobox;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return nullptr;
    }
}

int xaml_combobox_add_item(XamlComboBoxHandle handle, const wchar_t* item) {
    if (!handle || !item) return -1;
    try {
        auto* combobox = reinterpret_cast<std::shared_ptr<ComboBox>*>(handle);
        auto combobox_item = ComboBoxItem();
        combobox_item.Content(box_value(item));
        (*combobox)->Items().Append(combobox_item);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_combobox_set_selected_index(XamlComboBoxHandle handle, int index) {
    if (!handle) return -1;
    try {
        auto* combobox = reinterpret_cast<std::shared_ptr<ComboBox>*>(handle);
        (*combobox)->SelectedIndex(index);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_combobox_get_selected_index(XamlComboBoxHandle handle) {
    if (!handle) return -1;
    try {
        auto* combobox = reinterpret_cast<std::shared_ptr<ComboBox>*>(handle);
        return (*combobox)->SelectedIndex();
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

// ===== Slider Implementation =====

XamlSliderHandle xaml_slider_create() {
    try {
        auto* slider = new std::shared_ptr<Slider>(std::make_shared<Slider>());
        return slider;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return nullptr;
    }
}

int xaml_slider_set_minimum(XamlSliderHandle handle, double minimum) {
    if (!handle) return -1;
    try {
        auto* slider = reinterpret_cast<std::shared_ptr<Slider>*>(handle);
        (*slider)->Minimum(minimum);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_slider_set_maximum(XamlSliderHandle handle, double maximum) {
    if (!handle) return -1;
    try {
        auto* slider = reinterpret_cast<std::shared_ptr<Slider>*>(handle);
        (*slider)->Maximum(maximum);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_slider_set_value(XamlSliderHandle handle, double value) {
    if (!handle) return -1;
    try {
        auto* slider = reinterpret_cast<std::shared_ptr<Slider>*>(handle);
        (*slider)->Value(value);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

double xaml_slider_get_value(XamlSliderHandle handle) {
    if (!handle) return 0.0;
    try {
        auto* slider = reinterpret_cast<std::shared_ptr<Slider>*>(handle);
        return (*slider)->Value();
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return 0.0;
    }
}

// ===== ProgressBar Implementation =====

XamlProgressBarHandle xaml_progressbar_create() {
    try {
        auto* progressbar = new std::shared_ptr<ProgressBar>(std::make_shared<ProgressBar>());
        return progressbar;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return nullptr;
    }
}

int xaml_progressbar_set_minimum(XamlProgressBarHandle handle, double minimum) {
    if (!handle) return -1;
    try {
        auto* progressbar = reinterpret_cast<std::shared_ptr<ProgressBar>*>(handle);
        (*progressbar)->Minimum(minimum);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_progressbar_set_maximum(XamlProgressBarHandle handle, double maximum) {
    if (!handle) return -1;
    try {
        auto* progressbar = reinterpret_cast<std::shared_ptr<ProgressBar>*>(handle);
        (*progressbar)->Maximum(maximum);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_progressbar_set_value(XamlProgressBarHandle handle, double value) {
    if (!handle) return -1;
    try {
        auto* progressbar = reinterpret_cast<std::shared_ptr<ProgressBar>*>(handle);
        (*progressbar)->Value(value);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

int xaml_progressbar_set_is_indeterminate(XamlProgressBarHandle handle, bool is_indeterminate) {
    if (!handle) return -1;
    try {
        auto* progressbar = reinterpret_cast<std::shared_ptr<ProgressBar>*>(handle);
        (*progressbar)->IsIndeterminate(is_indeterminate);
        return 0;
    }
    catch (const hresult_error& ex) {
        set_last_error(ex.message().c_str());
        return -1;
    }
}

// ===== Type Conversion for New Controls =====

XamlUIElementHandle xaml_checkbox_as_uielement(XamlCheckBoxHandle checkbox) {
    return reinterpret_cast<XamlUIElementHandle>(checkbox);
}

XamlUIElementHandle xaml_combobox_as_uielement(XamlComboBoxHandle combobox) {
    return reinterpret_cast<XamlUIElementHandle>(combobox);
}

XamlUIElementHandle xaml_slider_as_uielement(XamlSliderHandle slider) {
    return reinterpret_cast<XamlUIElementHandle>(slider);
}

XamlUIElementHandle xaml_progressbar_as_uielement(XamlProgressBarHandle progressbar) {
    return reinterpret_cast<XamlUIElementHandle>(progressbar);
}

// ============================================================================
// Resource Dictionary Implementation
// ============================================================================

XamlResourceDictionaryHandle xaml_resource_dictionary_create() {
    try {
        auto dict = ResourceDictionary();
        auto* handle = new std::shared_ptr<ResourceDictionary>(
            std::make_shared<ResourceDictionary>(dict)
        );
        return reinterpret_cast<XamlResourceDictionaryHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_create");
        return nullptr;
    }
}

void xaml_resource_dictionary_destroy(XamlResourceDictionaryHandle dict) {
    if (dict) {
        auto* ptr = reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        delete ptr;
    }
}

int xaml_resource_dictionary_insert_color(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key,
    unsigned int color
) {
    if (!dict || !key) {
        set_last_error(L"Invalid handle or key");
        return -1;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        
        uint8_t a = (color >> 24) & 0xFF;
        uint8_t r = (color >> 16) & 0xFF;
        uint8_t g = (color >> 8) & 0xFF;
        uint8_t b = color & 0xFF;
        
        auto winrt_color = Color{ a, r, g, b };
        auto brush = SolidColorBrush(winrt_color);
        
        dict_ptr->Insert(box_value(key), brush);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_insert_color");
        return -1;
    }
}

int xaml_resource_dictionary_insert_double(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key,
    double value
) {
    if (!dict || !key) {
        set_last_error(L"Invalid handle or key");
        return -1;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        dict_ptr->Insert(box_value(key), box_value(value));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_insert_double");
        return -1;
    }
}

int xaml_resource_dictionary_insert_string(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key,
    const wchar_t* value
) {
    if (!dict || !key || !value) {
        set_last_error(L"Invalid handle, key, or value");
        return -1;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        dict_ptr->Insert(box_value(key), box_value(hstring(value)));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_insert_string");
        return -1;
    }
}

int xaml_resource_dictionary_has_key(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
) {
    if (!dict || !key) {
        return 0;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        return dict_ptr->HasKey(box_value(key)) ? 1 : 0;
    }
    catch (...) {
        return 0;
    }
}

unsigned int xaml_resource_dictionary_get_color(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
) {
    if (!dict || !key) {
        set_last_error(L"Invalid handle or key");
        return 0;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        auto value = dict_ptr->Lookup(box_value(key));
        auto brush = value.as<SolidColorBrush>();
        auto color = brush.Color();
        
        return (color.A << 24) | (color.R << 16) | (color.G << 8) | color.B;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return 0;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_get_color");
        return 0;
    }
}

double xaml_resource_dictionary_get_double(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
) {
    if (!dict || !key) {
        set_last_error(L"Invalid handle or key");
        return 0.0;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        auto value = dict_ptr->Lookup(box_value(key));
        return unbox_value<double>(value);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return 0.0;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_get_double");
        return 0.0;
    }
}

int xaml_resource_dictionary_remove(
    XamlResourceDictionaryHandle dict,
    const wchar_t* key
) {
    if (!dict || !key) {
        set_last_error(L"Invalid handle or key");
        return -1;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        dict_ptr->Remove(box_value(key));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_resource_dictionary_remove");
        return -1;
    }
}

void xaml_resource_dictionary_clear(XamlResourceDictionaryHandle dict) {
    if (!dict) {
        return;
    }

    try {
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        dict_ptr->Clear();
    }
    catch (...) {
        // Silently ignore errors in clear
    }
}

int xaml_uielement_set_resources(
    XamlUIElementHandle element,
    XamlResourceDictionaryHandle dict
) {
    if (!element || !dict) {
        set_last_error(L"Invalid element or dictionary handle");
        return -1;
    }

    try {
        auto& elem_ptr = *reinterpret_cast<std::shared_ptr<UIElement>*>(element);
        auto& dict_ptr = *reinterpret_cast<std::shared_ptr<ResourceDictionary>*>(dict);
        
        auto frameworkElement = elem_ptr->as<FrameworkElement>();
        frameworkElement.Resources(*dict_ptr);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_uielement_set_resources");
        return -1;
    }
}

// ============================================================================
// Control Template Implementation
// ============================================================================

XamlControlTemplateHandle xaml_control_template_create() {
    try {
        auto template_obj = ControlTemplate();
        auto* handle = new std::shared_ptr<ControlTemplate>(
            std::make_shared<ControlTemplate>(template_obj)
        );
        return reinterpret_cast<XamlControlTemplateHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_control_template_create");
        return nullptr;
    }
}

void xaml_control_template_destroy(XamlControlTemplateHandle template_handle) {
    if (template_handle) {
        auto* ptr = reinterpret_cast<std::shared_ptr<ControlTemplate>*>(template_handle);
        delete ptr;
    }
}

int xaml_control_template_set_content(
    XamlControlTemplateHandle template_handle,
    XamlUIElementHandle content
) {
    if (!template_handle || !content) {
        set_last_error(L"Invalid template or content handle");
        return -1;
    }

    try {
        // Note: Control templates in WinRT require XAML markup or
        // more complex programmatic setup. This is a simplified version.
        set_last_error(L"Control template content setting requires XAML markup");
        return -1;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_control_template_set_content");
        return -1;
    }
}

int xaml_button_set_template(
    XamlButtonHandle button,
    XamlControlTemplateHandle template_handle
) {
    if (!button || !template_handle) {
        set_last_error(L"Invalid button or template handle");
        return -1;
    }

    try {
        auto& btn_ptr = *reinterpret_cast<std::shared_ptr<Button>*>(button);
        auto& tmpl_ptr = *reinterpret_cast<std::shared_ptr<ControlTemplate>*>(template_handle);
        
        btn_ptr->Template(*tmpl_ptr);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_button_set_template");
        return -1;
    }
}

// ============================================================================
// Animation System Implementation
// ============================================================================

XamlStoryboardHandle xaml_storyboard_create() {
    try {
        auto storyboard = Storyboard();
        auto* handle = new std::shared_ptr<Storyboard>(
            std::make_shared<Storyboard>(storyboard)
        );
        return reinterpret_cast<XamlStoryboardHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_create");
        return nullptr;
    }
}

void xaml_storyboard_destroy(XamlStoryboardHandle storyboard) {
    if (storyboard) {
        auto* ptr = reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        delete ptr;
    }
}

int xaml_storyboard_add_animation(
    XamlStoryboardHandle storyboard,
    XamlDoubleAnimationHandle animation
) {
    if (!storyboard || !animation) {
        set_last_error(L"Invalid storyboard or animation handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<DoubleAnimation>*>(animation);
        
        sb_ptr->Children().Append(*anim_ptr);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_add_animation");
        return -1;
    }
}

int xaml_storyboard_add_color_animation(
    XamlStoryboardHandle storyboard,
    XamlColorAnimationHandle animation
) {
    if (!storyboard || !animation) {
        set_last_error(L"Invalid storyboard or color animation handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<ColorAnimation>*>(animation);
        
        sb_ptr->Children().Append(*anim_ptr);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_add_color_animation");
        return -1;
    }
}

int xaml_storyboard_begin(XamlStoryboardHandle storyboard) {
    if (!storyboard) {
        set_last_error(L"Invalid storyboard handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        sb_ptr->Begin();
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_begin");
        return -1;
    }
}

int xaml_storyboard_stop(XamlStoryboardHandle storyboard) {
    if (!storyboard) {
        set_last_error(L"Invalid storyboard handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        sb_ptr->Stop();
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_stop");
        return -1;
    }
}

int xaml_storyboard_pause(XamlStoryboardHandle storyboard) {
    if (!storyboard) {
        set_last_error(L"Invalid storyboard handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        sb_ptr->Pause();
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_pause");
        return -1;
    }
}

int xaml_storyboard_resume(XamlStoryboardHandle storyboard) {
    if (!storyboard) {
        set_last_error(L"Invalid storyboard handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        sb_ptr->Resume();
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_resume");
        return -1;
    }
}

int xaml_storyboard_set_target(
    XamlStoryboardHandle storyboard,
    XamlUIElementHandle target
) {
    if (!storyboard || !target) {
        set_last_error(L"Invalid storyboard or target handle");
        return -1;
    }

    try {
        auto& sb_ptr = *reinterpret_cast<std::shared_ptr<Storyboard>*>(storyboard);
        auto& target_ptr = *reinterpret_cast<std::shared_ptr<UIElement>*>(target);
        
        // Set target for all animations in the storyboard
        for (auto& timeline : sb_ptr->Children()) {
            Storyboard::SetTarget(timeline, *target_ptr);
        }
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_storyboard_set_target");
        return -1;
    }
}

XamlDoubleAnimationHandle xaml_double_animation_create() {
    try {
        auto animation = DoubleAnimation();
        auto* handle = new std::shared_ptr<DoubleAnimation>(
            std::make_shared<DoubleAnimation>(animation)
        );
        return reinterpret_cast<XamlDoubleAnimationHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_double_animation_create");
        return nullptr;
    }
}

void xaml_double_animation_destroy(XamlDoubleAnimationHandle animation) {
    if (animation) {
        auto* ptr = reinterpret_cast<std::shared_ptr<DoubleAnimation>*>(animation);
        delete ptr;
    }
}

int xaml_double_animation_set_from(XamlDoubleAnimationHandle animation, double from) {
    if (!animation) {
        set_last_error(L"Invalid animation handle");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<DoubleAnimation>*>(animation);
        anim_ptr->From(from);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_double_animation_set_from");
        return -1;
    }
}

int xaml_double_animation_set_to(XamlDoubleAnimationHandle animation, double to) {
    if (!animation) {
        set_last_error(L"Invalid animation handle");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<DoubleAnimation>*>(animation);
        anim_ptr->To(to);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_double_animation_set_to");
        return -1;
    }
}

int xaml_double_animation_set_duration(XamlDoubleAnimationHandle animation, int milliseconds) {
    if (!animation) {
        set_last_error(L"Invalid animation handle");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<DoubleAnimation>*>(animation);
        Duration duration;
        duration.TimeSpan = TimeSpan(std::chrono::milliseconds(milliseconds));
        anim_ptr->Duration(duration);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_double_animation_set_duration");
        return -1;
    }
}

int xaml_double_animation_set_target_property(
    XamlDoubleAnimationHandle animation,
    XamlUIElementHandle target,
    const wchar_t* property_path
) {
    if (!animation || !target || !property_path) {
        set_last_error(L"Invalid animation, target, or property path");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<DoubleAnimation>*>(animation);
        auto& target_ptr = *reinterpret_cast<std::shared_ptr<UIElement>*>(target);
        
        Storyboard::SetTarget(*anim_ptr, *target_ptr);
        Storyboard::SetTargetProperty(*anim_ptr, hstring(property_path));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_double_animation_set_target_property");
        return -1;
    }
}

XamlColorAnimationHandle xaml_color_animation_create() {
    try {
        auto animation = ColorAnimation();
        auto* handle = new std::shared_ptr<ColorAnimation>(
            std::make_shared<ColorAnimation>(animation)
        );
        return reinterpret_cast<XamlColorAnimationHandle>(handle);
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return nullptr;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_color_animation_create");
        return nullptr;
    }
}

void xaml_color_animation_destroy(XamlColorAnimationHandle animation) {
    if (animation) {
        auto* ptr = reinterpret_cast<std::shared_ptr<ColorAnimation>*>(animation);
        delete ptr;
    }
}

int xaml_color_animation_set_from(XamlColorAnimationHandle animation, unsigned int from) {
    if (!animation) {
        set_last_error(L"Invalid animation handle");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<ColorAnimation>*>(animation);
        
        uint8_t a = (from >> 24) & 0xFF;
        uint8_t r = (from >> 16) & 0xFF;
        uint8_t g = (from >> 8) & 0xFF;
        uint8_t b = from & 0xFF;
        
        anim_ptr->From(Color{ a, r, g, b });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_color_animation_set_from");
        return -1;
    }
}

int xaml_color_animation_set_to(XamlColorAnimationHandle animation, unsigned int to) {
    if (!animation) {
        set_last_error(L"Invalid animation handle");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<ColorAnimation>*>(animation);
        
        uint8_t a = (to >> 24) & 0xFF;
        uint8_t r = (to >> 16) & 0xFF;
        uint8_t g = (to >> 8) & 0xFF;
        uint8_t b = to & 0xFF;
        
        anim_ptr->To(Color{ a, r, g, b });
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_color_animation_set_to");
        return -1;
    }
}

int xaml_color_animation_set_duration(XamlColorAnimationHandle animation, int milliseconds) {
    if (!animation) {
        set_last_error(L"Invalid animation handle");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<ColorAnimation>*>(animation);
        Duration duration;
        duration.TimeSpan = TimeSpan(std::chrono::milliseconds(milliseconds));
        anim_ptr->Duration(duration);
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_color_animation_set_duration");
        return -1;
    }
}

int xaml_color_animation_set_target_property(
    XamlColorAnimationHandle animation,
    XamlUIElementHandle target,
    const wchar_t* property_path
) {
    if (!animation || !target || !property_path) {
        set_last_error(L"Invalid animation, target, or property path");
        return -1;
    }

    try {
        auto& anim_ptr = *reinterpret_cast<std::shared_ptr<ColorAnimation>*>(animation);
        auto& target_ptr = *reinterpret_cast<std::shared_ptr<UIElement>*>(target);
        
        Storyboard::SetTarget(*anim_ptr, *target_ptr);
        Storyboard::SetTargetProperty(*anim_ptr, hstring(property_path));
        return 0;
    }
    catch (const hresult_error& e) {
        set_last_error(e.message().c_str());
        return -1;
    }
    catch (...) {
        set_last_error(L"Unknown error in xaml_color_animation_set_target_property");
        return -1;
    }
}


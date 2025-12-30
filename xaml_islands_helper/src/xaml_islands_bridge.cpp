#include "xaml_islands_bridge.h"
#include <winrt/Windows.Foundation.h>
#include <winrt/Windows.Foundation.Collections.h>
#include <winrt/Windows.UI.Xaml.h>
#include <winrt/Windows.UI.Xaml.Controls.h>
#include <winrt/Windows.UI.Xaml.Controls.Primitives.h>
#include <winrt/Windows.UI.Xaml.Hosting.h>
#include <Windows.UI.Xaml.Hosting.DesktopWindowXamlSource.h>
#include <string>
#include <memory>

using namespace winrt;
using namespace Windows::Foundation;
using namespace Windows::UI::Xaml;
using namespace Windows::UI::Xaml::Controls;
using namespace Windows::UI::Xaml::Hosting;

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


#include "xaml_islands_bridge.h"
#include <winrt/Windows.Foundation.h>
#include <winrt/Windows.UI.Xaml.h>
#include <winrt/Windows.UI.Xaml.Controls.h>
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


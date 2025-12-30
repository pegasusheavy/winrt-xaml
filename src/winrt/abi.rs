//! WinRT ABI (Application Binary Interface) types and constants.

use windows::core::GUID;

/// Represents a WinRT interface identifier (IID).
pub type IID = GUID;

/// Represents a WinRT class identifier (CLSID).
pub type CLSID = GUID;

/// Represents a WinRT runtime class name.
pub type RuntimeClassName = *const u16;

/// TrustLevel enumeration for WinRT objects.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    BaseTrust = 0,
    PartialTrust = 1,
    FullTrust = 2,
}

/// Common IIDs for WinRT interfaces.
pub mod iids {
    use super::IID;

    /// IUnknown interface ID
    pub const IID_IUNKNOWN: IID = IID::from_u128(0x00000000_0000_0000_C000_000000000046);

    /// IInspectable interface ID
    pub const IID_IINSPECTABLE: IID = IID::from_u128(0xAF86E2E0_B12D_4c6a_9C5A_D7AA65101E90);

    /// IActivationFactory interface ID
    pub const IID_IACTIVATION_FACTORY: IID = IID::from_u128(0x00000035_0000_0000_C000_000000000046);

    /// Windows.UI.Xaml.Application interface ID (IApplication)
    pub const IID_IAPPLICATION: IID = IID::from_u128(0x74B861A1_7487_46A9_8635_000000000000);

    /// Windows.UI.Xaml.Window interface ID (IWindow)
    pub const IID_IWINDOW: IID = IID::from_u128(0x00000000_0000_0000_0000_000000000000); // Placeholder

    /// Windows.UI.Xaml.UIElement interface ID (IUIElement)
    pub const IID_IUIELEMENT: IID = IID::from_u128(0x00000000_0000_0000_0000_000000000000); // Placeholder
}

/// HRESULT success code.
pub const S_OK: i32 = 0;

/// HRESULT error codes.
pub const E_NOINTERFACE: i32 = 0x80004002_u32 as i32;
pub const E_POINTER: i32 = 0x80004003_u32 as i32;
pub const E_FAIL: i32 = 0x80004005_u32 as i32;
pub const E_NOTIMPL: i32 = 0x80004001_u32 as i32;

/// Helper to check if an HRESULT succeeded.
#[inline]
pub fn succeeded(hr: i32) -> bool {
    hr >= 0
}

/// Helper to check if an HRESULT failed.
#[inline]
pub fn failed(hr: i32) -> bool {
    hr < 0
}


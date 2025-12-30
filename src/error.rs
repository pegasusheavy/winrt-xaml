//! Error types for WinRT-XAML.

use thiserror::Error;

/// Result type alias using our Error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for WinRT-XAML operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Windows API error
    #[error("Windows API error: {0}")]
    WindowsError(String),

    /// Window creation failed
    #[error("Failed to create window: {0}")]
    WindowCreation(String),

    /// Control creation failed
    #[error("Failed to create control: {0}")]
    ControlCreation(String),

    /// XAML parsing error
    #[error("XAML parse error: {0}")]
    XamlParse(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Not implemented
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),

    /// Application error
    #[error("Application error: {0}")]
    Application(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8 conversion error
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a Windows API error.
    pub fn windows(msg: impl Into<String>) -> Self {
        Self::WindowsError(msg.into())
    }

    /// Create a window creation error.
    pub fn window_creation(msg: impl Into<String>) -> Self {
        Self::WindowCreation(msg.into())
    }

    /// Create a control creation error.
    pub fn control_creation(msg: impl Into<String>) -> Self {
        Self::ControlCreation(msg.into())
    }

    /// Create a XAML parse error.
    pub fn xaml_parse(msg: impl Into<String>) -> Self {
        Self::XamlParse(msg.into())
    }

    /// Create a resource not found error.
    pub fn resource_not_found(msg: impl Into<String>) -> Self {
        Self::ResourceNotFound(msg.into())
    }

    /// Create an invalid operation error.
    pub fn invalid_operation(msg: impl Into<String>) -> Self {
        Self::InvalidOperation(msg.into())
    }

    /// Create a not implemented error.
    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented(msg.into())
    }

    /// Create an application error.
    pub fn application(msg: impl Into<String>) -> Self {
        Self::Application(msg.into())
    }
}

impl From<windows::core::Error> for Error {
    fn from(err: windows::core::Error) -> Self {
        Error::WindowsError(format!("{:?}", err))
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

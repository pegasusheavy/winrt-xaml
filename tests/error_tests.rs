//! Unit tests for error handling.

use winrt_xaml::error::{Error, Result};

#[test]
fn test_error_creation() {
    let err = Error::windows("Test error");
    assert_eq!(err.to_string(), "Windows API error: Test error");

    let err = Error::window_creation("Window failed");
    assert_eq!(err.to_string(), "Failed to create window: Window failed");

    let err = Error::control_creation("Control failed");
    assert_eq!(err.to_string(), "Failed to create control: Control failed");
}

#[test]
fn test_error_from_str() {
    let err: Error = "Test error".into();
    assert_eq!(err.to_string(), "Test error");

    let err: Error = String::from("Another error").into();
    assert_eq!(err.to_string(), "Another error");
}

#[test]
fn test_not_implemented_error() {
    let err = Error::not_implemented("Feature X");
    assert_eq!(err.to_string(), "Feature not implemented: Feature X");
}

#[test]
fn test_xaml_parse_error() {
    let err = Error::xaml_parse("Invalid XAML");
    assert_eq!(err.to_string(), "XAML parse error: Invalid XAML");
}

#[test]
fn test_resource_not_found() {
    let err = Error::resource_not_found("MyResource");
    assert_eq!(err.to_string(), "Resource not found: MyResource");
}

#[test]
fn test_result_type() {
    fn returns_ok() -> Result<i32> {
        Ok(42)
    }

    fn returns_err() -> Result<i32> {
        Err(Error::windows("Failed"))
    }

    assert!(returns_ok().is_ok());
    assert_eq!(returns_ok().unwrap(), 42);
    assert!(returns_err().is_err());
}

#[test]
fn test_error_chain() {
    fn inner_function() -> Result<()> {
        Err(Error::windows("Inner error"))
    }

    fn outer_function() -> Result<()> {
        inner_function()?;
        Ok(())
    }

    match outer_function() {
        Err(e) => assert_eq!(e.to_string(), "Windows API error: Inner error"),
        Ok(_) => panic!("Expected error"),
    }
}


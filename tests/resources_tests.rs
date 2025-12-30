//! Unit tests for resource management.

use winrt_xaml::resources::*;

#[test]
fn test_resource_dictionary_creation() {
    let dict = ResourceDictionary::new();
    assert!(!dict.contains_key("test"));
}

#[test]
fn test_resource_insertion_and_retrieval() {
    let dict = ResourceDictionary::new();

    dict.insert("title", ResourceValue::String("Hello".to_string()));
    dict.insert("count", ResourceValue::Integer(42));
    dict.insert("ratio", ResourceValue::Float(3.14));
    dict.insert("enabled", ResourceValue::Boolean(true));

    assert!(dict.contains_key("title"));
    assert!(dict.contains_key("count"));
    assert!(dict.contains_key("ratio"));
    assert!(dict.contains_key("enabled"));

    match dict.get("title") {
        Some(ResourceValue::String(s)) => assert_eq!(s, "Hello"),
        _ => panic!("Expected string value"),
    }

    match dict.get("count") {
        Some(ResourceValue::Integer(i)) => assert_eq!(i, 42),
        _ => panic!("Expected integer value"),
    }
}

#[test]
fn test_resource_removal() {
    let dict = ResourceDictionary::new();

    dict.insert("key1", ResourceValue::String("value1".to_string()));
    assert!(dict.contains_key("key1"));

    let removed = dict.remove("key1");
    assert!(removed.is_some());
    assert!(!dict.contains_key("key1"));
}

#[test]
fn test_resource_clear() {
    let dict = ResourceDictionary::new();

    dict.insert("key1", ResourceValue::Integer(1));
    dict.insert("key2", ResourceValue::Integer(2));
    dict.insert("key3", ResourceValue::Integer(3));

    assert!(dict.contains_key("key1"));
    assert!(dict.contains_key("key2"));
    assert!(dict.contains_key("key3"));

    dict.clear();

    assert!(!dict.contains_key("key1"));
    assert!(!dict.contains_key("key2"));
    assert!(!dict.contains_key("key3"));
}

#[test]
fn test_resource_overwrite() {
    let dict = ResourceDictionary::new();

    dict.insert("key", ResourceValue::Integer(1));
    match dict.get("key") {
        Some(ResourceValue::Integer(i)) => assert_eq!(i, 1),
        _ => panic!("Expected integer value"),
    }

    dict.insert("key", ResourceValue::Integer(2));
    match dict.get("key") {
        Some(ResourceValue::Integer(i)) => assert_eq!(i, 2),
        _ => panic!("Expected integer value"),
    }
}

#[test]
fn test_resource_dictionary_clone() {
    let dict1 = ResourceDictionary::new();
    dict1.insert("key", ResourceValue::String("value".to_string()));

    let dict2 = dict1.clone();
    assert!(dict2.contains_key("key"));
}

#[test]
fn test_resource_value_clone() {
    let value1 = ResourceValue::String("test".to_string());
    let value2 = value1.clone();

    match (value1, value2) {
        (ResourceValue::String(s1), ResourceValue::String(s2)) => assert_eq!(s1, s2),
        _ => panic!("Values should match"),
    }
}

#[test]
fn test_missing_resource() {
    let dict = ResourceDictionary::new();
    assert!(dict.get("nonexistent").is_none());
}


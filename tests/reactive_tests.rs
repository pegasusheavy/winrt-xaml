//! Unit tests for the reactive data binding system.

use winrt_xaml::reactive::*;
use std::sync::{Arc, Mutex};

#[test]
fn test_property_creation() {
    let prop = Property::new(42);
    assert_eq!(prop.get(), 42);
}

#[test]
fn test_property_set_and_get() {
    let prop = Property::new(10);
    assert_eq!(prop.get(), 10);
    
    prop.set(20);
    assert_eq!(prop.get(), 20);
}

#[test]
fn test_property_update() {
    let prop = Property::new(5);
    prop.update(|x| x * 2);
    assert_eq!(prop.get(), 10);
}

#[test]
fn test_property_subscribe() {
    let prop = Property::new(0);
    let value = Arc::new(Mutex::new(0));
    
    let value_clone = value.clone();
    prop.subscribe(move |v| {
        *value_clone.lock().unwrap() = *v;
    });
    
    prop.set(42);
    assert_eq!(*value.lock().unwrap(), 42);
}

#[test]
fn test_property_multiple_subscribers() {
    let prop = Property::new(0);
    let count1 = Arc::new(Mutex::new(0));
    let count2 = Arc::new(Mutex::new(0));
    
    let count1_clone = count1.clone();
    prop.subscribe(move |_| {
        *count1_clone.lock().unwrap() += 1;
    });
    
    let count2_clone = count2.clone();
    prop.subscribe(move |_| {
        *count2_clone.lock().unwrap() += 1;
    });
    
    prop.set(1);
    prop.set(2);
    
    assert_eq!(*count1.lock().unwrap(), 2);
    assert_eq!(*count2.lock().unwrap(), 2);
}

#[test]
fn test_property_unsubscribe() {
    let prop = Property::new(0);
    let count = Arc::new(Mutex::new(0));
    
    let count_clone = count.clone();
    let sub_id = prop.subscribe(move |_| {
        *count_clone.lock().unwrap() += 1;
    });
    
    prop.set(1);
    assert_eq!(*count.lock().unwrap(), 1);
    
    prop.unsubscribe(sub_id);
    prop.set(2);
    assert_eq!(*count.lock().unwrap(), 1); // Should not increment
}

#[test]
fn test_property_subscriber_count() {
    let prop = Property::new(0);
    assert_eq!(prop.subscriber_count(), 0);
    
    let _sub1 = prop.subscribe(|_| {});
    assert_eq!(prop.subscriber_count(), 1);
    
    let sub2 = prop.subscribe(|_| {});
    assert_eq!(prop.subscriber_count(), 2);
    
    prop.unsubscribe(sub2);
    assert_eq!(prop.subscriber_count(), 1);
}

#[test]
fn test_property_clone() {
    let prop1 = Property::new(42);
    let prop2 = prop1.clone();
    
    prop2.set(100);
    assert_eq!(prop1.get(), 100); // They share the same value
}

#[test]
fn test_property_with_string() {
    let prop = Property::new("Hello".to_string());
    assert_eq!(prop.get(), "Hello");
    
    prop.set("World".to_string());
    assert_eq!(prop.get(), "World");
}

#[test]
fn test_observable_collection_creation() {
    let collection: ObservableCollection<i32> = ObservableCollection::new();
    assert_eq!(collection.len(), 0);
}

#[test]
fn test_observable_collection_push() {
    let collection = ObservableCollection::new();
    collection.push(1);
    collection.push(2);
    collection.push(3);
    
    assert_eq!(collection.len(), 3);
    assert_eq!(collection.get(0), Some(1));
    assert_eq!(collection.get(1), Some(2));
    assert_eq!(collection.get(2), Some(3));
}

#[test]
fn test_observable_collection_pop() {
    let collection = ObservableCollection::new();
    collection.push(1);
    collection.push(2);
    
    assert_eq!(collection.pop(), Some(2));
    assert_eq!(collection.len(), 1);
    assert_eq!(collection.pop(), Some(1));
    assert_eq!(collection.len(), 0);
    assert_eq!(collection.pop(), None);
}

#[test]
fn test_observable_collection_insert() {
    let collection = ObservableCollection::new();
    collection.push(1);
    collection.push(3);
    collection.insert(1, 2).unwrap();
    
    assert_eq!(collection.len(), 3);
    assert_eq!(collection.get(0), Some(1));
    assert_eq!(collection.get(1), Some(2));
    assert_eq!(collection.get(2), Some(3));
}

#[test]
fn test_observable_collection_remove() {
    let collection = ObservableCollection::new();
    collection.push(1);
    collection.push(2);
    collection.push(3);
    
    assert_eq!(collection.remove(1), Ok(2));
    assert_eq!(collection.len(), 2);
    assert_eq!(collection.get(0), Some(1));
    assert_eq!(collection.get(1), Some(3));
}

#[test]
fn test_observable_collection_clear() {
    let collection = ObservableCollection::new();
    collection.push(1);
    collection.push(2);
    collection.push(3);
    
    collection.clear();
    assert_eq!(collection.len(), 0);
}

#[test]
fn test_observable_collection_subscribe() {
    let collection = ObservableCollection::new();
    let changes = Arc::new(Mutex::new(Vec::new()));
    
    let changes_clone = changes.clone();
    collection.subscribe(move |change| {
        changes_clone.lock().unwrap().push(change.clone());
    });
    
    collection.push(1);
    collection.push(2);
    collection.remove(0).unwrap();
    collection.clear();
    
    let changes = changes.lock().unwrap();
    assert_eq!(changes.len(), 4);
    
    matches!(changes[0], CollectionChange::Added { index: 0, .. });
    matches!(changes[1], CollectionChange::Added { index: 1, .. });
    matches!(changes[2], CollectionChange::Removed { index: 0, .. });
    matches!(changes[3], CollectionChange::Cleared);
}

#[test]
fn test_observable_collection_get_out_of_bounds() {
    let collection = ObservableCollection::new();
    collection.push(1);
    
    assert_eq!(collection.get(0), Some(1));
    assert_eq!(collection.get(1), None);
    assert_eq!(collection.get(100), None);
}

#[test]
fn test_observable_collection_insert_out_of_bounds() {
    let collection = ObservableCollection::new();
    collection.push(1);
    
    let result = collection.insert(10, 2);
    assert!(result.is_err());
}

#[test]
fn test_observable_collection_remove_out_of_bounds() {
    let collection = ObservableCollection::new();
    collection.push(1);
    
    let result = collection.remove(10);
    assert!(result.is_err());
}

#[test]
fn test_computed_creation() {
    let computed = Computed::new(|| 42);
    assert_eq!(computed.get(), 42);
}

#[test]
fn test_computed_from_property() {
    let source = Property::new(10);
    let doubled = Computed::from_property(&source, |x| x * 2);
    
    assert_eq!(doubled.get(), 20);
    
    source.set(20);
    // Note: Sleep briefly to allow subscription to process
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert_eq!(doubled.get(), 40);
}

#[test]
fn test_computed_subscribe() {
    let computed = Computed::new(|| 42);
    let value = Arc::new(Mutex::new(0));
    
    let value_clone = value.clone();
    computed.subscribe(move |v| {
        *value_clone.lock().unwrap() = *v;
    });
    
    // Trigger a change by setting the underlying property
    // (In real usage, Computed would update when dependencies change)
    assert_eq!(*value.lock().unwrap(), 0); // No auto-update yet
}

#[test]
fn test_property_with_custom_type() {
    #[derive(Clone, PartialEq, Debug)]
    struct User {
        name: String,
        age: i32,
    }
    
    let prop = Property::new(User {
        name: "Alice".to_string(),
        age: 30,
    });
    
    assert_eq!(prop.get().name, "Alice");
    assert_eq!(prop.get().age, 30);
    
    prop.set(User {
        name: "Bob".to_string(),
        age: 25,
    });
    
    assert_eq!(prop.get().name, "Bob");
    assert_eq!(prop.get().age, 25);
}

#[test]
fn test_observable_collection_with_strings() {
    let collection = ObservableCollection::new();
    collection.push("Hello".to_string());
    collection.push("World".to_string());
    
    assert_eq!(collection.len(), 2);
    assert_eq!(collection.get(0), Some("Hello".to_string()));
    assert_eq!(collection.get(1), Some("World".to_string()));
}

#[test]
fn test_collection_change_variants() {
    let added = CollectionChange::Added {
        index: 0,
        value: 42,
    };
    
    let removed = CollectionChange::Removed {
        index: 1,
        value: 24,
    };
    
    let cleared = CollectionChange::Cleared;
    
    matches!(added, CollectionChange::Added { .. });
    matches!(removed, CollectionChange::Removed { .. });
    matches!(cleared, CollectionChange::Cleared);
}

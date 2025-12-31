//! Observable collection implementation.

use super::{Subscriber, SubscriptionId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents a change to an observable collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionChange<T> {
    /// An item was added at the specified index.
    Added { index: usize, item: T },
    /// An item was removed from the specified index.
    Removed { index: usize, item: T },
    /// An item at the specified index was replaced.
    Replaced {
        index: usize,
        old_item: T,
        new_item: T,
    },
    /// The entire collection was cleared.
    Cleared,
    /// The collection was reset with new items.
    Reset { items: Vec<T> },
}

/// An observable collection that notifies subscribers when items are added, removed, or changed.
///
/// `ObservableCollection<T>` provides a reactive wrapper around `Vec<T>`, notifying
/// subscribers whenever the collection changes.
///
/// # Example
///
/// ```rust
/// use winrt_xaml::reactive::{ObservableCollection, CollectionChange};
///
/// let todos = ObservableCollection::new();
///
/// // Subscribe to changes
/// todos.subscribe(|change| {
///     match change {
///         CollectionChange::Added { index, item } => {
///             println!("Added '{}' at index {}", item, index);
///         }
///         CollectionChange::Removed { index, item } => {
///             println!("Removed '{}' from index {}", item, index);
///         }
///         _ => {}
///     }
/// });
///
/// // Add items (triggers subscriber)
/// todos.push("Buy milk".to_string());
/// todos.push("Write code".to_string());
/// ```
pub struct ObservableCollection<T> {
    inner: Arc<Mutex<CollectionInner<T>>>,
}

struct CollectionInner<T> {
    items: Vec<T>,
    subscribers: HashMap<usize, Subscriber<CollectionChange<T>>>,
    next_id: usize,
}

impl<T: Clone> ObservableCollection<T> {
    /// Create a new empty observable collection.
    pub fn new() -> Self {
        ObservableCollection {
            inner: Arc::new(Mutex::new(CollectionInner {
                items: Vec::new(),
                subscribers: HashMap::new(),
                next_id: 0,
            })),
        }
    }

    /// Create a new observable collection from existing items.
    pub fn from_vec(items: Vec<T>) -> Self {
        ObservableCollection {
            inner: Arc::new(Mutex::new(CollectionInner {
                items,
                subscribers: HashMap::new(),
                next_id: 0,
            })),
        }
    }

    /// Get the number of items in the collection.
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().items.len()
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.lock().unwrap().items.is_empty()
    }

    /// Get a clone of the item at the specified index.
    pub fn get(&self, index: usize) -> Option<T> {
        self.inner.lock().unwrap().items.get(index).cloned()
    }

    /// Get a clone of all items.
    pub fn get_all(&self) -> Vec<T> {
        self.inner.lock().unwrap().items.clone()
    }

    /// Execute a closure with a reference to the items (without cloning).
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[T]) -> R,
    {
        let inner = self.inner.lock().unwrap();
        f(&inner.items)
    }

    /// Push an item to the end of the collection.
    pub fn push(&self, item: T) {
        let mut inner = self.inner.lock().unwrap();
        let index = inner.items.len();
        inner.items.push(item.clone());

        Self::notify_subscribers(
            &inner.subscribers,
            &CollectionChange::Added { index, item },
        );
    }

    /// Pop an item from the end of the collection.
    pub fn pop(&self) -> Option<T> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(item) = inner.items.pop() {
            let index = inner.items.len(); // index before pop
            Self::notify_subscribers(
                &inner.subscribers,
                &CollectionChange::Removed {
                    index,
                    item: item.clone(),
                },
            );
            Some(item)
        } else {
            None
        }
    }

    /// Insert an item at the specified index.
    pub fn insert(&self, index: usize, item: T) {
        let mut inner = self.inner.lock().unwrap();
        if index <= inner.items.len() {
            inner.items.insert(index, item.clone());
            Self::notify_subscribers(
                &inner.subscribers,
                &CollectionChange::Added { index, item },
            );
        }
    }

    /// Remove an item at the specified index.
    pub fn remove(&self, index: usize) -> Option<T> {
        let mut inner = self.inner.lock().unwrap();
        if index < inner.items.len() {
            let item = inner.items.remove(index);
            Self::notify_subscribers(
                &inner.subscribers,
                &CollectionChange::Removed {
                    index,
                    item: item.clone(),
                },
            );
            Some(item)
        } else {
            None
        }
    }

    /// Replace an item at the specified index.
    pub fn replace(&self, index: usize, new_item: T) -> Option<T> {
        let mut inner = self.inner.lock().unwrap();
        if index < inner.items.len() {
            let old_item = std::mem::replace(&mut inner.items[index], new_item.clone());
            Self::notify_subscribers(
                &inner.subscribers,
                &CollectionChange::Replaced {
                    index,
                    old_item: old_item.clone(),
                    new_item,
                },
            );
            Some(old_item)
        } else {
            None
        }
    }

    /// Clear all items from the collection.
    pub fn clear(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.items.clear();
        Self::notify_subscribers(&inner.subscribers, &CollectionChange::Cleared);
    }

    /// Reset the collection with new items.
    pub fn reset(&self, items: Vec<T>) {
        let mut inner = self.inner.lock().unwrap();
        inner.items = items.clone();
        Self::notify_subscribers(
            &inner.subscribers,
            &CollectionChange::Reset { items },
        );
    }

    /// Subscribe to collection changes. Returns a subscription ID.
    ///
    /// The subscriber will be called for each change to the collection.
    pub fn subscribe<F>(&self, f: F) -> SubscriptionId
    where
        F: FnMut(&CollectionChange<T>) + Send + 'static,
    {
        let mut inner = self.inner.lock().unwrap();
        let id = inner.next_id;
        inner.next_id += 1;

        let subscriber = Arc::new(Mutex::new(f));

        // Optionally notify subscriber of initial state
        // (commented out to avoid immediate callback)
        // if !inner.items.is_empty() {
        //     if let Ok(mut sub) = subscriber.lock() {
        //         sub(&CollectionChange::Reset { items: inner.items.clone() });
        //     }
        // }

        inner.subscribers.insert(id, subscriber);
        SubscriptionId::new(id)
    }

    /// Unsubscribe a subscriber by ID.
    pub fn unsubscribe(&self, id: SubscriptionId) {
        let mut inner = self.inner.lock().unwrap();
        inner.subscribers.remove(&id.0);
    }

    /// Get the number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.inner.lock().unwrap().subscribers.len()
    }

    fn notify_subscribers(
        subscribers: &HashMap<usize, Subscriber<CollectionChange<T>>>,
        change: &CollectionChange<T>,
    ) {
        for subscriber in subscribers.values() {
            if let Ok(mut sub) = subscriber.lock() {
                sub(change);
            }
        }
    }
}

impl<T: Clone> Clone for ObservableCollection<T> {
    fn clone(&self) -> Self {
        ObservableCollection {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone> Default for ObservableCollection<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_push_pop() {
        let col = ObservableCollection::new();
        col.push(1);
        col.push(2);
        col.push(3);

        assert_eq!(col.len(), 3);
        assert_eq!(col.pop(), Some(3));
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_collection_subscribe() {
        let col = ObservableCollection::new();
        let changes = Arc::new(Mutex::new(Vec::new()));

        col.subscribe({
            let changes = changes.clone();
            move |change| {
                changes.lock().unwrap().push(change.clone());
            }
        });

        col.push("A".to_string());
        col.push("B".to_string());
        col.remove(0);

        let recorded = changes.lock().unwrap();
        assert_eq!(recorded.len(), 3);
        assert!(matches!(recorded[0], CollectionChange::Added { .. }));
        assert!(matches!(recorded[1], CollectionChange::Added { .. }));
        assert!(matches!(recorded[2], CollectionChange::Removed { .. }));
    }

    #[test]
    fn test_collection_clear() {
        let col = ObservableCollection::new();
        col.push(1);
        col.push(2);
        col.push(3);

        assert_eq!(col.len(), 3);
        col.clear();
        assert_eq!(col.len(), 0);
        assert!(col.is_empty());
    }
}

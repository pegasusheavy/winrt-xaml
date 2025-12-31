//! Observable property implementation.

use super::{Subscriber, SubscriptionId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// An observable property that notifies subscribers when its value changes.
///
/// `Property<T>` is the core building block for reactive state management.
/// It holds a value and maintains a list of subscribers that are notified
/// whenever the value changes.
///
/// # Thread Safety
///
/// `Property<T>` is thread-safe and can be shared across threads using `Clone`.
/// All operations are protected by internal mutexes.
///
/// # Example
///
/// ```rust
/// use winrt_xaml::reactive::Property;
///
/// let name = Property::new("Alice".to_string());
///
/// // Subscribe to changes
/// name.subscribe(|value| {
///     println!("Name changed to: {}", value);
/// });
///
/// // Update value (triggers subscriber)
/// name.set("Bob".to_string());
/// ```
pub struct Property<T> {
    inner: Arc<Mutex<PropertyInner<T>>>,
}

struct PropertyInner<T> {
    value: T,
    subscribers: HashMap<usize, Subscriber<T>>,
    next_id: usize,
}

impl<T: Clone> Property<T> {
    /// Create a new property with an initial value.
    pub fn new(value: T) -> Self {
        Property {
            inner: Arc::new(Mutex::new(PropertyInner {
                value,
                subscribers: HashMap::new(),
                next_id: 0,
            })),
        }
    }

    /// Get the current value (clones the value).
    pub fn get(&self) -> T {
        self.inner.lock().unwrap().value.clone()
    }

    /// Get a reference to the current value without cloning.
    ///
    /// The provided closure receives a reference to the value while the lock is held.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let inner = self.inner.lock().unwrap();
        f(&inner.value)
    }

    /// Set a new value, notifying all subscribers if the value changed.
    pub fn set(&self, new_value: T)
    where
        T: PartialEq,
    {
        let mut inner = self.inner.lock().unwrap();
        if inner.value != new_value {
            inner.value = new_value;
            Self::notify_subscribers(&inner.subscribers, &inner.value);
        }
    }

    /// Set a new value without checking for equality, always notifying subscribers.
    pub fn set_always(&self, new_value: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.value = new_value;
        Self::notify_subscribers(&inner.subscribers, &inner.value);
    }

    /// Update the value using a closure, notifying subscribers if the value changed.
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
        T: PartialEq,
    {
        let mut inner = self.inner.lock().unwrap();
        let old_value = inner.value.clone();
        f(&mut inner.value);
        if inner.value != old_value {
            Self::notify_subscribers(&inner.subscribers, &inner.value);
        }
    }

    /// Update the value using a closure, always notifying subscribers.
    pub fn update_always<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut inner = self.inner.lock().unwrap();
        f(&mut inner.value);
        Self::notify_subscribers(&inner.subscribers, &inner.value);
    }

    /// Subscribe to value changes. Returns a subscription ID that can be used to unsubscribe.
    ///
    /// The subscriber callback will be called immediately with the current value,
    /// and then again whenever the value changes.
    pub fn subscribe<F>(&self, f: F) -> SubscriptionId
    where
        F: FnMut(&T) + Send + 'static,
    {
        let mut inner = self.inner.lock().unwrap();
        let id = inner.next_id;
        inner.next_id += 1;

        let subscriber = Arc::new(Mutex::new(f));

        // Call subscriber immediately with current value
        if let Ok(mut sub) = subscriber.lock() {
            sub(&inner.value);
        }

        inner.subscribers.insert(id, subscriber);
        SubscriptionId::new(id)
    }

    /// Subscribe to value changes without receiving the initial value.
    pub fn subscribe_silent<F>(&self, f: F) -> SubscriptionId
    where
        F: FnMut(&T) + Send + 'static,
    {
        let mut inner = self.inner.lock().unwrap();
        let id = inner.next_id;
        inner.next_id += 1;

        let subscriber = Arc::new(Mutex::new(f));
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

    fn notify_subscribers(subscribers: &HashMap<usize, Subscriber<T>>, value: &T) {
        for subscriber in subscribers.values() {
            if let Ok(mut sub) = subscriber.lock() {
                sub(value);
            }
        }
    }

}

impl<T: Clone> Clone for Property<T> {
    fn clone(&self) -> Self {
        Property {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone + Default> Default for Property<T> {
    fn default() -> Self {
        Property::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_get_set() {
        let prop = Property::new(42);
        assert_eq!(prop.get(), 42);

        prop.set(100);
        assert_eq!(prop.get(), 100);
    }

    #[test]
    fn test_property_subscribe() {
        let prop = Property::new(0);
        let received = Arc::new(Mutex::new(Vec::new()));

        prop.subscribe({
            let received = received.clone();
            move |value| {
                received.lock().unwrap().push(*value);
            }
        });

        prop.set(1);
        prop.set(2);
        prop.set(3);

        let values = received.lock().unwrap();
        assert_eq!(*values, vec![0, 1, 2, 3]); // 0 is initial value
    }

    #[test]
    fn test_property_update() {
        let prop = Property::new(10);
        prop.update(|v| *v += 5);
        assert_eq!(prop.get(), 15);
    }

    #[test]
    fn test_property_unsubscribe() {
        let prop = Property::new(0);
        let received = Arc::new(Mutex::new(0));

        let id = prop.subscribe({
            let received = received.clone();
            move |value| {
                *received.lock().unwrap() = *value;
            }
        });

        prop.set(5);
        assert_eq!(*received.lock().unwrap(), 5);

        prop.unsubscribe(id);
        prop.set(10);
        assert_eq!(*received.lock().unwrap(), 5); // Should still be 5
    }
}

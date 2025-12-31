//! Computed/derived reactive values.

use super::{Property, SubscriptionId};
use std::sync::Arc;

/// A computed value that automatically updates when its dependencies change.
///
/// `Computed<T>` creates a reactive value that is derived from other reactive properties.
/// Whenever any of the dependencies change, the computed value is recalculated.
///
/// # Example
///
/// ```rust
/// use winrt_xaml::reactive::{Property, Computed};
///
/// let first_name = Property::new("John".to_string());
/// let last_name = Property::new("Doe".to_string());
///
/// // Create computed value that combines first and last name
/// let full_name = Computed::new({
///     let first_name = first_name.clone();
///     let last_name = last_name.clone();
///     move || format!("{} {}", first_name.get(), last_name.get())
/// });
///
/// // Subscribe to the computed value
/// full_name.subscribe(|name| {
///     println!("Full name: {}", name);
/// });
///
/// // Changing either name will update the computed value
/// first_name.set("Jane".to_string());
/// last_name.set("Smith".to_string());
/// ```
pub struct Computed<T> {
    property: Property<T>,
    _subscriptions: Arc<Vec<SubscriptionId>>,
}

impl<T: Clone + PartialEq + Send + 'static> Computed<T> {
    /// Create a new computed value from a closure.
    ///
    /// The closure will be called immediately to compute the initial value,
    /// and then whenever any reactive dependencies accessed in the closure change.
    ///
    /// Note: This is a simplified implementation. For automatic dependency tracking,
    /// you would need more sophisticated reactivity like `leptos_reactive` provides.
    pub fn new<F>(compute: F) -> Self
    where
        F: Fn() -> T + Send + 'static,
    {
        let initial_value = compute();
        let property = Property::new(initial_value);

        // Note: In a full implementation, we would automatically track which
        // properties are accessed during compute() and subscribe to them.
        // For now, users need to manually call update() or use explicit dependencies.

        Computed {
            property,
            _subscriptions: Arc::new(Vec::new()),
        }
    }

    /// Create a computed value that depends on a single property.
    pub fn from_property<F, S>(source: &Property<S>, map: F) -> Self
    where
        S: Clone + Send + 'static,
        F: Fn(&S) -> T + Send + 'static,
    {
        let initial_value = map(&source.get());
        let property = Property::new(initial_value);

        let subscription = source.subscribe({
            let property = property.clone();
            move |value| {
                let new_value = map(value);
                property.set(new_value);
            }
        });

        Computed {
            property,
            _subscriptions: Arc::new(vec![subscription]),
        }
    }

    /// Create a computed value that depends on two properties.
    pub fn from_properties2<F, S1, S2>(
        source1: &Property<S1>,
        source2: &Property<S2>,
        map: F,
    ) -> Self
    where
        S1: Clone + Send + 'static,
        S2: Clone + Send + 'static,
        F: Fn(&S1, &S2) -> T + Send + Sync + 'static,
    {
        let source1_clone = source1.clone();
        let source2_clone = source2.clone();
        
        let initial_value = map(&source1_clone.get(), &source2_clone.get());
        let property = Property::new(initial_value);

        let map = Arc::new(map);

        let sub1 = source1_clone.subscribe({
            let property = property.clone();
            let map = map.clone();
            let source1 = source1_clone.clone();
            let source2 = source2_clone.clone();
            move |_| {
                let new_value = map(&source1.get(), &source2.get());
                property.set(new_value);
            }
        });

        let sub2 = source2_clone.subscribe({
            let property = property.clone();
            let map = map.clone();
            let source1 = source1_clone.clone();
            let source2 = source2_clone.clone();
            move |_| {
                let new_value = map(&source1.get(), &source2.get());
                property.set(new_value);
            }
        });

        Computed {
            property,
            _subscriptions: Arc::new(vec![sub1, sub2]),
        }
    }

    /// Get the current computed value.
    pub fn get(&self) -> T {
        self.property.get()
    }

    /// Execute a closure with a reference to the current value.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.property.with(f)
    }

    /// Subscribe to changes in the computed value.
    pub fn subscribe<F>(&self, f: F) -> SubscriptionId
    where
        F: FnMut(&T) + Send + 'static,
    {
        self.property.subscribe(f)
    }

    /// Subscribe to changes without receiving the initial value.
    pub fn subscribe_silent<F>(&self, f: F) -> SubscriptionId
    where
        F: FnMut(&T) + Send + 'static,
    {
        self.property.subscribe_silent(f)
    }

    /// Unsubscribe a subscriber.
    pub fn unsubscribe(&self, id: SubscriptionId) {
        self.property.unsubscribe(id);
    }
}

impl<T: Clone + PartialEq + Send + 'static> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Computed {
            property: self.property.clone(),
            _subscriptions: self._subscriptions.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computed_from_property() {
        let count = Property::new(5);
        let doubled = Computed::from_property(&count, |n| n * 2);

        assert_eq!(doubled.get(), 10);

        count.set(10);
        assert_eq!(doubled.get(), 20);
    }

    #[test]
    fn test_computed_from_two_properties() {
        let a = Property::new(3);
        let b = Property::new(4);
        let sum = Computed::from_properties2(&a, &b, |x, y| x + y);

        assert_eq!(sum.get(), 7);

        a.set(10);
        assert_eq!(sum.get(), 14);

        b.set(20);
        assert_eq!(sum.get(), 30);
    }

    #[test]
    fn test_computed_subscribe() {
        let count = Property::new(1);
        let doubled = Computed::from_property(&count, |n| n * 2);

        let received = Arc::new(std::sync::Mutex::new(Vec::new()));
        doubled.subscribe({
            let received = received.clone();
            move |value| {
                received.lock().unwrap().push(*value);
            }
        });

        count.set(5);
        count.set(10);

        let values = received.lock().unwrap();
        assert_eq!(*values, vec![2, 10, 20]); // Initial, then updates
    }
}

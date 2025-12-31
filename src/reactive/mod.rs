//! Reactive state management for WinRT-XAML applications.
//!
//! This module provides a Rust-idiomatic approach to data binding and reactive UI updates.
//! Instead of traditional XAML INotifyPropertyChanged, we use observable properties and
//! collections that automatically notify subscribers when values change.
//!
//! # Features
//!
//! - **Property<T>**: Observable value that notifies subscribers on change
//! - **ObservableCollection<T>**: Observable vector with change notifications
//! - **Computed<T>**: Derived values that update automatically
//! - Thread-safe by default using Arc<Mutex<_>>
//!
//! # Example
//!
//! ```rust,no_run
//! use winrt_xaml::reactive::Property;
//! use winrt_xaml::prelude::*;
//!
//! # fn main() -> Result<()> {
//! // Create reactive property
//! let count = Property::new(0);
//!
//! // Create UI
//! let text = XamlTextBlock::new()?;
//!
//! // Bind text to count property
//! count.subscribe({
//!     let text = text.clone();
//!     move |value| {
//!         let _ = text.set_text(&format!("Count: {}", value));
//!     }
//! });
//!
//! // Update count (automatically updates UI)
//! count.set(5);
//! # Ok(())
//! # }
//! ```

mod property;
mod collection;
mod computed;

pub use property::Property;
pub use collection::{ObservableCollection, CollectionChange};
pub use computed::Computed;

use std::sync::{Arc, Mutex};

/// A subscriber callback that receives value updates.
pub type Subscriber<T> = Arc<Mutex<dyn FnMut(&T) + Send>>;

/// Unique identifier for subscribers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(usize);

impl SubscriptionId {
    pub(crate) fn new(id: usize) -> Self {
        SubscriptionId(id)
    }
}

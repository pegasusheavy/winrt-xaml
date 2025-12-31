//! Event handling system.

use parking_lot::RwLock;
use std::sync::Arc;

/// Event handler callback type.
type EventCallback<T> = Box<dyn Fn(&T) + Send + Sync>;

/// A type-safe event handler.
pub struct EventHandler<T> {
    handlers: Arc<RwLock<Vec<EventCallback<T>>>>,
}

impl<T> std::fmt::Debug for EventHandler<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandler")
            .field("handlers", &format!("{} handler(s)", self.handlers.read().len()))
            .finish()
    }
}

impl<T> EventHandler<T> {
    /// Create a new event handler.
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Subscribe to the event.
    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.handlers.write().push(Box::new(handler));
    }

    /// Invoke all subscribed handlers.
    pub fn invoke(&self, args: &T) {
        for handler in self.handlers.read().iter() {
            handler(args);
        }
    }

    /// Get the number of subscribed handlers.
    pub fn subscriber_count(&self) -> usize {
        self.handlers.read().len()
    }

    /// Clear all subscribed handlers.
    pub fn clear(&self) {
        self.handlers.write().clear();
    }
}

impl<T> Clone for EventHandler<T> {
    fn clone(&self) -> Self {
        Self {
            handlers: self.handlers.clone(),
        }
    }
}

impl<T> Default for EventHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Routed event args with source information.
#[derive(Debug, Clone)]
pub struct RoutedEventArgs {
    /// Whether the event has been handled.
    pub handled: bool,
    /// The source object of the event.
    pub source: Option<String>,
}

impl RoutedEventArgs {
    /// Create new routed event args.
    pub fn new() -> Self {
        Self {
            handled: false,
            source: None,
        }
    }

    /// Create routed event args with a source.
    pub fn with_source(source: impl Into<String>) -> Self {
        Self {
            handled: false,
            source: Some(source.into()),
        }
    }
}

impl Default for RoutedEventArgs {
    fn default() -> Self {
        Self::new()
    }
}

/// Click event arguments.
#[derive(Debug, Clone)]
pub struct ClickEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
}

impl ClickEventArgs {
    /// Create new click event args.
    pub fn new() -> Self {
        Self {
            routed: RoutedEventArgs::new(),
        }
    }
}

impl Default for ClickEventArgs {
    fn default() -> Self {
        Self::new()
    }
}

/// Text changed event arguments.
#[derive(Debug, Clone)]
pub struct TextChangedEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// The new text value.
    pub text: String,
}

impl TextChangedEventArgs {
    /// Create new text changed event args.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            text: text.into(),
        }
    }
}

/// Selection changed event arguments.
#[derive(Debug, Clone)]
pub struct SelectionChangedEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// The new selected index.
    pub selected_index: i32,
}

impl SelectionChangedEventArgs {
    /// Create new selection changed event args.
    pub fn new(selected_index: i32) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            selected_index,
        }
    }
}

/// Value changed event arguments.
#[derive(Debug, Clone)]
pub struct ValueChangedEventArgs<T> {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// The old value.
    pub old_value: T,
    /// The new value.
    pub new_value: T,
}

impl<T> ValueChangedEventArgs<T> {
    /// Create new value changed event args.
    pub fn new(old_value: T, new_value: T) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            old_value,
            new_value,
        }
    }
}

/// Checked event arguments.
#[derive(Debug, Clone)]
pub struct CheckedEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// The new checked state.
    pub is_checked: bool,
}

impl CheckedEventArgs {
    /// Create new checked event args.
    pub fn new(is_checked: bool) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            is_checked,
        }
    }
}

/// Key event arguments.
#[derive(Debug, Clone)]
pub struct KeyEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// The virtual key code.
    pub key_code: u32,
    /// Whether the key is down.
    pub is_down: bool,
}

impl KeyEventArgs {
    /// Create new key event args.
    pub fn new(key_code: u32, is_down: bool) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            key_code,
            is_down,
        }
    }
}

/// Mouse event arguments.
#[derive(Debug, Clone)]
pub struct MouseEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// The X position.
    pub x: i32,
    /// The Y position.
    pub y: i32,
}

impl MouseEventArgs {
    /// Create new mouse event args.
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            x,
            y,
        }
    }
}

/// Focus event arguments.
#[derive(Debug, Clone)]
pub struct FocusEventArgs {
    /// Base routed event args.
    pub routed: RoutedEventArgs,
    /// Whether the control gained focus.
    pub got_focus: bool,
}

impl FocusEventArgs {
    /// Create new focus event args.
    pub fn new(got_focus: bool) -> Self {
        Self {
            routed: RoutedEventArgs::new(),
            got_focus,
        }
    }
}

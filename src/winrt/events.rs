//! WinRT event handling system.
//!
//! Provides event subscription and routing for WinRT objects.

use crate::error::Result;
use std::sync::{Arc, RwLock};

/// Event token returned when subscribing to events.
/// Used to unsubscribe from events.
#[derive(Debug, Clone, Copy)]
pub struct EventToken {
    id: u64,
}

impl EventToken {
    /// Create a new event token with the given ID.
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    /// Get the token ID.
    pub fn id(&self) -> u64 {
        self.id
    }
}

/// WinRT event handler.
pub type WinRTEventHandler<TArgs> = Box<dyn Fn(&TArgs) + Send + Sync>;

/// WinRT event registration for a specific event type.
pub struct WinRTEvent<TArgs> {
    handlers: Arc<RwLock<Vec<(EventToken, WinRTEventHandler<TArgs>)>>>,
    next_token: Arc<RwLock<u64>>,
}

impl<TArgs> WinRTEvent<TArgs> {
    /// Create a new WinRT event.
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
            next_token: Arc::new(RwLock::new(1)),
        }
    }

    /// Subscribe to this event.
    pub fn subscribe<F>(&self, handler: F) -> Result<EventToken>
    where
        F: Fn(&TArgs) + Send + Sync + 'static,
    {
        let mut next_token = self.next_token.write()
            .map_err(|e| crate::error::Error::synchronization(format!("Failed to get next token: {}", e)))?;
        let token = EventToken::new(*next_token);
        *next_token += 1;

        let mut handlers = self.handlers.write()
            .map_err(|e| crate::error::Error::synchronization(format!("Failed to add handler: {}", e)))?;
        handlers.push((token, Box::new(handler)));

        println!("   📡 Event subscribed (token: {})", token.id());
        Ok(token)
    }

    /// Unsubscribe from this event.
    pub fn unsubscribe(&self, token: EventToken) -> Result<()> {
        let mut handlers = self.handlers.write()
            .map_err(|e| crate::error::Error::synchronization(format!("Failed to remove handler: {}", e)))?;
        handlers.retain(|(t, _)| t.id() != token.id());

        println!("   📡 Event unsubscribed (token: {})", token.id());
        Ok(())
    }

    /// Invoke all handlers for this event.
    pub fn invoke(&self, args: &TArgs) -> Result<()> {
        let handlers = self.handlers.read()
            .map_err(|e| crate::error::Error::synchronization(format!("Failed to read handlers: {}", e)))?;

        for (_token, handler) in handlers.iter() {
            handler(args);
        }
        Ok(())
    }

    /// Get the number of subscribed handlers.
    pub fn handler_count(&self) -> Result<usize> {
        let handlers = self.handlers.read()
            .map_err(|e| crate::error::Error::synchronization(format!("Failed to read handlers: {}", e)))?;
        Ok(handlers.len())
    }
}

impl<TArgs> Clone for WinRTEvent<TArgs> {
    fn clone(&self) -> Self {
        Self {
            handlers: Arc::clone(&self.handlers),
            next_token: Arc::clone(&self.next_token),
        }
    }
}

impl<TArgs> Default for WinRTEvent<TArgs> {
    fn default() -> Self {
        Self::new()
    }
}

/// Common WinRT event args.
#[derive(Debug, Clone)]
pub struct WinRTRoutedEventArgs {
    /// The event source.
    pub source: String,
}

impl WinRTRoutedEventArgs {
    /// Create new routed event args.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

/// WinRT click event args.
#[derive(Debug, Clone)]
pub struct WinRTClickEventArgs {
    /// Base routed event args.
    pub routed_args: WinRTRoutedEventArgs,
}

impl WinRTClickEventArgs {
    /// Create new click event args.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            routed_args: WinRTRoutedEventArgs::new(source),
        }
    }
}

/// WinRT property changed event args.
#[derive(Debug, Clone)]
pub struct WinRTPropertyChangedEventArgs {
    /// The property name that changed.
    pub property_name: String,
    /// The old value (placeholder).
    pub old_value: String,
    /// The new value (placeholder).
    pub new_value: String,
}

impl WinRTPropertyChangedEventArgs {
    /// Create new property changed event args.
    pub fn new(
        property_name: impl Into<String>,
        old_value: impl Into<String>,
        new_value: impl Into<String>,
    ) -> Self {
        Self {
            property_name: property_name.into(),
            old_value: old_value.into(),
            new_value: new_value.into(),
        }
    }
}


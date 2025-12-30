//! Resource management and dictionaries.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// A dictionary of resources.
#[derive(Clone)]
pub struct ResourceDictionary {
    inner: Arc<RwLock<HashMap<String, ResourceValue>>>,
}

/// A resource value that can hold various types.
#[derive(Clone)]
pub enum ResourceValue {
    /// String resource.
    String(String),
    /// Integer resource.
    Integer(i32),
    /// Float resource.
    Float(f64),
    /// Boolean resource.
    Boolean(bool),
}

impl ResourceDictionary {
    /// Create a new resource dictionary.
    pub fn new() -> Self {
        ResourceDictionary {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insert a resource.
    pub fn insert(&self, key: impl Into<String>, value: ResourceValue) {
        self.inner.write().insert(key.into(), value);
    }

    /// Get a resource.
    pub fn get(&self, key: &str) -> Option<ResourceValue> {
        self.inner.read().get(key).cloned()
    }

    /// Remove a resource.
    pub fn remove(&self, key: &str) -> Option<ResourceValue> {
        self.inner.write().remove(key)
    }

    /// Clear all resources.
    pub fn clear(&self) {
        self.inner.write().clear();
    }

    /// Check if a resource exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.read().contains_key(key)
    }
}

impl Default for ResourceDictionary {
    fn default() -> Self {
        Self::new()
    }
}

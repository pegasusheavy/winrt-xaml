//! ListView control - stub implementation.

use crate::controls::UIElement;
use crate::error::Result;
use crate::events::{EventHandler, SelectionChangedEventArgs};
use parking_lot::RwLock;
use std::sync::Arc;
use windows::Win32::Foundation::*;

/// A list view control.
#[derive(Clone)]
pub struct ListView {
    element: UIElement,
    inner: Arc<ListViewInner>,
}

struct ListViewInner {
    items: RwLock<Vec<String>>,
    selected_index: RwLock<i32>,
    selection_changed: EventHandler<SelectionChangedEventArgs>,
}

impl ListView {
    /// Create a new list view.
    pub fn new() -> Result<Self> {
        let inner = Arc::new(ListViewInner {
            items: RwLock::new(Vec::new()),
            selected_index: RwLock::new(-1),
            selection_changed: EventHandler::new(),
        });

        Ok(ListView {
            element: UIElement::empty(),
            inner,
        })
    }

    /// Get the items in the list view.
    pub fn items(&self) -> Vec<String> {
        self.inner.items.read().clone()
    }

    /// Add an item to the list view.
    pub fn add_item(&self, item: impl Into<String>) {
        self.inner.items.write().push(item.into());
    }

    /// Clear all items from the list view.
    pub fn clear_items(&self) {
        self.inner.items.write().clear();
    }

    /// Get the selected index.
    pub fn selected_index(&self) -> i32 {
        *self.inner.selected_index.read()
    }

    /// Set the selected index.
    pub fn set_selected_index(&self, index: i32) {
        *self.inner.selected_index.write() = index;
    }

    /// Set the selected index (fluent API).
    pub fn with_selected_index(self, index: i32) -> Self {
        self.set_selected_index(index);
        self
    }

    /// Subscribe to the selection changed event.
    pub fn selection_changed(&self) -> &EventHandler<SelectionChangedEventArgs> {
        &self.inner.selection_changed
    }

    /// Get the underlying UI element.
    pub fn element(&self) -> &UIElement {
        &self.element
    }

    /// Get the HWND of this list view.
    pub fn hwnd(&self) -> HWND {
        self.element.hwnd()
    }
}

impl Default for ListView {
    fn default() -> Self {
        Self::new().expect("Failed to create list view")
    }
}

impl From<ListView> for UIElement {
    fn from(listview: ListView) -> Self {
        listview.element.clone()
    }
}

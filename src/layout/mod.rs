//! Layout panels and containers using manual layout calculations.

mod stack_panel;
mod grid;
mod border;
mod scroll_viewer;

pub use self::border::Border;
pub use self::grid::Grid;
pub use self::scroll_viewer::ScrollViewer;
pub use self::stack_panel::StackPanel;

/// Orientation for layout panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Horizontal orientation.
    Horizontal,
    /// Vertical orientation.
    Vertical,
}

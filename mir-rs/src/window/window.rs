//! The Window handle type.

use crate::geometry::{Point, Size};

/// A handle to a window in the compositor.
///
/// Windows are lightweight, cloneable identifiers. They can be used as keys
/// in `HashMap` or `HashSet` for the external storage pattern.
///
/// A default-constructed `Window` is invalid (not backed by any surface).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Window {
    id: u64,
    position: Point,
    size: Size,
}

impl Window {
    /// Create a window from its FFI components.
    pub(crate) fn from_ffi(id: u64, position: Point, size: Size) -> Self {
        Self { id, position, size }
    }

    /// Get the unique identifier for this window.
    ///
    /// This ID is stable for the lifetime of the window and can be used
    /// as a key in data structures.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the top-left position of the window frame.
    pub fn top_left(&self) -> Point {
        self.position
    }

    /// Get the size of the window frame (including decorations).
    pub fn size(&self) -> Size {
        self.size
    }

    /// Check if this window handle is valid (backed by a surface).
    pub fn is_valid(&self) -> bool {
        self.id != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_window_is_invalid() {
        let window = Window::default();
        assert_eq!(window.id(), 0);
        assert_eq!(window.top_left(), Point::default());
        assert_eq!(window.size(), Size::default());
        assert!(!window.is_valid());
    }

    #[test]
    fn window_exposes_its_ffi_components() {
        let window = Window::from_ffi(7, Point::new(10, 20), Size::new(640, 480));
        assert_eq!(window.id(), 7);
        assert_eq!(window.top_left(), Point::new(10, 20));
        assert_eq!(window.size(), Size::new(640, 480));
        assert!(window.is_valid());
    }
}

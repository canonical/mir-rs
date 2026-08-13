//! Workspace management.
//!
//! Workspaces are virtual desktops that group windows together.
//! Each workspace has a unique identifier and can be activated/deactivated.

/// A handle to a workspace (virtual desktop).
///
/// Workspaces are lightweight, cloneable identifiers. They can be used
/// as keys in `HashMap` or `HashSet`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Workspace {
    id: u64,
}

impl Workspace {
    /// Create a workspace handle from a raw ID.
    ///
    /// Workspace IDs are opaque; the compositor supplies them and a policy may store or
    /// recreate a handle from one.
    pub fn from_id(id: u64) -> Self {
        Self { id }
    }

    /// Get the unique identifier for this workspace.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Build a handle from a raw workspace ID returned by the FFI layer.
    ///
    /// The single conversion seam the crate uses at the FFI boundary, mirroring
    /// [`Window::from_ffi`](crate::window::Window::from_ffi).
    pub(crate) fn from_ffi(id: u64) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_id_round_trips() {
        assert_eq!(Workspace::from_id(42).id(), 42);
    }

    #[test]
    fn from_ffi_round_trips() {
        assert_eq!(Workspace::from_ffi(7).id(), 7);
        assert_eq!(Workspace::from_ffi(7), Workspace::from_id(7));
    }

    #[test]
    fn workspaces_with_the_same_id_are_equal() {
        assert_eq!(Workspace::from_id(3), Workspace::from_id(3));
        assert_ne!(Workspace::from_id(3), Workspace::from_id(4));
    }
}

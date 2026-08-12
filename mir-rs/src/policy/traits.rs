//! The WindowManagementPolicy trait and Advice enum.

use crate::application::ApplicationInfo;
use crate::geometry::{Displacement, Point, Rectangle, Size};
use crate::input::{InputEvent, KeyboardEvent, PointerEvent, TouchEvent};
use crate::output::{Output, Zone};
use crate::policy::WindowManagerTools;
use crate::window::{ResizeEdge, Window, WindowInfo, WindowSpecification, WindowState};

/// Advisory notifications from the compositor.
///
/// These are fire-and-forget notifications — no response is expected.
/// Use `match` to handle the events you care about and ignore the rest
/// with a wildcard (`_ => {}`).
///
/// Marked `#[non_exhaustive]` so new events can be added without breaking
/// existing code.
#[non_exhaustive]
pub enum Advice {
    /// A group of related notifications is starting.
    Begin,
    /// A group of related notifications has ended.
    End,

    // --- Application lifecycle ---
    /// A new application has connected.
    NewApp {
        /// The application info for the newly connected app.
        app: ApplicationInfo,
    },
    /// An application has disconnected.
    DeleteApp {
        /// The application info for the disconnecting app.
        app: ApplicationInfo,
    },

    // --- Window lifecycle ---
    /// A new window has been created.
    NewWindow {
        /// Information about the new window.
        window_info: WindowInfo,
    },
    /// A window is being destroyed.
    DeleteWindow {
        /// Information about the window being destroyed.
        window_info: WindowInfo,
    },
    /// A window has gained input focus.
    FocusGained {
        /// Information about the focused window.
        window_info: WindowInfo,
    },
    /// A window has lost input focus.
    FocusLost {
        /// Information about the window that lost focus.
        window_info: WindowInfo,
    },
    /// A window's state has changed (e.g., maximized, minimized).
    StateChange {
        /// Information about the window whose state changed.
        window_info: WindowInfo,
        /// The new state.
        state: WindowState,
    },
    /// A window has moved to a new position.
    MoveTo {
        /// Information about the moved window.
        window_info: WindowInfo,
        /// The new top-left position.
        top_left: Point,
    },
    /// A window has been resized.
    Resize {
        /// Information about the resized window.
        window_info: WindowInfo,
        /// The new size.
        new_size: Size,
    },
    /// Windows have been raised in the stacking order.
    Raise {
        /// The windows that were raised.
        windows: Vec<Window>,
    },

    // --- Output lifecycle ---
    /// A new output has been connected.
    OutputCreate {
        /// The new output.
        output: Output,
    },
    /// An existing output's properties have changed.
    OutputUpdate {
        /// The updated output state.
        updated: Output,
        /// The previous output state.
        original: Output,
    },
    /// An output has been disconnected.
    OutputDelete {
        /// The disconnected output.
        output: Output,
    },

    // --- Application zone lifecycle ---
    /// A new application zone has been created.
    ZoneCreate {
        /// The new zone.
        zone: Zone,
    },
    /// An existing zone's extents have changed.
    ZoneUpdate {
        /// The updated zone.
        updated: Zone,
        /// The previous zone extents.
        original: Zone,
    },
    /// A zone has been removed.
    ZoneDelete {
        /// The removed zone.
        zone: Zone,
    },

    // --- Workspace lifecycle ---
    /// Windows are being added to a workspace.
    AddingToWorkspace {
        /// The windows being added.
        windows: Vec<Window>,
    },
    /// Windows are being removed from a workspace.
    RemovingFromWorkspace {
        /// The windows being removed.
        windows: Vec<Window>,
    },
}

/// The primary trait for implementing a window management policy.
///
/// Every method has a sensible default — an empty `impl` behaves like a floating
/// window manager that honors all client requests. Override the methods you care
/// about.
///
/// The window manager tools are always available to a policy through
/// [`tools()`](Self::tools); there is nothing to store and nothing to initialize.
/// The `advise` method receives lifecycle notifications that don't require a
/// response.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Default)]
/// struct MyPolicy;
///
/// impl WindowManagementPolicy for MyPolicy {
///     fn place_new_window(
///         &mut self,
///         _app_info: &ApplicationInfo,
///         requested: &WindowSpecification,
///     ) -> WindowSpecification {
///         // Place all windows at (0, 0)
///         requested.clone().with_top_left(Point::new(0, 0))
///     }
/// }
/// ```
pub trait WindowManagementPolicy: Send + 'static {
    /// Access the window manager tools.
    ///
    /// The tools provide actions like raising windows, setting focus, and
    /// modifying window properties. The server makes them available once it has
    /// constructed the policy, so this works in every policy method — but not in
    /// the policy's own constructor, where miral is still building its window
    /// management model.
    ///
    /// Override this only if your policy holds its own
    /// [`WindowManagerTools`] handle and you would rather return that.
    fn tools(&self) -> &WindowManagerTools {
        WindowManagerTools::global_ref()
    }

    /// Called to determine where a new window should be placed.
    ///
    /// Default: honors the requested specification as-is.
    fn place_new_window(
        &mut self,
        _app_info: &ApplicationInfo,
        requested: &WindowSpecification,
    ) -> WindowSpecification {
        requested.clone()
    }

    /// Called when a window's first buffer has been posted and it is ready to display.
    ///
    /// Default: no-op.
    fn handle_window_ready(&mut self, _window_info: &WindowInfo) {}

    /// Called when a client requests modifications to its window.
    ///
    /// Default: applies all requested modifications.
    fn handle_modify_window(
        &mut self,
        window_info: &WindowInfo,
        modifications: &WindowSpecification,
    ) {
        self.tools()
            .modify_window(window_info.window(), modifications);
    }

    /// Called when a client requests its window be activated.
    ///
    /// Default: raises the window tree.
    fn handle_activate_window(&mut self, window_info: &WindowInfo) {
        self.tools().raise_tree(window_info.window());
    }

    /// Confirm placement of a maximized/fullscreen window.
    ///
    /// Default: accepts the suggested placement rectangle.
    fn confirm_placement_on_display(
        &mut self,
        _window_info: &WindowInfo,
        _new_state: WindowState,
        new_placement: Rectangle,
    ) -> Rectangle {
        new_placement
    }

    /// Handle a keyboard event. Return `true` if consumed.
    ///
    /// Default: not consumed (passes through to clients).
    fn handle_keyboard_event(&mut self, _event: &KeyboardEvent) -> bool {
        false
    }

    /// Handle a touch event. Return `true` if consumed.
    ///
    /// Default: not consumed (passes through to clients).
    fn handle_touch_event(&mut self, _event: &TouchEvent) -> bool {
        false
    }

    /// Handle a pointer (mouse) event. Return `true` if consumed.
    ///
    /// Default: not consumed (passes through to clients).
    fn handle_pointer_event(&mut self, _event: &PointerEvent) -> bool {
        false
    }

    /// Handle a client-initiated interactive move request.
    ///
    /// This is triggered by the client calling `xdg_toplevel::move`.
    /// Default: no-op.
    fn handle_request_move(&mut self, _window_info: &WindowInfo, _input_event: &InputEvent) {}

    /// Handle a client-initiated interactive resize request.
    ///
    /// This is triggered by the client calling `xdg_toplevel::resize`.
    /// `edge` identifies which edge or corner the client is dragging.
    /// Default: no-op.
    fn handle_request_resize(
        &mut self,
        _window_info: &WindowInfo,
        _input_event: &InputEvent,
        _edge: ResizeEdge,
    ) {
    }

    /// Confirm child window placement when its parent moves.
    ///
    /// Default: applies the displacement to the current position.
    fn confirm_inherited_move(
        &mut self,
        window_info: &WindowInfo,
        movement: Displacement,
    ) -> Rectangle {
        Rectangle {
            top_left: window_info.window().top_left() + movement,
            size: window_info.window().size(),
        }
    }

    /// Called for all advisory notifications from the compositor.
    ///
    /// Override this to react to lifecycle events such as new/deleted
    /// applications, windows, outputs, and zones.
    ///
    /// Default: no-op.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn advise(&mut self, event: Advice) {
    ///     match event {
    ///         Advice::NewApp { app } => {
    ///             println!("New app: {:?}", app.name());
    ///         }
    ///         Advice::ZoneUpdate { updated, .. } => {
    ///             self.retile(updated.extents());
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// ```
    fn advise(&mut self, _event: Advice) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{KeyAction, Modifiers};

    /// A policy needs no methods at all: the tools are supplied by the framework.
    struct MinimalPolicy;

    impl WindowManagementPolicy for MinimalPolicy {}

    #[test]
    fn default_place_new_window_honors_the_request() {
        let mut policy = MinimalPolicy;
        let app = ApplicationInfo::from_ffi(1, "test-app".to_string());
        let requested = WindowSpecification::new()
            .with_top_left(Point::new(3, 5))
            .with_size(Size::new(640, 480));

        let placed = policy.place_new_window(&app, &requested);

        assert_eq!(placed.top_left(), Some(Point::new(3, 5)));
        assert_eq!(placed.size(), Some(Size::new(640, 480)));
    }

    #[test]
    fn default_input_handlers_do_not_consume_events() {
        let mut policy = MinimalPolicy;
        let event = KeyboardEvent {
            action: KeyAction::Down,
            key_code: 30,
            keysym: 0x61,
            modifiers: Modifiers::default(),
            timestamp_ns: 0,
        };

        assert!(!policy.handle_keyboard_event(&event));
    }

    #[test]
    fn default_tools_is_the_shared_handle() {
        let policy = MinimalPolicy;
        assert_eq!(*policy.tools(), WindowManagerTools::current());
    }
}

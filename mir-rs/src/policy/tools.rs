//! Window manager tools for performing actions from within a policy.

use std::pin::Pin;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::geometry::{Displacement, Point, Rectangle, Size};
use crate::output::Zone;
use crate::window::{Window, WindowInfo, WindowSpecification, WindowState};
use crate::workspace::Workspace;

/// The C++ tools object for the running server.
///
/// Published once the policy has been constructed and cleared as soon as the
/// policy is destroyed, so a handle that outlives the server panics on use rather
/// than dereferencing a destroyed C++ object.
static TOOLS_PTR: AtomicPtr<crate::sys::ffi::MiralTools> = AtomicPtr::new(ptr::null_mut());

/// The handle returned by [`WindowManagerTools::global_ref`].
static TOOLS: WindowManagerTools = WindowManagerTools { _private: () };

/// Provides actions that a policy can take to manage windows.
///
/// This is a lightweight handle, not an owner: every method is a call through to
/// the compositor's window management model. Obtain one with
/// [`WindowManagerTools::current`], or simply call
/// [`WindowManagementPolicy::tools`](crate::policy::WindowManagementPolicy::tools)
/// from inside a policy — a policy does not need to store anything.
///
/// # Availability
///
/// The tools become usable once the policy has been constructed, and stop being
/// usable when the server destroys it. In particular they are **not** usable
/// while a policy is being constructed: miral is still building its window
/// management model at that point, so a policy's constructor may store a handle
/// but must not call through it. Every method panics when the tools are
/// unavailable; [`is_available`](Self::is_available) reports the state.
///
/// # One server per process
///
/// Mir runs a single server, with a single window management policy, per process
/// ([`MirRunner::run`](crate::runner::MirRunner::run) consumes the runner), so
/// all handles refer to that one server.
///
/// # Threads
///
/// Policy dispatch is single-threaded; from any other thread, wrap tools calls in
/// [`invoke_under_lock`](Self::invoke_under_lock).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowManagerTools {
    _private: (),
}

impl WindowManagerTools {
    /// Get a handle to the running server's window manager tools.
    ///
    /// The handle stays valid for as long as the server runs; see
    /// [availability](Self#availability). Storing it in a policy is optional —
    /// [`WindowManagementPolicy::tools`](crate::policy::WindowManagementPolicy::tools)
    /// returns the same thing.
    pub fn current() -> Self {
        Self { _private: () }
    }

    /// Whether the tools are usable, i.e. the server has finished constructing
    /// the policy and has not destroyed it yet.
    ///
    /// Calling any other method while this returns `false` panics.
    pub fn is_available() -> bool {
        !TOOLS_PTR.load(Ordering::Acquire).is_null()
    }

    /// Publish the tools pointer supplied by the runner.
    ///
    /// Must not be called before the policy has been constructed: miral hands the
    /// tools over while its window manager is still being built, and calling
    /// through them before that finishes is undefined behaviour.
    ///
    /// # Safety
    ///
    /// `ptr` must point at a live `MiralTools` object that stays valid until
    /// [`uninstall`](Self::uninstall) is called.
    pub(crate) unsafe fn install(ptr: *mut crate::sys::ffi::MiralTools) {
        TOOLS_PTR.store(ptr, Ordering::Release);
    }

    /// Invalidate every handle, before the C++ tools object is destroyed.
    pub(crate) fn uninstall() {
        TOOLS_PTR.store(ptr::null_mut(), Ordering::Release);
    }

    /// A borrowed handle, for the default implementation of
    /// [`WindowManagementPolicy::tools`](crate::policy::WindowManagementPolicy::tools).
    pub(crate) fn global_ref() -> &'static Self {
        &TOOLS
    }

    /// Get a pinned mutable reference to the underlying tools.
    ///
    /// Takes `&self` because the C++ `MiralTools` object is owned by the server, not by
    /// this handle: every tools method is a call *through* the pointer rather than a
    /// mutation of the Rust struct, so requiring `&mut self` would only force needless
    /// `RefCell`s on policy implementations.
    ///
    /// Safety: the pointer is published once the policy exists and cleared when the
    /// policy is dropped, so it is valid whenever it is non-null; dispatch is
    /// single-threaded (and other threads must go through `invoke_under_lock`), so no
    /// two `&mut` borrows are live at once. The assertion catches use outside that
    /// window.
    #[allow(clippy::mut_from_ref)]
    fn pin_mut(&self) -> Pin<&mut crate::sys::ffi::MiralTools> {
        let raw = TOOLS_PTR.load(Ordering::Acquire);
        assert!(!raw.is_null(), "WindowManagerTools not available");
        unsafe { Pin::new_unchecked(&mut *raw) }
    }

    /// Get an immutable reference to the underlying tools.
    fn as_ref(&self) -> &crate::sys::ffi::MiralTools {
        let raw = TOOLS_PTR.load(Ordering::Acquire);
        assert!(!raw.is_null(), "WindowManagerTools not available");
        unsafe { &*raw }
    }

    /// Raise a window (and its children) to the top of the stacking order.
    ///
    /// This brings the window and all of its child/satellite windows to
    /// the top of the z-order.
    pub fn raise_tree(&self, window: &Window) {
        crate::sys::ffi::miral_tools_raise_tree_by_id(self.pin_mut(), window.id());
    }

    /// Set the input focus to the given window.
    ///
    /// The window will receive keyboard events after this call.
    pub fn select_active_window(&self, window: &Window) {
        crate::sys::ffi::miral_tools_select_active_window_by_id(self.pin_mut(), window.id());
    }

    /// Modify a window's properties according to the given specification.
    ///
    /// Only fields that are set in the specification will be changed.
    pub fn modify_window(&self, window: &Window, spec: &WindowSpecification) {
        let ffi_spec = spec.to_ffi();
        crate::sys::ffi::miral_tools_modify_window_by_id(self.pin_mut(), window.id(), &ffi_spec);
    }

    /// Focus the next application's window.
    pub fn focus_next_application(&self) {
        crate::sys::ffi::miral_tools_focus_next_application(self.pin_mut());
    }

    /// Focus the previous application's window.
    pub fn focus_prev_application(&self) {
        crate::sys::ffi::miral_tools_focus_prev_application(self.pin_mut());
    }

    /// Focus the next window within the active application.
    pub fn focus_next_within_application(&self) {
        crate::sys::ffi::miral_tools_focus_next_within_application(self.pin_mut());
    }

    /// Focus the previous window within the active application.
    pub fn focus_prev_within_application(&self) {
        crate::sys::ffi::miral_tools_focus_prev_within_application(self.pin_mut());
    }

    /// Ask a client to close its window gracefully.
    ///
    /// This sends a close request to the client (equivalent to clicking
    /// the window's close button). The client may choose to ignore it.
    pub fn ask_client_to_close(&self, window: &Window) {
        crate::sys::ffi::miral_tools_ask_client_to_close_by_id(self.pin_mut(), window.id());
    }

    /// Drag a window by the given displacement.
    pub fn drag_window(&self, window: &Window, movement: Displacement) {
        crate::sys::ffi::miral_tools_drag_window_by_id(
            self.pin_mut(),
            window.id(),
            movement.into(),
        );
    }

    /// Drag the currently active window by the given displacement.
    pub fn drag_active_window(&self, movement: Displacement) {
        crate::sys::ffi::miral_tools_drag_active_window(self.pin_mut(), movement.into());
    }

    /// Get the active (focused) window, if any.
    pub fn active_window(&self) -> Option<Window> {
        let id = crate::sys::ffi::miral_tools_active_window_id(self.as_ref());
        if id == 0 {
            None
        } else {
            Some(Window::from_ffi(id, Point::default(), Size::default()))
        }
    }

    /// Get the info for a specific window.
    ///
    /// Returns a snapshot of the window's current properties.
    pub fn info_for(&self, window: &Window) -> WindowInfo {
        let snapshot = crate::sys::ffi::miral_tools_info_for_window_id(self.as_ref(), window.id());
        WindowInfo::from_ffi(&snapshot, window.id())
    }

    /// Get the active application zone (the area available for tiling).
    ///
    /// The application zone is the output area minus any reserved space
    /// (e.g., panels, docks).
    pub fn active_application_zone(&self) -> Zone {
        let snapshot = crate::sys::ffi::miral_tools_active_application_zone(self.as_ref());
        Zone::from_ffi(&snapshot)
    }

    /// Get the rectangle of the active output.
    pub fn active_output(&self) -> Rectangle {
        crate::sys::ffi::miral_tools_active_output(self.as_ref()).into()
    }

    /// Get the number of connected applications.
    pub fn count_applications(&self) -> u32 {
        crate::sys::ffi::miral_tools_count_applications(self.as_ref())
    }

    /// Swap two windows in the stacking order.
    ///
    /// After this call, `window_a` will be at `window_b`'s old stacking position
    /// and vice versa.
    pub fn swap_tree_order(&self, window_a: &Window, window_b: &Window) {
        crate::sys::ffi::miral_tools_swap_tree_order_by_id(
            self.pin_mut(),
            window_a.id(),
            window_b.id(),
        );
    }

    /// Send a window tree to the back of the stacking order.
    pub fn send_tree_to_back(&self, window: &Window) {
        crate::sys::ffi::miral_tools_send_tree_to_back_by_id(self.pin_mut(), window.id());
    }

    /// Move the cursor to a specific point.
    pub fn move_cursor_to(&self, point: Point) {
        crate::sys::ffi::miral_tools_move_cursor_to(self.pin_mut(), point.into());
    }

    /// Find the window at a given point.
    ///
    /// Returns the topmost window under the point, or `None` if no window is there.
    pub fn window_at(&self, point: Point) -> Option<Window> {
        let id = crate::sys::ffi::miral_tools_window_id_at(self.as_ref(), point.into());
        if id == 0 {
            None
        } else {
            Some(Window::from_ffi(id, Point::default(), Size::default()))
        }
    }

    /// Get all window IDs in the active workspace.
    ///
    /// Returns a list of all windows, which can be used for iteration.
    pub fn all_windows(&self) -> Vec<Window> {
        let ids = crate::sys::ffi::miral_tools_all_window_ids(self.as_ref());
        ids.into_iter()
            .map(|id| Window::from_ffi(id, Point::default(), Size::default()))
            .collect()
    }

    /// Calculate the placement for a window given a new state.
    ///
    /// Returns the rectangle the window would occupy in the given state.
    pub fn place_and_size_for_state(
        &self,
        window: &Window,
        new_state: WindowState,
        current_rect: Rectangle,
    ) -> Rectangle {
        let ffi_rect: crate::sys::ffi::Rectangle = current_rect.into();
        crate::sys::ffi::miral_tools_place_and_size_for_state(
            self.as_ref(),
            window.id(),
            new_state.to_raw(),
            &ffi_rect,
        )
        .into()
    }

    /// Create a new workspace.
    ///
    /// Workspaces are a purely associative grouping of windows; miral attaches no
    /// behaviour to them. It is up to the policy to decide what a workspace means
    /// (for example, showing only the windows of an "active" workspace). The
    /// returned [`Workspace`] handle is used with the other workspace methods.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see [`is_available`](Self::is_available)).
    pub fn create_workspace(&self) -> Workspace {
        let id = crate::sys::ffi::miral_tools_create_workspace(self.pin_mut());
        Workspace::from_ffi(id)
    }

    /// Add a window (and its child tree) to a workspace.
    ///
    /// A window may belong to more than one workspace.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see [`is_available`](Self::is_available)).
    pub fn add_tree_to_workspace(&self, window: &Window, workspace: &Workspace) {
        crate::sys::ffi::miral_tools_add_tree_to_workspace(
            self.pin_mut(),
            window.id(),
            workspace.id(),
        );
    }

    /// Remove a window (and its child tree) from a workspace.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see [`is_available`](Self::is_available)).
    pub fn remove_tree_from_workspace(&self, window: &Window, workspace: &Workspace) {
        crate::sys::ffi::miral_tools_remove_tree_from_workspace(
            self.pin_mut(),
            window.id(),
            workspace.id(),
        );
    }

    /// Move every window associated with `from` into `to`.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see [`is_available`](Self::is_available)).
    pub fn move_workspace_content_to_workspace(&self, to: &Workspace, from: &Workspace) {
        crate::sys::ffi::miral_tools_move_workspace_content_to_workspace(
            self.pin_mut(),
            to.id(),
            from.id(),
        );
    }

    /// Get every workspace that contains the given window.
    ///
    /// Returns a snapshot: unlike miral's callback-based enumeration, the caller
    /// may freely add or remove windows from these workspaces afterwards.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see [`is_available`](Self::is_available)).
    pub fn workspaces_containing(&self, window: &Window) -> Vec<Workspace> {
        let ids =
            crate::sys::ffi::miral_tools_workspaces_containing_window(self.pin_mut(), window.id());
        ids.into_iter().map(Workspace::from_ffi).collect()
    }

    /// Get every window contained in the given workspace.
    ///
    /// Returns a snapshot: unlike miral's callback-based enumeration, the caller
    /// may freely add or remove windows from the workspace afterwards.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see [`is_available`](Self::is_available)).
    pub fn windows_in_workspace(&self, workspace: &Workspace) -> Vec<Window> {
        let ids = crate::sys::ffi::miral_tools_windows_in_workspace(self.pin_mut(), workspace.id());
        ids.into_iter()
            .map(|id| Window::from_ffi(id, Point::default(), Size::default()))
            .collect()
    }

    /// Acquire the window management model lock and run `callback` under it.
    ///
    /// This is how a thread that is **not** running a policy callback can safely
    /// call other [`WindowManagerTools`] methods: miral guards its window model
    /// with a lock, and every tools method expects that lock to be held.
    ///
    /// The callback runs synchronously, on the calling thread, before this method
    /// returns.
    ///
    /// # Panics
    ///
    /// Panics if the tools are not available (see
    /// [`is_available`](Self::is_available)).
    ///
    /// A panic inside `callback` cannot unwind through the C++ frames that hold
    /// the lock, so it aborts the process. Handle errors inside the callback.
    ///
    /// # Deadlocks
    ///
    /// Never call this from inside a policy callback (anything dispatched through
    /// [`WindowManagementPolicy`](crate::policy::WindowManagementPolicy)). Those
    /// already run under the lock, and the lock is not recursive, so re-acquiring
    /// it deadlocks the compositor. From within a policy, call the tools methods
    /// directly instead.
    ///
    /// Similarly, keep the callback short and never block in it — the whole
    /// window manager is stalled while it runs.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::sync::atomic::{AtomicBool, Ordering};
    /// use std::sync::Arc;
    ///
    /// use mir::prelude::*;
    ///
    /// // On a thread that is not running policy callbacks:
    /// fn tidy_up(done: Arc<AtomicBool>) {
    ///     WindowManagerTools::current().invoke_under_lock(move || {
    ///         // Tools calls made here are serialised against the compositor.
    ///         done.store(true, Ordering::SeqCst);
    ///     });
    /// }
    /// ```
    pub fn invoke_under_lock<F>(&self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        crate::sys::ffi::miral_tools_invoke_under_lock(
            self.pin_mut(),
            crate::sys::rust_closure_new(callback),
        );
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard};

    static LOCK: Mutex<()> = Mutex::new(());

    /// Serialises the tests that touch the process-wide tools pointer.
    ///
    /// A `should_panic` test poisons the lock; the guarded state is still sound.
    pub(crate) fn lock() -> MutexGuard<'static, ()> {
        LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// A non-null address that is never dereferenced — the tests only exercise
    /// the availability bookkeeping, never an FFI call.
    pub(crate) fn dummy_ptr() -> *mut crate::sys::ffi::MiralTools {
        std::ptr::NonNull::<crate::sys::ffi::MiralTools>::dangling().as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_and_uninstall_toggle_availability() {
        let _guard = test_support::lock();

        assert!(
            !WindowManagerTools::is_available(),
            "no server is running in tests"
        );

        // Safety: the pointer is never dereferenced — it is uninstalled below and
        // no tools method is called while it is installed.
        unsafe { WindowManagerTools::install(test_support::dummy_ptr()) };
        assert!(WindowManagerTools::is_available());

        WindowManagerTools::uninstall();
        assert!(
            !WindowManagerTools::is_available(),
            "handles must be invalidated once the server stops"
        );
    }

    #[test]
    fn handles_are_interchangeable() {
        assert_eq!(
            WindowManagerTools::current(),
            *WindowManagerTools::global_ref()
        );
        assert_eq!(WindowManagerTools::current(), WindowManagerTools::default());
    }

    #[test]
    #[should_panic(expected = "WindowManagerTools not available")]
    fn use_without_a_server_panics() {
        let _guard = test_support::lock();
        WindowManagerTools::current().as_ref();
    }
}

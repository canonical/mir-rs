/*
 * Copyright © Canonical Ltd.
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 or 3,
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Idiomatic Rust API for building Wayland compositors with the Mir display server.
//!
//! This crate provides safe, ergonomic abstractions over the battle-tested miral C++ library.
//! Compositor authors implement the [`policy::WindowManagementPolicy`] trait to define custom
//! window management behavior, then use [`runner::MirRunner`] to start the compositor.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use mir::prelude::*;
//!
//! #[derive(Default)]
//! struct MyPolicy;
//!
//! impl WindowManagementPolicy for MyPolicy {}
//!
//! fn main() {
//!     MirRunner::new(std::env::args())
//!         .add(Decorations::prefer_csd())
//!         .add_window_management_policy::<MyPolicy>()
//!         .run()
//!         .expect("Server failed");
//! }
//! ```

#![deny(missing_docs)]
// The quick-start example above is a whole compositor binary, so it keeps its
// `fn main` even though rustdoc would wrap the body in one for us.
#![allow(clippy::needless_doctest_main)]

pub mod application;
pub mod client;
pub mod configuration;
pub mod extensions;
pub mod geometry;
pub mod input;
pub mod output;
pub mod policy;
pub mod runner;
mod sys;
pub mod window;
pub mod workspace;

/// Context passed to [`extensions::ServerExtension`] implementations.
///
/// The underlying FFI module remains private; this opaque re-export makes the
/// context nameable by downstream extension implementations.
#[doc(hidden)]
pub use sys::ffi::MiralRunner as ExtensionContext;

/// Convenience re-exports for the most commonly used types.
///
/// Import with `use mir::prelude::*` to get everything needed for a basic compositor.
pub mod prelude {
    pub use crate::application::{Application, ApplicationInfo};
    pub use crate::client::ExternalClientLauncher;
    pub use crate::configuration::ConfigurationOption;
    pub use crate::extensions::{
        AddInitCallback, BounceKeys, CursorScale, CursorTheme, Decorations, DisplayConfiguration,
        IdleListener, InputConfiguration, Keymap, LocatePointer, Magnifier, MouseKeysConfig,
        OutputFilter, ServerExtension, SessionLockListener, SetTerminator, SlowKeys, StickyKeys,
        WaylandExtensions, X11Support,
    };
    pub use crate::geometry::{
        Displacement, DisplacementF, Point, PointF, Rectangle, RectangleF, Scalar, Size, SizeF,
    };
    pub use crate::input::{
        InputEvent, KeyAction, KeyboardEvent, PointerAction, PointerEvent, TouchEvent,
    };
    pub use crate::output::{
        FormFactor, Orientation, Output, OutputType, PhysicalSizeMM, PowerMode, Zone,
    };
    pub use crate::policy::{Advice, WindowManagementPolicy, WindowManagerTools};
    pub use crate::runner::{MirRunner, RunnerHandle};
    pub use crate::window::{
        AspectRatio, DepthLayer, FocusMode, FocusStealing, InputReceptionMode, OrientationMode,
        PlacementGravity, PlacementHints, PointerConfinementState, ResizeEdge, ShellChrome,
        TiledEdges, Window, WindowInfo, WindowSpecification, WindowState, WindowType,
    };
    pub use crate::workspace::Workspace;

    /// **Experimental.** Re-exported so `WindowSpecification::with_transform` can
    /// be called via `use mir::prelude::*`. Gated behind the `experimental`
    /// feature; see [`WindowSpecification::with_transform`](crate::window::WindowSpecification::with_transform).
    #[cfg(feature = "experimental")]
    pub use glam::Mat4;
}

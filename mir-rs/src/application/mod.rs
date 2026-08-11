//! Application types representing connected clients.

// `application::application` holds the `Application` handle itself; the name mirrors the
// public path.
#[allow(clippy::module_inception)]
mod application;
mod info;

pub use application::Application;
pub use info::ApplicationInfo;

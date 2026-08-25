//! Output filter configuration.

use super::ServerExtension;

use std::pin::Pin;

/// Enables standard output filter command-line options.
#[derive(Debug, Default, Clone, Copy)]
pub struct OutputFilter;

impl OutputFilter {
    /// Create an output filter extension.
    pub fn new() -> Self {
        Self
    }
}

impl ServerExtension for OutputFilter {
    fn name(&self) -> &str {
        "OutputFilter"
    }

    fn apply(self: Box<Self>, runner: Pin<&mut crate::sys::ffi::MiralRunner>) {
        crate::sys::ffi::miral_runner_add_output_filter(runner);
    }
}

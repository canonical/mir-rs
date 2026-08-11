# mir

Idiomatic Rust API for building Wayland compositors with the [Mir](https://github.com/canonical/mir) display server.

This crate lives in the `mir-rs/` directory of the
[mir-rs](https://github.com/canonical/mir-rs) workspace; see the
[workspace README](../README.md) for an overview of how it fits together with the
[`mir-sys`](../mir-rs-sys/README.md) bridge.

## System Requirements

This crate requires the **miral C++ library** to be installed on your system at build time and runtime.

### Ubuntu / Debian

```bash
sudo apt install libmiral-dev
```

### From Source

See the [Mir build instructions](https://github.com/canonical/mir/blob/main/HACKING.md) for building from source.

Afterwards, set your `LD_LIBRARY_PATH` to include the `miral` library's installation directory. For example:

```bash
export LD_LIBRARY_PATH=/usr/local/lib
```

## Quick Start

```rust
use mir::prelude::*;

#[derive(Default)]
struct MyPolicy {
    tools: WindowManagerTools,
}

impl WindowManagementPolicy for MyPolicy {
    fn tools(&self) -> &WindowManagerTools { &self.tools }
    fn tools_mut(&mut self) -> &mut WindowManagerTools { &mut self.tools }

    fn handle_keyboard_event(&mut self, event: &KeyboardEvent) -> bool {
        // Handle keyboard shortcuts here; return true to consume the event.
        false
    }

    fn advise(&mut self, event: Advice) {
        if let Advice::NewWindow { window_info } = event {
            println!("New window: {:?}", window_info.name());
        }
    }
}

fn main() {
    MirRunner::new(std::env::args())
        .add(WaylandExtensions::default())
        .add(Decorations::prefer_csd())
        .add_window_management_policy::<MyPolicy>()
        .run()
        .expect("Server failed");
}
```

## Architecture

This crate provides a safe, idiomatic Rust layer on top of the battle-tested miral C++ library:

```
┌─────────────────────────────────────────────┐
│  mir  (this crate - idiomatic Rust API)     │
├─────────────────────────────────────────────┤
│  mir-sys  (cxx.rs FFI + bindgen)            │
├─────────────────────────────────────────────┤
│  libmiral  (C++ library, system-installed)  │
└─────────────────────────────────────────────┘
```

## Key Concepts

- **`MirRunner`** — Manages the compositor lifecycle (builder pattern)
- **`WindowManagementPolicy`** — Trait for custom window management logic
- **`WindowManagerTools`** — API for querying and modifying windows
- **`Advice`** — Enum of lifecycle notifications (new window, focus change, etc.)
- **`WindowSpecification`** — Builder for window property changes
- **`ServerExtension`** — Trait implemented by everything passed to `MirRunner::add`

## Example

A complete tiling compositor lives in
[`examples/mir-rs-tiling`](examples/mir-rs-tiling/README.md):

```bash
WAYLAND_DISPLAY=wayland-98 cargo run -p mir-rs-tiling
```

## License

This project is licensed under the GNU General Public License version 2 or later.

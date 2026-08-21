# mir

Idiomatic Rust API for building Wayland compositors with the [Mir](https://github.com/canonical/mir) display server.

This crate lives in the `mir-rs/` directory of the
[mir-rs](https://github.com/canonical/mir-rs) workspace; see the
[workspace README](../README.md) for an overview. Its low-level FFI bridge is kept
internally under `src/sys/`.

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

### Experimental feature

The optional `experimental` cargo feature enables
[`WindowSpecification::with_transform`], which sets the 4×4 transform of the
`mir::scene::Surface` behind a window (as a [`glam`](https://crates.io/crates/glam)
`Mat4`, re-exported from the prelude). Enabling it makes the underlying `mir-sys`
crate **link against `mirserver`** (package `libmirserver-dev`), since that is the
only place `mir::scene::Surface::set_transformation` is declared.

This is a temporary **"necessary evil"** — the surface transform is expected to be
removed upstream, so both the API and its `mirserver` dependency will go away with
it. It is off by default:

```bash
cargo add mir --features experimental
```

[`WindowSpecification::with_transform`]: https://docs.rs/mir

## Quick Start

```rust
use mir::prelude::*;

#[derive(Default)]
struct MyPolicy;

impl WindowManagementPolicy for MyPolicy {
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
│  sys  (cxx.rs FFI + bindgen, internal)      │
├─────────────────────────────────────────────┤
│  libmiral  (C++ library, system-installed)  │
└─────────────────────────────────────────────┘
```

## Key Concepts

- **`MirRunner`** — Manages the compositor lifecycle (builder pattern)
- **`WindowManagementPolicy`** — Trait for custom window management logic
- **`WindowManagerTools`** — API for querying and modifying windows, always available to a policy via `self.tools()`
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

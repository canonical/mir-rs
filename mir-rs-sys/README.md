# mir-sys

Low-level FFI bindings to the [Mir](https://github.com/canonical/mir) compositor's **miral** C++ library.

This crate lives in the `mir-rs-sys/` directory of the
[mir-rs](https://github.com/canonical/mir-rs) workspace; see the
[workspace README](../README.md) for an overview, and the [`mir`](../mir-rs/README.md) crate
for the safe API built on top of this one.

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

## Usage

This crate is not intended for direct use by compositor authors. Instead, use the [`mir`](https://crates.io/crates/mir) crate which provides an idiomatic, safe Rust API.

If you need low-level access to the FFI layer:

```toml
[dependencies]
mir-sys = "0.1"
```

## How It Works

- **bindgen** generates Rust bindings for C-linkage enums from `mir_toolkit/common.h` and `mir_toolkit/events/enums.h`
- **cxx.rs** bridges C++ classes (`MirRunner`, `WindowManagerTools`, `WindowInfo`, etc.)
- **pkg-config** locates the system-installed `miral` library at build time

The C++ side of the bridge is hand-written in `src/bridge.h` and `src/bridge.cpp`; it flattens
`miral`'s templates, `std::optional` values and virtual policy classes into shapes cxx.rs can
express, passing windows across the boundary as stable `u64` IDs. Every function declared
there must be mirrored in the `#[cxx::bridge]` module in `src/lib.rs`.

Minimum library versions (enforced by `build.rs`, so a too-old system fails at configure time):
`miral` 6.0, `mircore` 2.29, `mircommon` 2.29.

## Experimental feature

The optional `experimental` cargo feature builds the surface-transform support used
by [`mir`](../mir-rs/README.md)'s `WindowSpecification::with_transform`. It calls
`mir::scene::Surface::set_transformation`, which is only declared in the
`mirserver-internal` headers, so enabling it makes `build.rs` additionally probe and
**link against `mirserver`** (package `libmirserver-dev`, minimum version 2.29) and
compile the transform code path under a `MIR_RS_EXPERIMENTAL` define.

This is a temporary **"necessary evil"**: the underlying surface transform is
expected to be removed from Mir upstream, so the feature and its `mirserver`
dependency are opt-in and off by default. The default build never touches
`mirserver`.

## License

This project is licensed under the GNU General Public License version 2 or later.

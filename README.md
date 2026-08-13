# mir-rs

Rust bindings for [Mir](https://github.com/canonical/mir), Canonical's display server
toolkit. `mir-rs` lets you write a Wayland compositor in Rust on top of Mir's `miral`
window-management library.

This repository is a Cargo workspace containing **two crates** plus a worked example.

| Directory                   | Crate name      | Role                                                         |
| --------------------------- | --------------- | ------------------------------------------------------------ |
| `mir-rs-sys/`               | `mir-sys`       | Low-level FFI bridge between C++ `miral` and Rust             |
| `mir-rs/`                   | `mir`           | The user-facing, safe, idiomatic Rust API                     |
| `mir-rs/examples/mir-rs-tiling/` | `mir-rs-tiling` | A working tiling compositor built with `mir` (not published) |

> **Note on naming:** the *directories* are named `mir-rs*` to describe the project, but the
> *crates* they publish are named `mir` and `mir-sys`. In `Cargo.toml` and in `use` statements
> you always refer to `mir` and `mir_sys`.

```
┌─────────────────────────────────────────────┐
│  your compositor  (e.g. mir-rs-tiling)      │
├─────────────────────────────────────────────┤
│  mir       — idiomatic, safe Rust API       │  mir-rs/
├─────────────────────────────────────────────┤
│  mir-sys   — cxx.rs bridge + bindgen enums  │  mir-rs-sys/
├─────────────────────────────────────────────┤
│  libmiral  — C++ library, system-installed  │
└─────────────────────────────────────────────┘
```

## Requirements

The bridge targets the miral 6.0 API and is built against the system-installed
Mir development libraries:

| Library     | Minimum version | Ubuntu/Debian package |
|-------------|-----------------|-----------------------|
| `miral`     | 6.0             | `libmiral-dev`        |
| `mircore`   | 2.29            | `libmircore-dev`      |
| `mircommon` | 2.29            | `libmircommon-dev`    |

```
sudo apt install libmiral-dev libmircore-dev libmircommon-dev
```

These minimums are enforced by `mir-rs-sys/build.rs` via `pkg-config`, so an
unsupported version fails at configure time with a clear message rather than
part-way through the C++ build.

A C++ toolchain is also required (the bridge compiles `mir-rs-sys/src/bridge.cpp`), and the
Rust toolchain must be at least **1.82.0** (edition 2021).

### Experimental features

The optional `experimental` cargo feature exposes
`WindowSpecification::with_transform`, which sets the 4×4 transform of the
`mir::scene::Surface` backing a window (using the [`glam`](https://crates.io/crates/glam)
`Mat4` type). Enabling it makes `mir-sys` **link against `mirserver`** (via the
`mirserver-internal` headers, package `libmirserver-dev`), because that is the only
place `mir::scene::Surface::set_transformation` is declared.

This is a deliberate, temporary **"necessary evil"**: the surface transform is
expected to be removed from Mir upstream, at which point the feature and its
`mirserver` dependency will disappear. It is therefore **off by default** and only
enabled explicitly:

```bash
cargo build -p mir --features experimental
```

If you built Mir from source rather than installing the packages, point the loader at it:


## Contributing

```bash
cargo build --workspace                              # build both crates and the example
cargo test --workspace                               # run the test suite
cargo clippy --workspace --all-targets -- -D warnings # lint (must be clean)
cargo fmt --all                                      # format
cargo doc -p mir --no-deps --open                    # browse the API docs
```

Contributor and agent rules live in [`AGENTS.md`](AGENTS.md).

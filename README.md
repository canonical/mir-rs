# mir-rs
Rust bindings for Mir (https://github.com/canonical/mir).

mir-rs-sys: Provides the bridge between mir/miral and the mir-rs Rust library
mir-rs: the user-facing Rust library for authoring Mir compositors in Rust

See mir-rs/examples/mir-rs-tiling for a working example.

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

# Repository rules

Rules for anyone — human or coding agent — working in `mir-rs`. Read this before changing
code. [`README.md`](README.md) explains what the two crates are and how they fit together.

## The short version

Before you consider a change done:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p mir-rs-tiling
```

All four must pass locally before you push. CI (`.github/workflows/ci.yml`) runs the same
gates against `ppa:mir-team/dev` on Ubuntu 26.04, plus a nightly scheduled rebuild against
whatever Mir is current in that PPA.

## Linting: clippy must always be clean

- `cargo clippy --workspace --all-targets -- -D warnings` is the gate. `--all-targets` matters:
  it lints tests and the example too, which plain `cargo clippy` does not.
- **Never** silence a lint with a crate-wide `#![allow(...)]`, and never disable a lint just to
  make the build quiet. Fix the code.
- A narrowly scoped `#[allow(...)]` on a single item is acceptable *only* when the lint is
  genuinely wrong for FFI-shaped or builder-shaped code, and **only with a comment explaining
  why**. The existing ones are the precedent to follow:
  - `clippy::mut_from_ref` on `WindowManagerTools::pin_mut` — the C++ object is owned by the
    server, so `&self` is correct and `&mut self` would only force `RefCell`s on users.
  - `clippy::should_implement_trait` on `MirRunner::add` — it is a consuming builder method,
    not `std::ops::Add`.
  - `clippy::module_inception` on `window::window` and `application::application` — the module
    names mirror the public type paths.
- Prefer the idiomatic fix over the allow: derive `Default` with `#[default]` rather than
  hand-writing the impl, implement `Default` rather than an inherent `default()`, and so on.
- Don't leave dead code behind an `#[allow(dead_code)]`. Either delete it or wire it up. If a
  helper exists but is unused, that is usually a sign a code path was never finished — check
  before deleting.

## Testing

- Add tests for anything that can be tested without a running compositor: enum conversions,
  `Default` values, builders, geometry arithmetic, extension configuration, ID round-trips.
  Most bugs in this codebase are in exactly these places (an early `ResizeEdge::from_raw`
  mapped sequential values onto what is actually a bitmask, and no test caught it).
- When you change or add a `from_raw`/`to_raw` conversion, add a round-trip test **and** a test
  for the out-of-range fallback. Verify the numeric values against the real C header
  (`/usr/include/mircore/mir_toolkit/common.h`) rather than assuming they are sequential.
- Unit tests live in a `#[cfg(test)] mod tests` at the bottom of the file they test.
- For code that calls into miral, use the seam pattern from `mir-rs/src/extensions/wayland.rs`:
  a `#[cfg(not(test))]` function that performs the real FFI call and a `#[cfg(test)]` twin that
  records its arguments, so the logic can be asserted without starting a server.
- Prefer tests that need no FFI at all, so `cargo test` stays fast and runnable.
- `cargo test --workspace` must pass. Don't `#[ignore]` a failing test to get it green.

## Formatting, MSRV and documentation

- `cargo fmt --all` before committing; `cargo fmt --all -- --check` must be clean.
- MSRV is **1.85.0**, edition 2024, declared once in the workspace manifest and inherited by
  each crate (`version.workspace = true`, etc.). Don't use newer language or std features, and
  don't hard-code the version in a member `Cargo.toml`.
- The `mir` crate is `#![deny(missing_docs)]` — every public item, including enum variants and
  struct fields, needs a doc comment. `cargo doc -p mir --no-deps` must build without warnings.
- Doc examples that would start a server must be ```` ```rust,no_run ```` (or `ignore` if they
  cannot compile standalone) so `cargo test` doesn't try to run a compositor.
- Keep the READMEs honest: if you change a public API that appears in `README.md`,
  `mir-rs/README.md` or the example's README, update it in the same
  change.

## Layering

- `mir-rs/src/sys` stays thin and unopinionated: it exposes miral, nothing more. No ergonomics, no
  policy, no clever Rust types.
- All safety, ergonomics and idiomatic API design belong in `mir`.
- **Never leak `crate::sys::ffi` types through `mir`'s public API.** Convert at the boundary
  (`From`/`Into` impls, `from_raw`/`to_raw`, `from_ffi`) and expose Rust types only.
- Adding a miral capability usually means touching four places, in this order:
  1. `mir-rs/src/sys/bridge.h` / `bridge.cpp` — the C++ shim.
  2. The `#[cxx::bridge]` module in `mir-rs/src/sys/mod.rs` — must mirror the shim exactly.
  3. The safe wrapper in the appropriate `mir-rs/src/` module.
  4. The `prelude` in `mir-rs/src/lib.rs`, if it is something compositor authors use directly.

## Unsafe and FFI

- Every `unsafe` block, `unsafe fn` and `unsafe impl` needs a `// Safety:` comment stating the
  invariant that makes it sound.
- The raw-pointer invariants for the policy live in `mir-rs/src/policy/tools.rs`: the pointer is
  published once the policy has been constructed, cleared when the policy adapter is dropped,
  and dispatch is single-threaded. Anything relying on those facts should say so.
- Windows cross the FFI boundary as stable `u64` IDs, not as C++ objects. Keep it that way.
- Assert rather than dereference blindly (`assert!(!raw.is_null(), ...)`) — a clear panic
  beats undefined behaviour.

## Dependencies and build

- Building requires the Mir development libraries: `libmiral-dev`, `libmircore-dev`,
  `libmircommon-dev` (miral ≥ 6.0, mircore/mircommon ≥ 2.29). `mir-rs/build.rs` enforces
  the minimums via `pkg-config` and fails with an actionable message.
- Add dependencies sparingly, and pin shared ones in `[workspace.dependencies]` so both crates
  agree. `cxx` and `cxx-build` versions must match.
- Don't commit `target/` or other build output.

## Licensing and file conventions

- The project is GPL-2.0-or-later. New source files carry the same Canonical GPL header as the
  existing ones — copy it verbatim from a neighbouring file.
- One concept per module; re-export the public names from the module's `mod.rs`.
- Keep `mir-rs/examples/mir-rs-tiling` compiling. It is the only end-to-end exercise of the API
  and doubles as its smoke test — if an API change breaks it, update the example in the same
  change.

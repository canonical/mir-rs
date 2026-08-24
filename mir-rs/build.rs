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

use std::env;
use std::path::PathBuf;

/// Minimum supported miral version.
///
/// The C++ bridge targets the miral 6.0 API: `miral::WindowSpecification` and
/// `miral::ConfigurationOption` use `std::optional` (not `mir::optional_value`),
/// `WindowManagementPolicy::handle_activate_window` replaced
/// `handle_raise_window`, the workspace advice methods take `std::span`, and
/// `miral::SetTerminator` was removed in favour of
/// `MirRunner::register_signal_handler`.
const MIRAL_MIN_VERSION: &str = "6.0";

/// Minimum supported mircore/mircommon version.
///
/// miral 6.0 is built against the mir 2.29 core libraries.
const MIR_MIN_VERSION: &str = "2.29";

/// Probe for a system library, reporting the underlying pkg-config error.
///
/// `pkg-config` distinguishes "not installed" from "installed but too old", so
/// the real error is included rather than assuming the library is missing.
fn probe(name: &str, min_version: &str, hint: &str) -> pkg_config::Library {
    pkg_config::Config::new()
        .atleast_version(min_version)
        .probe(name)
        .unwrap_or_else(|e| {
            panic!(
                "\n\n\
                ERROR: mir requires the {name} C++ library, version {min_version} or newer.\n\
                \n\
                pkg-config reported:\n\
                {e}\n\
                \n\
                {hint}\n\
                \n\
                For other systems, see: https://github.com/canonical/mir/blob/main/HACKING.md\n\
                \n"
            )
        })
}

fn main() {
    let miral = probe(
        "miral",
        MIRAL_MIN_VERSION,
        "On Ubuntu/Debian:\n\n    sudo apt install libmiral-dev",
    );

    let mir_core = probe(
        "mircore",
        MIR_MIN_VERSION,
        "On Ubuntu/Debian:\n\n    sudo apt install libmircore-dev",
    );

    let mir_common = probe(
        "mircommon",
        MIR_MIN_VERSION,
        "On Ubuntu/Debian:\n\n    sudo apt install libmircommon-dev",
    );

    // Gather include paths from pkg-config
    let include_paths: Vec<PathBuf> = miral
        .include_paths
        .iter()
        .chain(mir_core.include_paths.iter())
        .chain(mir_common.include_paths.iter())
        .cloned()
        .collect();

    // --- bindgen: Generate Rust bindings for C-linkage enums ---
    let mut bindgen_builder = bindgen::Builder::default()
        .header_contents(
            "bindgen_input.h",
            "#include <mir_toolkit/common.h>\n#include <mir_toolkit/events/enums.h>\n",
        )
        .allowlist_type("Mir.*")
        .allowlist_var("mir_.*")
        .rustified_enum("Mir.*")
        .derive_debug(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true);

    for path in &include_paths {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Failed to generate bindgen bindings for mir_toolkit enums");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("mir_enums.rs"))
        .expect("Failed to write bindgen output");

    // --- cxx-build: Build the C++ bridge ---
    let mut cxx_build = cxx_build::bridge("src/sys/mod.rs");

    for path in &include_paths {
        cxx_build.include(path);
    }

    // Add the src directory so generated CXX code can find bridge.h
    cxx_build.include("src/sys");

    // Experimental: the surface transform is applied by calling
    // `mir::scene::Surface::set_transformation`, which is only declared in the
    // `mirserver-internal` headers and needs `mirserver` linked in. This is an
    // opt-in "necessary evil" gated behind the `experimental` feature; the plain
    // build never touches mirserver.
    let experimental = env::var_os("CARGO_FEATURE_EXPERIMENTAL").is_some();
    let mut experimental_libs: Vec<String> = Vec::new();
    let mut experimental_link_paths: Vec<PathBuf> = Vec::new();
    if experimental {
        let mir_server = probe(
            "mirserver-internal",
            MIR_MIN_VERSION,
            "On Ubuntu/Debian:\n\n    sudo apt install libmirserver-dev",
        );
        for path in &mir_server.include_paths {
            cxx_build.include(path);
        }
        cxx_build.define("MIR_RS_EXPERIMENTAL", None);
        experimental_libs = mir_server.libs.clone();
        experimental_link_paths = mir_server.link_paths.clone();
    }

    cxx_build
        .file("src/sys/bridge.cpp")
        .std("c++23")
        .compile("mir_bridge");

    // Link against miral and its dependencies
    for lib in &miral.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    for path in &miral.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // Link against mircommon explicitly; miral's pkg-config lists mircore as a
    // Requires but not mircommon, so mircommon must be linked directly.
    for lib in &mir_common.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    for path in &mir_common.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // Link against mirserver only when the experimental transform API is enabled.
    for lib in &experimental_libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    for path in &experimental_link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/sys/mod.rs");
    println!("cargo:rerun-if-changed=src/sys/bridge.cpp");
    println!("cargo:rerun-if-changed=src/sys/bridge.h");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_EXPERIMENTAL");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR");
}

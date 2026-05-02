// SPDX-License-Identifier: CC0-1.0

//! Generate Rust bindings for the Bitcoin Core JSON-RPC API from its OpenRPC spec.
//!
//! Usage:
//!
//! ```text
//! btc-codegen <version>      # e.g. `30`, regenerates one version
//! btc-codegen all            # regenerates every spec under specs/
//! ```
//!
//! Output is written to `output/v{N}/{mod,types,options,methods}.rs` next to the binary
//! (relative to the manifest dir, so `cargo run` does the right thing).

use std::path::{Path, PathBuf};
use std::{env, fs, process};

use btc_codegen::generate;

fn main() {
    let mut args = env::args().skip(1);
    let version = args.next().unwrap_or_else(|| {
        eprintln!("usage: btc-codegen <version|all>");
        process::exit(2)
    });

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let specs_dir = manifest_dir.join("specs");
    let output_dir = manifest_dir.join("output");

    let versions = if version == "all" {
        list_versions(&specs_dir).unwrap_or_else(|e| die(&e))
    } else {
        vec![version]
    };

    if versions.is_empty() {
        die(&format!("no specs found under {}", specs_dir.display()));
    }

    for v in versions {
        let spec_path = match find_spec(&specs_dir, &v) {
            Some(p) => p,
            None => {
                eprintln!("[codegen] no spec for v{v}, skipping");
                continue;
            }
        };
        let target = output_dir.join(format!("v{v}"));
        if let Err(e) = generate(&spec_path, &target, &v) {
            die(&format!("[codegen] v{v}: {e}"));
        }
    }
}

fn die(msg: &str) -> ! {
    eprintln!("{msg}");
    process::exit(1)
}

fn find_spec(specs_dir: &Path, version: &str) -> Option<PathBuf> {
    let prefix = format!("v{version}_");
    let mut matches: Vec<_> = fs::read_dir(specs_dir)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|n| n.starts_with(&prefix) && n.ends_with("_openrpc.json"))
                .unwrap_or(false)
        })
        .map(|e| e.path())
        .collect();
    matches.sort();
    matches.into_iter().next()
}

fn list_versions(specs_dir: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let entries =
        fs::read_dir(specs_dir).map_err(|e| format!("read {}: {e}", specs_dir.display()))?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if !name.starts_with('v') || !name.ends_with("_openrpc.json") {
            continue;
        }
        // `v30_2_0_openrpc.json` → first underscore-separated token without the leading `v`.
        if let Some(major) = name[1..].split('_').next() {
            if !out.contains(&major.to_owned()) {
                out.push(major.to_owned());
            }
        }
    }
    out.sort();
    Ok(out)
}

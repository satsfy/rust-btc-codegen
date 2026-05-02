// SPDX-License-Identifier: CC0-1.0

//! Bitcoin Core OpenRPC → Rust bindings generator.
//!
//! Public surface: [`generate`], which reads a single spec file and writes a four-file module
//! tree (`mod.rs`, `types.rs`, `options.rs`, `methods.rs`) into the target directory.
//!
//! Everything else lives behind the binary. The library exists so `tests/` can call into the
//! same code path the CLI does.

mod codegen;
mod names;
mod spec;

use std::fs;
use std::path::Path;

use crate::spec::Spec;

/// Reads the OpenRPC spec at `spec_path` and writes the generated modules under `out_dir`.
///
/// `version` is the short Bitcoin Core version ("30", "28") used in module headers.
pub fn generate(
    spec_path: &Path,
    out_dir: &Path,
    version: &str,
) -> Result<Summary, String> {
    let raw = fs::read_to_string(spec_path)
        .map_err(|e| format!("read {}: {e}", spec_path.display()))?;
    let spec: Spec = serde_json::from_str(&raw)
        .map_err(|e| format!("parse {}: {e}", spec_path.display()))?;

    fs::create_dir_all(out_dir)
        .map_err(|e| format!("mkdir {}: {e}", out_dir.display()))?;

    let modules = codegen::lower(&spec);
    let summary = Summary {
        types: modules.types_count(),
        methods: modules.methods_count(),
        option_structs: modules.option_count(),
        out_dir: out_dir.display().to_string(),
    };
    modules.write(out_dir, version)?;

    println!(
        "[codegen] v{}: {} types, {} methods, {} option structs -> {}",
        version, summary.types, summary.methods, summary.option_structs, summary.out_dir
    );
    Ok(summary)
}

/// Counts returned by [`generate`] for human-readable feedback.
#[derive(Debug, Clone)]
pub struct Summary {
    pub types: usize,
    pub methods: usize,
    pub option_structs: usize,
    pub out_dir: String,
}

#!/usr/bin/env python3
"""Top-level codegen driver.

Reads an OpenRPC spec for a single Bitcoin Core version and writes a self-contained
versioned directory containing the four files consumed by `corepc-client`'s codegen
folder:

    output/v{N}/
    ├── mod.rs       — module re-exports + a pointer to the hand-written model layer
    ├── types.rs     — return types (one struct per RPC response shape)
    ├── options.rs   — `*Options` structs for RPCs that have any optional parameters
    └── methods.rs   — `impl<'a> Raw<'a> { ... }` blocks calling `call_raw`

The driver delegates schema-shape work to the two specialised modules:

* `codegen.py`      — return-type generator (already battle-tested).
* `methods_gen.py`  — method & options generator (new).

Why split the output into multiple files? It makes review tractable: a reviewer
auditing a new Core version sees one `methods.rs` diff for the call surface, one
`types.rs` for the response shapes, and one `options.rs` for the new optional-arg
surface. A single 5,000-line file would obscure all three.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import codegen
import methods_gen


def _build_types_rs(version: str, spec: dict) -> str:
    """Use `codegen.py`'s machinery to produce a single `types.rs` covering every category."""
    modules = codegen.generate_modules(spec["methods"])
    out = [
        "// SPDX-License-Identifier: CC0-1.0\n",
        "\n",
        f"//! Auto-generated return types for Bitcoin Core `{version}`.\n",
        "//!\n",
        "//! This file is produced by `rust-btc-codegen` and committed to the tree.\n",
        "//! **Do not edit by hand.** Re-run `just codegen` to regenerate.\n",
        "\n",
        "#![allow(non_camel_case_types, clippy::large_enum_variant)]\n",
        "\n",
        "use serde::{Deserialize, Serialize};\n",
    ]
    for mod in modules:
        if not mod.exports:
            continue
        out.append(f"\n// ---------- {mod.name} ----------\n")
        # Strip the per-module prelude (use serde, etc.) — we already imported above.
        body = mod.code
        body = body.split("use serde::{Deserialize, Serialize};\n", 1)[-1]
        out.append(body)
    return "".join(out)


def generate_version(spec_path: Path, out_dir: Path, version: str) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)

    with spec_path.open() as f:
        spec = json.load(f)

    # Types --------------------------------------------------------------------
    types_rs = _build_types_rs(version, spec)
    (out_dir / "types.rs").write_text(types_rs)

    # Methods & options --------------------------------------------------------
    methods = methods_gen.collect_methods_with_categories(spec)
    (out_dir / "options.rs").write_text(methods_gen.emit_options_rs(version, methods))
    (out_dir / "methods.rs").write_text(methods_gen.emit_methods_rs(version, methods))

    # Module entry point -------------------------------------------------------
    (out_dir / "mod.rs").write_text(methods_gen.emit_mod_rs(version))

    print(
        f"[codegen] v{version}: "
        f"{sum(1 for _ in (out_dir / 'types.rs').read_text().splitlines() if _.startswith('pub struct'))} types, "
        f"{len([m for m, _c in methods])} methods, "
        f"{len([m for m, _c in methods if m.optional_params()])} option structs "
        f"-> {out_dir}"
    )


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def _resolve_spec(specs_dir: Path, version: str) -> Path | None:
    matches = sorted(specs_dir.glob(f"v{version}_*_openrpc.json"))
    return matches[0] if matches else None


def main() -> int:
    ap = argparse.ArgumentParser(description="Generate Rust bindings from a Bitcoin Core OpenRPC spec.")
    ap.add_argument(
        "version",
        help="Bitcoin Core version to generate, e.g. `30`, or `all` to generate every spec in specs/.",
    )
    ap.add_argument("--specs-dir", type=Path, default=Path(__file__).parent / "specs")
    ap.add_argument("--output-dir", type=Path, default=Path(__file__).parent / "output")
    args = ap.parse_args()

    if args.version == "all":
        versions = sorted({p.name.split("_", 1)[0][1:] for p in args.specs_dir.glob("v*_openrpc.json")})
        if not versions:
            print(f"No specs found in {args.specs_dir}", file=sys.stderr)
            return 1
        print(f"[codegen] generating for versions: {', '.join(versions)}")
    else:
        versions = [args.version]

    for v in versions:
        spec = _resolve_spec(args.specs_dir, v)
        if spec is None:
            print(f"[codegen] no spec for v{v}, skipping", file=sys.stderr)
            continue
        generate_version(spec, args.output_dir / f"v{v}", v)
    return 0


if __name__ == "__main__":
    sys.exit(main())

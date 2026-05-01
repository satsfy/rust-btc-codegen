#!/usr/bin/env python3
"""Generate Rust method implementations and options structs from a Bitcoin Core OpenRPC spec.

Reads an OpenRPC JSON document and produces, per Bitcoin Core version:

* `methods.rs`  — `impl<'a> Raw<'a> { ... }` blocks calling `Client::call_raw`.
* `options.rs`  — one `*Options` struct per method that has any optional parameters,
                  with `Option<T>` fields and a `Default` impl.

The `types.rs` companion file (return types) is produced by `codegen.py` and consumed
here transparently — we only emit `use super::types::*;` and trust the names line up.

# Optional argument strategy

OpenRPC marks each parameter with `required: bool`. Bitcoin Core itself accepts a
JSON `null` for any unset optional positional parameter and falls back to the C++
default. We exploit that:

* Methods with **no** optional arguments → one fn `name(required...)`.
* Methods with **any** optional arguments → two fns:
    - `name(required...)` — sends only the required positional args, Core uses
      its own defaults.
    - `name_with(required..., opts: NameOptions)` — sends every positional slot
      including trailing optionals; unset fields serialise to JSON `null`, which
      Core treats identically to "use default".

This keeps the common path ergonomic and the full path explicit, without a
combinatorial explosion of overloads.
"""

from __future__ import annotations

import re
from dataclasses import dataclass

# Imports from the type-generation module so we share name-conversion logic.
import codegen as type_gen


_RUST_KW_PARAM = type_gen._RUST_KW
_FIELD_WORDS = type_gen._FIELD_WORDS


# ---------------------------------------------------------------------------
# Method-name conversion (snake_case)
# ---------------------------------------------------------------------------

def method_to_snake(name: str) -> str:
    """Convert an RPC method name (e.g. ``getblockheader``) to snake_case (``get_block_header``).

    Uses the same word list used for type names so naming stays consistent.
    """
    pascal = type_gen.method_to_pascal(name)
    # CamelCase → snake_case, lowercase.
    snake = re.sub(r"(?<!^)(?=[A-Z])", "_", pascal).lower()
    return _RUST_KW_PARAM.get(snake, snake)


def param_to_snake(name: str) -> str:
    """Map a JSON parameter name to a Rust identifier (snake_case, keyword-safe)."""
    s = type_gen.to_rust_field(name)
    return _RUST_KW_PARAM.get(s, s)


# ---------------------------------------------------------------------------
# Parameter type inference
# ---------------------------------------------------------------------------

def _param_type(schema: dict) -> str:
    """Infer the Rust type for a JSON-RPC parameter schema.

    Falls back to `serde_json::Value` for shapes that are too dynamic to type
    precisely (most commonly object-shaped argument bundles like
    `walletcreatefundedpsbt`'s `options`). Callers can still set those by
    constructing a `serde_json::Value` directly.
    """
    if schema.get("x-bitcoin-type") == "amount":
        return "f64"
    if schema.get("x-bitcoin-type") == "hex":
        return "String"

    if "oneOf" in schema or "anyOf" in schema:
        # Some `oneOf` cases (number-or-string for BTC amounts) are handled
        # above via x-bitcoin-type. Anything else stays as Value.
        return "serde_json::Value"

    t = schema.get("type")
    if t == "string":
        return "String"
    if t == "boolean":
        return "bool"
    if t == "integer":
        return "i64"
    if t == "number":
        # No x-bitcoin-type=amount → leave as f64 (numbers in Core's RPC params
        # are usually fee rates, conf counts, indices). Use f64 for safety;
        # bitcoind will accept integers in a number field.
        return "f64"
    if t == "array":
        items = schema.get("items")
        if isinstance(items, dict):
            inner = _param_type(items)
            return f"Vec<{inner}>"
        return "Vec<serde_json::Value>"
    if t == "object":
        return "serde_json::Value"
    return "serde_json::Value"


# ---------------------------------------------------------------------------
# Method/options model
# ---------------------------------------------------------------------------

@dataclass
class Param:
    name: str            # JSON name as it appears on the wire
    rust_name: str       # snake_case Rust ident (keyword-safe)
    rust_type: str       # rust type (no `Option<>` wrapping)
    required: bool
    description: str
    default: object      # for documentation only


@dataclass
class GenMethod:
    rpc_name: str         # JSON name (`getblockheader`)
    rust_name: str        # snake_case (`get_block_header`)
    summary: str
    description: str
    return_type: str      # already-generated Rust ident or `()` / `serde_json::Value`
    params: list[Param]

    def required_params(self) -> list[Param]:
        return [p for p in self.params if p.required]

    def optional_params(self) -> list[Param]:
        return [p for p in self.params if not p.required]

    def options_struct_name(self) -> str:
        return f"{type_gen.method_to_pascal(self.rpc_name)}Options"


# ---------------------------------------------------------------------------
# Return-type inference (mirror of codegen.Generator naming logic)
# ---------------------------------------------------------------------------

def _return_type_for(method: dict) -> str:
    """Return the Rust type the method should resolve to.

    The rules mirror `codegen.Generator.generate`:
    * `null` result → `()` (we let serde-null deserialise to unit).
    * `oneOf`/`anyOf` result → `serde_json::Value` (callers can deserialise into a
      typed variant from `super::types` themselves; verbose dispatch is hand-written).
    * Simple scalar (`string`, `boolean`, `number`, `integer`) → the wrapper
      newtype emitted by `codegen.py` (e.g. `GetBlockCount`).
    * Object/array → the same PascalCase name used by `codegen.py`.
    """
    schema = method["result"]["schema"]
    if type_gen._returns_null(method):
        return "()"
    if "oneOf" in schema or "anyOf" in schema:
        return "serde_json::Value"
    return type_gen.method_to_pascal(method["name"])


def parse_method(method: dict) -> GenMethod:
    params: list[Param] = []
    for raw in method.get("params") or []:
        s = raw.get("schema") or {}
        params.append(Param(
            name=raw["name"],
            rust_name=param_to_snake(raw["name"]),
            rust_type=_param_type(s),
            required=bool(raw.get("required", False)),
            description=(s.get("description") or raw.get("description") or "").strip(),
            default=s.get("default"),
        ))
    return GenMethod(
        rpc_name=method["name"],
        rust_name=method_to_snake(method["name"]),
        summary=(method.get("summary") or "").strip(),
        description=(method.get("description") or "").strip(),
        return_type=_return_type_for(method),
        params=params,
    )


# ---------------------------------------------------------------------------
# Code emission
# ---------------------------------------------------------------------------

def _doc_block(*lines: str, indent: str = "") -> str:
    out = []
    for ln in lines:
        if not ln:
            out.append(f"{indent}///")
            continue
        for sub in ln.splitlines() or [""]:
            out.append(f"{indent}/// {sub}".rstrip())
    return "\n".join(out) + ("\n" if out else "")


def _format_default(default) -> str:
    if default is None:
        return "no default"
    if isinstance(default, str):
        return repr(default)
    return str(default)


def _emit_options_struct(m: GenMethod) -> str:
    """Emit the `*Options` struct for a method with at least one optional param."""
    name = m.options_struct_name()
    lines = [
        _doc_block(
            f"Optional parameters for the [`{m.rpc_name}`] JSON-RPC method.",
            "",
            "Every field is `None` by default; setting a field to `Some(_)`"
            " causes the value to be sent as the corresponding positional argument."
            " Unset fields are serialised as JSON `null`, which Bitcoin Core treats"
            " as 'use the documented default'.",
            "",
            f"[`{m.rpc_name}`]: ../methods/struct.Raw.html#method.{m.rust_name}_with",
        ),
        "#[derive(Clone, Debug, Default, serde::Serialize)]",
        "#[serde(rename_all = \"camelCase\")]",
        f"pub struct {name} {{",
    ]
    for p in m.optional_params():
        lines.append(_doc_block(
            p.description or "",
            f"Default in Bitcoin Core: `{_format_default(p.default)}`.",
            indent="    ",
        ))
        rename = p.name
        if rename != p.rust_name:
            lines.append(f'    #[serde(rename = "{rename}")]')
        lines.append(f"    pub {p.rust_name}: Option<{p.rust_type}>,")
    lines.append("}")
    return "\n".join(lines) + "\n"


def _params_array_required_only(m: GenMethod) -> str:
    if not m.required_params():
        return "&[(); 0] as &[()]"
    refs = ", ".join(f"json!({p.rust_name})" for p in m.required_params())
    return f"&[{refs}]"


def _params_array_with_opts(m: GenMethod) -> str:
    parts = [f"json!({p.rust_name})" for p in m.required_params()]
    parts.extend(f"json!(opts.{p.rust_name})" for p in m.optional_params())
    if not parts:
        return "&[(); 0] as &[()]"
    return "&[" + ", ".join(parts) + "]"


def _emit_method(m: GenMethod) -> str:
    """Emit method(s) for a single RPC. Always emits the bare form; emits a `_with`
    form too when there is at least one optional parameter."""
    out: list[str] = []

    # Bare (required-only) form.
    bare_params = ", ".join(f"{p.rust_name}: {p.rust_type}" for p in m.required_params())
    bare_sig_args = ("&self, " + bare_params) if bare_params else "&self"
    bare_call = _params_array_required_only(m)

    out.append(_doc_block(
        f"`{m.rpc_name}` — required arguments only.",
        "",
        m.summary or "",
        ("\n" + m.description) if m.description else "",
        indent="    ",
    ))
    out.append(f"    pub async fn {m.rust_name}({bare_sig_args}) -> Result<{m.return_type}> {{")
    out.append(f'        self.client.call_raw("{m.rpc_name}", {bare_call}).await')
    out.append("    }")

    if m.optional_params():
        opts_name = m.options_struct_name()
        with_params = ", ".join(f"{p.rust_name}: {p.rust_type}" for p in m.required_params())
        sep = ", " if with_params else ""
        with_sig = f"&self, {with_params}{sep}opts: {opts_name}"
        with_call = _params_array_with_opts(m)

        out.append("")
        out.append(_doc_block(
            f"`{m.rpc_name}` — with all optional arguments via [`{opts_name}`].",
            "",
            m.summary or "",
            indent="    ",
        ))
        out.append(f"    pub async fn {m.rust_name}_with({with_sig}) -> Result<{m.return_type}> {{")
        out.append(f'        self.client.call_raw("{m.rpc_name}", {with_call}).await')
        out.append("    }")

    return "\n".join(out) + "\n"


# ---------------------------------------------------------------------------
# File-level emitters
# ---------------------------------------------------------------------------

_METHODS_PRELUDE = """\
// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method implementations for Bitcoin Core `{version}`.
//!
//! This file is produced by `rust-btc-codegen` and committed to the tree. **Do not edit by hand.**
//! Re-run `just codegen` to regenerate. Hand-written model wrappers live in
//! `client_async/model/`; this module is the raw, version-specific surface.

#![allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde_json::json;

use super::options::*;
use super::types::*;
use crate::client_async::error::Result;
use crate::client_async::raw::Raw;

impl<'a> Raw<'a> {{
"""

_OPTIONS_PRELUDE = """\
// SPDX-License-Identifier: CC0-1.0

//! Auto-generated options structs for Bitcoin Core `{version}`.
//!
//! This file is produced by `rust-btc-codegen` and committed to the tree. **Do not edit by hand.**
//! Re-run `just codegen` to regenerate.

#![allow(non_snake_case)]
"""

_MOD_PRELUDE = """\
// SPDX-License-Identifier: CC0-1.0

//! Auto-generated bindings for Bitcoin Core `{version}`.
//!
//! Generated by `rust-btc-codegen`. **Do not edit any file in this module by hand.**
//! Re-run `just codegen` from the workspace root to regenerate.
//!
//! # Layout
//!
//! * [`types`]   — return types for every RPC (raw, version-specific shape).
//! * [`options`] — `*Options` structs for methods with optional positional arguments.
//! * [`methods`] — `impl Raw<'_>` blocks calling `Client::call_raw`.
//!
//! Hand-written `into_model()`-style wrappers live in `client_async::model::v{version}`,
//! which is *not* part of this module.

pub mod methods;
pub mod options;
pub mod types;

pub use options::*;
pub use types::*;
"""


def emit_methods_rs(version: str, methods: list[tuple[GenMethod, str]]) -> str:
    body = []
    by_cat: dict[str, list[GenMethod]] = {}
    for m, cat in methods:
        by_cat.setdefault(cat, []).append(m)

    for cat in sorted(by_cat):
        body.append(f"\n    // ---------- {cat} ----------\n")
        for m in by_cat[cat]:
            body.append(_emit_method(m))
            body.append("\n")
    return _METHODS_PRELUDE.format(version=version) + "".join(body) + "}\n"


def emit_options_rs(version: str, methods: list[tuple[GenMethod, str]]) -> str:
    parts = [_OPTIONS_PRELUDE.format(version=version)]
    for m, _cat in methods:
        if m.optional_params():
            parts.append("\n")
            parts.append(_emit_options_struct(m))
    return "".join(parts)


def emit_mod_rs(version: str) -> str:
    return _MOD_PRELUDE.format(version=version)


# ---------------------------------------------------------------------------
# Public driver
# ---------------------------------------------------------------------------

def collect_methods_with_categories(spec: dict) -> list[tuple[GenMethod, str]]:
    out: list[tuple[GenMethod, str]] = []
    for raw in spec["methods"]:
        cat = raw.get("x-bitcoin-category", "misc")
        out.append((parse_method(raw), cat))
    return out

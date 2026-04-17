#!/usr/bin/env python3
"""Generate Rust type definitions from a Bitcoin Core OpenRPC specification."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

# -- Word lists for name conversion ----------------------------------------
# Longest-first so "addresses" beats "address", etc.

_METHOD_WORDS = sorted([
    "abandon", "abort", "add", "address", "addresses", "addrman", "all",
    "analyze", "ancestors", "api", "backup", "balance", "balances", "banned",
    "best", "bip125", "blockchain", "block", "blocks", "bump", "chain",
    "change", "clear", "combine", "connection", "control", "convert", "count",
    "create", "decode", "delete", "deployment", "derive", "descendants",
    "descriptor", "descriptors", "difficulty", "dir", "disconnect", "display",
    "dump", "echo", "encrypt", "entry", "enumerate", "estimate", "fee",
    "filter", "finalize", "for", "from", "fund", "funded", "generate", "get",
    "group", "groupings", "hash", "hashps", "hd", "header", "height", "help",
    "hex", "import", "index", "info", "ipc", "join", "json", "key", "keys",
    "label", "labels", "list", "load", "loaded", "lock", "logging", "mempool",
    "memory", "message", "mining", "multisig", "net", "network", "new",
    "node", "orphan", "out", "outset", "package", "passphrase", "peer",
    "peers", "pool", "precious", "prioritise", "prioritised", "priv",
    "process", "proof", "prune", "psbt", "psbts", "raw", "received",
    "recipient", "rescan", "restore", "rpc", "save", "scan", "script",
    "send", "set", "sign", "signer", "signers", "simulate", "since", "smart",
    "spending", "stats", "status", "stop", "submit", "test", "tips", "to",
    "totals", "transactions", "transaction", "txout", "txs", "tx", "unload",
    "unlock", "unspent", "update", "upgrade", "uptime", "utxo", "validate",
    "verify", "wait", "wallet", "wallets", "zmq", "activity",
], key=lambda w: -len(w))

_FIELD_WORDS = sorted([
    "chainwork", "mediantime", "nchaintx",
    "addresses", "address", "amounts", "amount", "ancestors", "ancestor",
    "automatic", "balances", "balance", "banned", "bare", "bestblock",
    "blocks", "block", "bytes", "carrier", "chainstates", "chainstate",
    "chains", "chain", "challenge", "changes", "change", "coins", "coin",
    "confirmations", "conf", "connections", "counts", "count", "current",
    "data", "deltas", "delta", "depths", "depth", "descendants", "descendant",
    "descriptors", "descriptor", "desc", "difficulty", "disk", "download",
    "effective", "entries", "entry", "errors", "error", "estimated",
    "feerates", "feerate", "fees", "fee", "filters", "filter", "final",
    "first", "full", "hashps", "hash", "hd", "headers", "header", "heights",
    "height", "hex", "included", "incremental", "index", "info", "initial",
    "internal", "keys", "key", "labels", "label", "last", "limits", "limit",
    "loaded", "locktimes", "locktime", "locks", "lock", "log", "max",
    "mempool", "messages", "message", "min", "mining", "modified", "multisig",
    "names", "name", "networks", "network", "new", "next", "nodes", "node",
    "nonce", "num", "only", "orphans", "orphan", "outputs", "output",
    "outset", "paths", "path", "peers", "peer", "permit", "pooled",
    "previous", "prev", "prioritised", "priority", "processed", "progress",
    "proofs", "proof", "pruned", "pruneheight", "pruning", "prune",
    "pubkeys", "pubkey", "raw", "rbf", "reachable", "received", "relay",
    "required", "results", "result", "scanning", "scan", "scripts", "script",
    "sequences", "sequence", "signet", "sizes", "size", "spending", "start",
    "states", "state", "stats", "status", "stripped", "success", "targets",
    "target", "times", "time", "tips", "totals", "total", "transactions",
    "transaction", "txids", "txid", "txout", "txs", "tx", "types", "type_",
    "type", "unbroadcast", "unconfirmed", "unlock", "unspent", "used",
    "utxos", "utxo", "values", "value", "verification", "versions",
    "version", "vsize", "wallets", "wallet", "warnings", "watch", "weights",
    "weight", "witness", "work", "written", "out", "accept",
], key=lambda w: -len(w))

_RUST_KW = {"type": "type_", "match": "match_", "ref": "ref_", "self": "self_",
            "mod": "mod_", "async": "async_", "await": "await_", "use": "use_"}


# -- Name conversion -------------------------------------------------------

def _greedy_split(name: str, words: list[str]) -> list[str]:
    """Split `name` into known words using greedy longest-match."""
    parts, remaining = [], name.lower()
    while remaining:
        for w in words:
            if remaining.startswith(w):
                parts.append(w)
                remaining = remaining[len(w):]
                break
        else:
            if parts:
                parts.append(remaining)
            else:
                return [remaining]
            break
    return parts


def method_to_pascal(name: str) -> str:
    """Convert RPC method name to PascalCase, matching original Rust codegen behavior.

    Known words get capitalized. Unmatched trailing characters are appended
    as-is (lowercase), except the very first character which is always upper.
    """
    result: list[str] = []
    remaining = name.lower()
    while remaining:
        matched = False
        for w in _METHOD_WORDS:
            if remaining.startswith(w):
                result.append(w[0].upper() + w[1:])
                remaining = remaining[len(w):]
                matched = True
                break
        if not matched:
            ch = remaining[0]
            if not result:
                result.append(ch.upper())
            else:
                result.append(ch)
            remaining = remaining[1:]
    return "".join(result)


def to_rust_field(name: str) -> str:
    name = name.replace("-", "_")
    if "_" in name:
        return _RUST_KW.get(name.lower(), name.lower())
    decameled = re.sub(r"([a-z])([A-Z])", r"\1_\2", name).lower()
    if "_" in decameled:
        return _RUST_KW.get(decameled, decameled)
    snake = "_".join(_greedy_split(name, _FIELD_WORDS))
    return _RUST_KW.get(snake, snake)


def _to_pascal(s: str) -> str:
    s = s.replace("-", "_")
    s = re.sub(r"([a-z])([A-Z])", r"\1_\2", s)
    return "".join(p.capitalize() for p in s.split("_") if p)


# -- Data model -------------------------------------------------------------

@dataclass
class GenType:
    name: str
    definition: str
    nested: list[GenType] = field(default_factory=list)


@dataclass
class MethodInfo:
    name: str
    struct_name: str | None
    notes: str


@dataclass
class Module:
    name: str
    code: str
    exports: list[str]
    methods: list[MethodInfo]


CATEGORY_MODULE = {
    "blockchain": "blockchain", "control": "control", "generating": "generating",
    "hidden": "hidden", "mining": "mining", "network": "network",
    "rawtransactions": "raw_transactions", "signer": "signer",
    "util": "util", "wallet": "wallet", "zmq": "zmq",
}


# -- Schema helpers ----------------------------------------------------------

def _returns_null(m: dict) -> bool:
    return m["result"]["schema"].get("type") == "null"


def _is_simple(schema: dict) -> bool:
    return schema.get("type") in ("string", "boolean", "number", "integer") and not _has_props(schema)


def _is_dynamic(schema: dict) -> bool:
    return schema.get("x-bitcoin-object-dynamic", False)


def _has_props(schema: dict) -> bool:
    return any(isinstance(v, dict) for v in (schema.get("properties") or {}).values())


def _simple_type(schema: dict) -> str | None:
    return {"string": "String", "boolean": "bool", "number": "i64", "integer": "i64"}.get(schema.get("type"))


def _esc(s: str) -> str:
    return s.replace("\n", "\n/// ")


def _method_doc(method: dict) -> str:
    return (f'/// Result of the JSON-RPC method `{method["name"]}`.\n'
            f'///\n/// > {method["name"]}\n/// >\n'
            f'/// > {_esc(method["summary"])}')


_DERIVE = ("#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
           '#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]')


def _verbose_suffix(condition: str, index: int) -> str:
    """Map a verbose condition string to VerboseZero/One/Two/Three."""
    c = condition.lower()
    for level, patterns in [
        ("VerboseZero", [r"verbose\s*(?:=\s*(?:false|0)|is not set|is\s+false)",
                         r"verbosity\s*=\s*0"]),
        ("VerboseOne",  [r"verbose\s*(?:=\s*(?:true|1)|is set to\s*(?:true|1))",
                         r"verbosity\s*=\s*1"]),
        ("VerboseTwo",  [r"verbose\s*=\s*2", r"verbosity\s*=\s*2"]),
        ("VerboseThree", [r"verbose\s*=\s*3", r"verbosity\s*=\s*3"]),
    ]:
        if any(re.search(p, c) for p in patterns):
            return level
    return ["VerboseZero", "VerboseOne", "VerboseTwo", "VerboseThree", "VerboseFour"][min(index, 4)]


# -- Generator ---------------------------------------------------------------

class Generator:
    def __init__(self) -> None:
        self._seen: set[str] = set()

    def reset(self) -> None:
        self._seen.clear()

    def generate(self, method: dict) -> GenType | None:
        schema = method["result"]["schema"]
        name = method_to_pascal(method["name"])

        if _returns_null(method):
            return None
        if _is_simple(schema) and not _is_dynamic(schema):
            return self._wrapper(name, _simple_type(schema), _method_doc(method))
        for key in ("oneOf", "anyOf"):
            if key in schema:
                return self._one_of(name, schema[key], method.get("description"))
        if _is_dynamic(schema):
            return self._map(name, schema, _method_doc(method))
        if schema.get("type") == "array":
            return self._array(name, schema, _method_doc(method))
        return self._struct(name, schema, method.get("description"))

    # -- Wrapper: pub struct Foo(pub T); -------------------------------------

    def _wrapper(self, name: str, inner: str | None, doc: str | None) -> GenType | None:
        if not inner or name in self._seen:
            return None
        self._seen.add(name)
        d = f"{doc}\n" if doc else ""
        return GenType(name, f"{d}{_DERIVE}\npub struct {name}(pub {inner});\n")

    # -- Array: pub struct Foo(pub Vec<T>); ----------------------------------

    def _array(self, name: str, schema: dict, doc: str | None) -> GenType | None:
        if name in self._seen:
            return None
        item_type, nested = self._array_item(schema, name)
        self._seen.add(name)
        d = self._fmt_doc(doc)
        return GenType(name, f"{d}{_DERIVE}\npub struct {name}(pub Vec<{item_type}>);\n", nested)

    def _array_item(self, schema: dict, parent: str) -> tuple[str, list[GenType]]:
        items = schema.get("items")
        if items is None or isinstance(items, list):
            return ("serde_json::Value" if isinstance(items, list) else "String", [])
        if items.get("type") == "object" and "properties" in items:
            return self._inner(f"{parent}Item", items)
        return self._to_type(items, parent, "")

    # -- Map: pub struct Foo(pub BTreeMap<String, T>); -----------------------

    def _map(self, name: str, schema: dict, doc: str | None) -> GenType | None:
        if name in self._seen:
            return None
        self._seen.add(name)
        vt, nested = self._map_value(schema, name)
        d = self._fmt_doc(doc)
        desc = schema.get("description", "Map entries")
        return GenType(name,
            f"{d}{_DERIVE}\npub struct {name}(\n"
            f"    /// {desc}\n"
            f"    pub std::collections::BTreeMap<String, {vt}>,\n);\n", nested)

    def _map_value(self, schema: dict, parent: str) -> tuple[str, list[GenType]]:
        ap = schema.get("additionalProperties")
        if isinstance(ap, dict):
            suffix = "Item" if parent.endswith("Entry") else "Entry"
            return self._inner(f"{parent}{suffix}", ap)
        return ("serde_json::Value", [])

    # -- oneOf / anyOf -------------------------------------------------------

    def _one_of(self, name: str, variants: list[dict], doc: str | None) -> GenType | None:
        if len(variants) == 1:
            return self._dispatch(name, variants[0], doc)

        all_types: list[GenType] = []
        for i, v in enumerate(variants):
            cond = v.get("x-bitcoin-condition", "") or v.get("description", "")
            suffix = _verbose_suffix(cond, i) if " and " not in cond else _verbose_suffix("", i)
            gen = self._dispatch(f"{name}{suffix}", v, doc)
            if gen:
                all_types.append(gen)

        if not all_types:
            return None
        primary = all_types[0]
        for extra in all_types[1:]:
            primary.nested.extend(extra.nested)
            primary.nested.append(extra)
        return primary

    def _dispatch(self, name: str, schema: dict, doc: str | None) -> GenType | None:
        t = schema.get("type")
        if t == "object":
            return self._map(name, schema, doc) if _is_dynamic(schema) else self._struct(name, schema, doc)
        if t == "array":
            return self._array(name, schema, doc)
        return self._wrapper(name, _simple_type(schema), self._fmt_doc(doc).rstrip("\n") if doc else None)

    # -- Struct: pub struct Foo { ... } --------------------------------------

    def _struct(self, name: str, schema: dict, doc: str | None) -> GenType | None:
        if name in self._seen:
            return None
        self._seen.add(name)

        props = schema.get("properties") or {}
        required = set(schema.get("required") or [])
        fields, nested, commentary_raw = [], [], []

        for pname in sorted(props):
            pval = props[pname]
            if isinstance(pval, str):
                commentary_raw.append(pval)
                continue
            optional = pname not in required or pval.get("x-bitcoin-optional", False)
            ft, fn = self._to_type(pval, name, pname)
            nested.extend(fn)
            rname = to_rust_field(pname)
            rename = f'    #[serde(rename = "{pname}")]\n' if rname != pname else ""
            ftype = f"Option<{ft}>" if optional else ft
            fdoc = f"    /// {_esc(pval['description'])}\n" if pval.get("description") else ""
            fields.append(f"{fdoc}{rename}    pub {rname}: {ftype},")

        d = self._fmt_doc(doc)

        if not fields and any("decoderawtransaction" in t.lower() for t in commentary_raw):
            return GenType(name, f"{d}pub type {name} = DecodeRawTransaction;\n", nested)

        body = "\n".join(fields)
        return GenType(name, f"{d}{_DERIVE}\npub struct {name} {{\n{body}\n}}\n", nested)

    # -- Helpers -------------------------------------------------------------

    def _inner(self, name: str, schema: dict) -> tuple[str, list[GenType]]:
        if "properties" in schema:
            gen = self._struct(name, schema, schema.get("description"))
            if gen:
                return (name, [gen] + list(gen.nested))
        if _is_dynamic(schema):
            ap = schema.get("additionalProperties")
            if isinstance(ap, dict):
                it, n = self._inner(f"{name}Entry", ap)
                return (f"std::collections::BTreeMap<String, {it}>", n)
        return self._to_type(schema, name, "")

    def _to_type(self, schema: dict, parent: str, fname: str) -> tuple[str, list[GenType]]:
        if "oneOf" in schema or "anyOf" in schema:
            return ("serde_json::Value", [])
        if schema.get("x-bitcoin-type") == "hex":
            return ("String", [])
        if schema.get("x-bitcoin-type") == "amount":
            return ("f64", [])

        t = schema.get("type")
        if t == "string":  return ("String", [])
        if t == "boolean": return ("bool", [])
        if t in ("number", "integer"):
            return ("f64", []) if schema.get("x-bitcoin-type") == "amount" else ("i64", [])
        if t == "null":    return ("()", [])

        if t == "array":
            items = schema.get("items")
            if items is None or isinstance(items, list):
                return ("Vec<serde_json::Value>", [])
            if items.get("type") == "object" and "properties" in items:
                iname = f"{parent}{_to_pascal(fname)}Item"
                tn, n = self._inner(iname, items)
                return (f"Vec<{tn}>", n)
            it, n = self._to_type(items, parent, fname)
            return (f"Vec<{it}>", n)

        if t == "object":
            if _is_dynamic(schema) or (isinstance(schema.get("additionalProperties"), dict) and not _has_props(schema)):
                ap = schema.get("additionalProperties")
                if isinstance(ap, dict):
                    if ap.get("type") == "object" and "properties" in ap:
                        vt, n = self._inner(f"{parent}{_to_pascal(fname)}", ap)
                    else:
                        vt, n = self._to_type(ap, parent, fname)
                    return (f"std::collections::BTreeMap<String, {vt}>", n)
                return ("std::collections::BTreeMap<String, serde_json::Value>", [])
            if "properties" in schema:
                nname = f"{parent}{_to_pascal(fname)}"
                gen = self._struct(nname, schema, schema.get("description"))
                if gen:
                    return (nname, list(gen.nested) + [gen])
            return ("serde_json::Value", [])

        return ("serde_json::Value", [])

    def _fmt_doc(self, doc: str | None) -> str:
        if not doc:
            return ""
        return doc + "\n" if doc.startswith("///") else f"/// {_esc(doc)}\n"


# -- Module generation -------------------------------------------------------

def generate_modules(methods: list[dict]) -> list[Module]:
    by_cat: dict[str, list[dict]] = {}
    for m in methods:
        by_cat.setdefault(m.get("x-bitcoin-category", "misc"), []).append(m)

    gen = Generator()
    modules = []
    for cat in sorted(by_cat):
        gen.reset()
        modules.append(_gen_module(cat, by_cat[cat], gen))
    return modules


def _gen_module(category: str, methods: list[dict], gen: Generator) -> Module:
    mod_name = CATEGORY_MODULE.get(category, category)
    all_types, method_infos, exports = [], [], []

    for m in methods:
        schema = m["result"]["schema"]
        if _returns_null(m):
            mi = MethodInfo(m["name"], None, "returns nothing")
        elif _is_simple(schema):
            mi = MethodInfo(m["name"], None, f"returns {schema.get('type', '?')}")
        else:
            mi = MethodInfo(m["name"], method_to_pascal(m["name"]), "version + model")
        method_infos.append(mi)

        gt = gen.generate(m)
        if gt:
            exports.append(gt.name)
            for n in gt.nested:
                exports.append(n.name)
                all_types.append(n)
            all_types.append(gt)

    lines = [
        f"// SPDX-License-Identifier: CC0-1.0\n\n",
        f"//! The JSON-RPC API for Bitcoin Core - {category}.\n",
        f"//! Types for methods found under the `== {_to_pascal(category)} ==` section.\n",
        f"//! Auto-generated from OpenRPC specification.\n\n",
        f"use serde::{{Deserialize, Serialize}};\n\n",
    ]
    for gt in sorted(all_types, key=lambda g: g.name):
        lines.append(gt.definition + "\n")

    return Module(mod_name, "".join(lines), exports, method_infos)


def generate_mod_rs(modules: list[Module], version: str) -> str:
    lines = [f"""\
// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `{version}`
//!
//! Auto-generated from OpenRPC specification.

"""]
    for mod in modules:
        lines.append(f"//! <details>\n//! <summary> == {_to_pascal(mod.name)} == </summary>\n//!\n")
        lines.append("//! | Method | Returns |\n//! |:---|:---|\n")
        for mi in mod.methods:
            ret = f"`{mi.struct_name}`" if mi.struct_name else mi.notes
            lines.append(f"//! | `{mi.name}` | {ret} |\n")
        lines.append("//!\n//! </details>\n//!\n")

    lines.append("\n")
    for mod in modules:
        if mod.exports:
            lines.append(f"pub mod {mod.name};\n")
    lines.append("\n")
    for mod in modules:
        if mod.exports:
            lines.append(f"pub use self::{mod.name}::{{{', '.join(mod.exports)}}};\n")
    return "".join(lines)


# -- CLI ---------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description="Generate Rust types from OpenRPC spec")
    ap.add_argument("--input", "-i", required=True)
    ap.add_argument("--output", "-o", required=True)
    ap.add_argument("--core-version", "-c", required=True)
    ap.add_argument("--single-file", action="store_true")
    args = ap.parse_args()

    with open(args.input) as f:
        doc = json.load(f)

    modules = generate_modules(doc["methods"])
    os.makedirs(args.output, exist_ok=True)

    if args.single_file:
        combined = generate_mod_rs(modules, args.core_version)
        combined += "\n// ============ Generated Types ============\n\n"
        for mod in modules:
            if mod.exports:
                combined += f"\n// --- {mod.name} ---\n\n{mod.code}"
        Path(args.output, "generated.rs").write_text(combined)
    else:
        for mod in modules:
            if mod.exports:
                Path(args.output, f"{mod.name}.rs").write_text(mod.code)
        Path(args.output, "mod.rs").write_text(generate_mod_rs(modules, args.core_version))

    print(f"Generated {sum(len(m.exports) for m in modules)} types in {len(modules)} modules")
    return 0


if __name__ == "__main__":
    sys.exit(main())

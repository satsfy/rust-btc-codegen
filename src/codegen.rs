// SPDX-License-Identifier: CC0-1.0

//! Translate a parsed OpenRPC [`Spec`] into Rust source for the four output files.
//!
//! The translation walks each [`spec::Method`], producing up to three artefacts:
//!
//! * a return type (a `pub struct`, a `pub struct Foo(pub T);` newtype, or a `pub type` alias);
//! * an `*Options` struct if the method has any optional positional parameters;
//! * one `pub async fn` on `Raw<'a>`, with a paired `_with` form when there are options.
//!
//! Type generation is recursive (object-typed params spawn nested `Item`/`Entry` types) but
//! deliberately non-clever: the same shape is emitted twice by design across two methods, since
//! a deep dedup graph would obscure diffs more than it shrinks them.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::names::{method_to_pascal, method_to_snake, to_pascal, to_rust_field};
use crate::spec::{AdditionalProperties, Method, Param, Schema, Spec};

/// Standard derive line for every emitted type.
const DERIVES: &str =
    "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n\
     #[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]";

/// One emitted Rust definition; `nested` holds the helper types this definition depends on.
#[derive(Debug)]
struct GenType {
    name: String,
    body: String,
    nested: Vec<GenType>,
}

/// Output bundle: every type, every method, every options struct, ready to be written to disk.
pub struct Modules {
    types: Vec<GenType>,
    methods: Vec<MethodOut>,
}

#[derive(Debug)]
struct MethodOut {
    method_name: String, // wire name (`getblockheader`)
    snake: String,       // Rust fn name (`get_block_header`)
    pascal: String,      // Rust return-type ident (`GetBlockHeader`)
    summary: String,
    description: String,
    return_type: String,
    params: Vec<ParamOut>,
    category: String,
}

#[derive(Debug)]
struct ParamOut {
    rust_name: String,
    rust_type: String,
    description: String,
    required: bool,
    default: Option<Value>,
}

impl Modules {
    pub fn types_count(&self) -> usize { self.types.len() }
    pub fn methods_count(&self) -> usize { self.methods.len() }
    pub fn option_count(&self) -> usize {
        self.methods.iter().filter(|m| m.has_optional()).count()
    }

    pub fn write(&self, dir: &Path, version: &str) -> Result<(), String> {
        write_file(&dir.join("mod.rs"), &emit_mod_rs(version))?;
        write_file(&dir.join("types.rs"), &self.emit_types_rs(version))?;
        write_file(&dir.join("options.rs"), &self.emit_options_rs(version))?;
        write_file(&dir.join("methods.rs"), &self.emit_methods_rs(version))?;
        Ok(())
    }

    fn emit_types_rs(&self, version: &str) -> String {
        let mut s = String::with_capacity(64 * 1024);
        s.push_str(&format!(
            "// SPDX-License-Identifier: CC0-1.0\n\n\
             //! Auto-generated return types for Bitcoin Core `{version}`.\n//!\n\
             //! Produced by `rust-btc-codegen`. **Do not edit by hand** — re-run\n\
             //! `just codegen` to regenerate.\n\n\
             #![allow(non_camel_case_types, clippy::large_enum_variant)]\n\n\
             use serde::{{Deserialize, Serialize}};\n\n"
        ));
        let mut sorted: Vec<&GenType> = self.types.iter().collect();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        for ty in sorted {
            s.push_str(&ty.body);
            s.push('\n');
        }
        s
    }

    fn emit_options_rs(&self, version: &str) -> String {
        let mut s = String::with_capacity(16 * 1024);
        s.push_str(&format!(
            "// SPDX-License-Identifier: CC0-1.0\n\n\
             //! Auto-generated options structs for Bitcoin Core `{version}`.\n//!\n\
             //! Produced by `rust-btc-codegen`. **Do not edit by hand.**\n\n\
             #![allow(non_snake_case)]\n"
        ));
        for m in &self.methods {
            if m.has_optional() {
                s.push('\n');
                s.push_str(&emit_options_struct(m));
            }
        }
        s
    }

    fn emit_methods_rs(&self, version: &str) -> String {
        let mut s = String::with_capacity(96 * 1024);
        s.push_str(&format!(
            "// SPDX-License-Identifier: CC0-1.0\n\n\
             //! Auto-generated method implementations for Bitcoin Core `{version}`.\n//!\n\
             //! Produced by `rust-btc-codegen`. **Do not edit by hand** — re-run\n\
             //! `just codegen` to regenerate. Hand-written model wrappers live in\n\
             //! `client_async/model/`; this module is the raw, version-specific surface.\n\n\
             #![allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]\n\n\
             use serde_json::json;\n\n\
             use super::options::*;\n\
             use super::types::*;\n\
             use crate::client_async::error::Result;\n\
             use crate::client_async::raw::Raw;\n\n\
             impl<'a> Raw<'a> {{\n"
        ));
        // Group methods by category, sort categories alphabetically, methods within by wire name.
        let mut by_cat: std::collections::BTreeMap<&str, Vec<&MethodOut>> =
            std::collections::BTreeMap::new();
        for m in &self.methods {
            by_cat.entry(m.category.as_str()).or_default().push(m);
        }
        for (cat, mut ms) in by_cat {
            ms.sort_by(|a, b| a.method_name.cmp(&b.method_name));
            s.push_str(&format!("\n    // ---------- {cat} ----------\n"));
            for m in ms {
                s.push_str(&emit_method(m));
                s.push('\n');
            }
        }
        s.push_str("}\n");
        s
    }
}

impl MethodOut {
    fn has_optional(&self) -> bool { self.params.iter().any(|p| !p.required) }
    fn options_struct_name(&self) -> String { format!("{}Options", self.pascal) }
}

/// PascalCase names that collide with std/prelude items. The codegen rewrites these to a
/// non-colliding ident (`Send` → `SendResult`) for both the emitted type and the method's
/// `Result<...>` return-type expression.
const RESERVED_TYPE_NAMES: &[(&str, &str)] = &[
    ("Send", "SendResult"),
    ("Sync", "SyncResult"),
    ("Drop", "DropResult"),
    ("Box", "BoxResult"),
    ("Vec", "VecResult"),
    ("Option", "OptionResult"),
    ("Result", "ResultResponse"),
];

fn safe_type_name(pascal: &str) -> String {
    for &(needle, replacement) in RESERVED_TYPE_NAMES {
        if pascal == needle {
            return replacement.to_owned();
        }
    }
    pascal.to_owned()
}

/// Top-level lowering: produces every type and every method from the [`Spec`].
pub fn lower(spec: &Spec) -> Modules {
    let mut types: Vec<GenType> = Vec::new();
    let mut methods: Vec<MethodOut> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for m in &spec.methods {
        let pascal = safe_type_name(&method_to_pascal(&m.name));
        if let Some(gt) = generate_return_type(m, &mut seen) {
            collect(gt, &mut types);
        }
        methods.push(MethodOut {
            method_name: m.name.clone(),
            snake: method_to_snake(&m.name),
            pascal: pascal.clone(),
            summary: m.summary.trim().to_owned(),
            description: m.description.trim().to_owned(),
            return_type: return_type_ident(m),
            params: m
                .params
                .iter()
                .map(|p| ParamOut {
                    rust_name: to_rust_field(&p.name),
                    rust_type: param_type(&p.schema, Some(&p.name)),
                    description: param_description(p),
                    required: p.required,
                    default: p.schema.default.clone(),
                })
                .collect(),
            category: m.category.clone(),
        });
    }
    Modules { types, methods }
}

fn collect(gt: GenType, out: &mut Vec<GenType>) {
    for n in gt.nested {
        collect(n, out);
    }
    out.push(GenType { name: gt.name, body: gt.body, nested: Vec::new() });
}

// ---------------------------------------------------------------------------
// Return-type generation
// ---------------------------------------------------------------------------

/// Produce the Rust return-type identifier the method's `Result<>` wraps.
fn return_type_ident(method: &Method) -> String {
    let s = &method.result.schema;
    if s.returns_null() {
        return "()".to_owned();
    }
    if s.one_of.is_some() || s.any_of.is_some() {
        return "serde_json::Value".to_owned();
    }
    safe_type_name(&method_to_pascal(&method.name))
}

fn generate_return_type(method: &Method, seen: &mut BTreeSet<String>) -> Option<GenType> {
    let s = &method.result.schema;
    if s.returns_null() {
        return None;
    }
    let name = safe_type_name(&method_to_pascal(&method.name));
    let doc = method_doc(method);

    if s.is_simple() && !s.dynamic {
        return wrapper(&name, simple_type(s)?, Some(&doc), seen);
    }
    if let Some(variants) = s.one_of.as_ref().or(s.any_of.as_ref()) {
        return one_of(&name, variants, method.description.as_deref_opt(), seen);
    }
    if s.dynamic {
        return Some(map_type(&name, s, Some(&doc), seen));
    }
    if s.primary_kind() == Some("array") {
        return Some(array_type(&name, s, Some(&doc), seen));
    }
    Some(struct_type(&name, s, method.description.as_deref_opt(), seen))
}

trait OptStrExt {
    fn as_deref_opt(&self) -> Option<&str>;
}
impl OptStrExt for String {
    fn as_deref_opt(&self) -> Option<&str> {
        if self.is_empty() { None } else { Some(self.as_str()) }
    }
}

fn wrapper(
    name: &str,
    inner: &str,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
    if !seen.insert(name.to_owned()) {
        return None;
    }
    let mut body = fmt_doc(doc);
    body.push_str(DERIVES);
    body.push('\n');
    body.push_str(&format!("pub struct {name}(pub {inner});\n"));
    Some(GenType { name: name.to_owned(), body, nested: vec![] })
}

fn array_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    seen.insert(name.to_owned());
    let (item_ty, nested) = array_item(schema, name, seen);
    let mut body = fmt_doc(doc);
    body.push_str(DERIVES);
    body.push('\n');
    body.push_str(&format!("pub struct {name}(pub Vec<{item_ty}>);\n"));
    GenType { name: name.to_owned(), body, nested }
}

fn array_item(schema: &Schema, parent: &str, seen: &mut BTreeSet<String>)
    -> (String, Vec<GenType>)
{
    let Some(items) = schema.items.as_ref().and_then(|i| i.primary()) else {
        return ("String".to_owned(), vec![]);
    };
    if items.primary_kind() == Some("object") && items.properties.is_some() {
        return inner_type(&format!("{parent}Item"), items, seen);
    }
    schema_to_type(items, parent, "", seen)
}

fn map_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    seen.insert(name.to_owned());
    let (vt, nested) = map_value(schema, name, seen);
    let desc = schema.description.as_deref().unwrap_or("Map entries");
    let mut body = fmt_doc(doc);
    body.push_str(DERIVES);
    body.push_str(&format!(
        "\npub struct {name}(\n    /// {desc}\n    pub std::collections::BTreeMap<String, {vt}>,\n);\n"
    ));
    GenType { name: name.to_owned(), body, nested }
}

fn map_value(schema: &Schema, parent: &str, seen: &mut BTreeSet<String>) -> (String, Vec<GenType>) {
    if let Some(AdditionalProperties::Schema(ap)) = &schema.additional_properties {
        let suffix = if parent.ends_with("Entry") { "Item" } else { "Entry" };
        return inner_type(&format!("{parent}{suffix}"), ap, seen);
    }
    ("serde_json::Value".to_owned(), vec![])
}

fn one_of(
    name: &str,
    variants: &[Schema],
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
    if variants.len() == 1 {
        return dispatch(name, &variants[0], doc, seen);
    }
    let mut emitted: Vec<GenType> = Vec::new();
    for (i, v) in variants.iter().enumerate() {
        let cond = v.condition.clone().unwrap_or_else(|| v.description.clone().unwrap_or_default());
        let suffix = if cond.contains(" and ") {
            verbose_suffix("", i)
        } else {
            verbose_suffix(&cond, i)
        };
        if let Some(g) = dispatch(&format!("{name}{suffix}"), v, doc, seen) {
            emitted.push(g);
        }
    }
    if emitted.is_empty() {
        return None;
    }
    let mut primary = emitted.remove(0);
    for extra in emitted {
        primary.nested.extend(extra.nested);
        primary.nested.push(GenType { name: extra.name, body: extra.body, nested: vec![] });
    }
    Some(primary)
}

fn dispatch(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
    match schema.primary_kind() {
        Some("object") => Some(if schema.dynamic {
            map_type(name, schema, doc, seen)
        } else {
            struct_type(name, schema, doc, seen)
        }),
        Some("array") => Some(array_type(name, schema, doc, seen)),
        _ => wrapper(name, simple_type(schema)?, doc, seen),
    }
}

fn struct_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    if !seen.insert(name.to_owned()) {
        return GenType { name: name.to_owned(), body: String::new(), nested: vec![] };
    }
    let props_map = match &schema.properties {
        Some(m) => m,
        None => &serde_json::Map::new(),
    };
    let required: BTreeSet<&str> =
        schema.required.as_ref().map(|v| v.iter().map(String::as_str).collect()).unwrap_or_default();

    let mut field_lines: Vec<String> = Vec::new();
    let mut nested: Vec<GenType> = Vec::new();
    let mut commentary_only: Vec<&str> = Vec::new();

    let mut keys: Vec<&String> = props_map.keys().collect();
    keys.sort();
    for k in keys {
        let v = &props_map[k];
        if v.is_string() {
            commentary_only.push(v.as_str().unwrap());
            continue;
        }
        let Ok(field_schema) = serde_json::from_value::<Schema>(v.clone()) else {
            continue;
        };
        let optional = !required.contains(k.as_str()) || field_schema.bitcoin_optional;
        let (ty, nested_ty) = schema_to_type(&field_schema, name, k, seen);
        nested.extend(nested_ty);
        let rust_name = to_rust_field(k);
        let rename = if rust_name == *k {
            String::new()
        } else {
            format!("    #[serde(rename = \"{}\")]\n", k)
        };
        let final_ty = if optional { format!("Option<{ty}>") } else { ty };
        let fdoc = field_schema
            .description
            .as_deref()
            .filter(|s| !s.is_empty())
            .map(|d| format!("    /// {}\n", d.replace('\n', "\n    /// ")))
            .unwrap_or_default();
        field_lines.push(format!("{fdoc}{rename}    pub {rust_name}: {final_ty},"));
    }

    let header = fmt_doc(doc);

    if field_lines.is_empty()
        && commentary_only
            .iter()
            .any(|s| s.to_lowercase().contains("decoderawtransaction"))
    {
        let body = format!("{header}pub type {name} = DecodeRawTransaction;\n");
        return GenType { name: name.to_owned(), body, nested };
    }

    let body = if field_lines.is_empty() {
        format!("{header}{DERIVES}\npub struct {name} {{}}\n")
    } else {
        format!(
            "{header}{DERIVES}\npub struct {name} {{\n{}\n}}\n",
            field_lines.join("\n")
        )
    };
    GenType { name: name.to_owned(), body, nested }
}

fn inner_type(name: &str, schema: &Schema, seen: &mut BTreeSet<String>)
    -> (String, Vec<GenType>)
{
    if schema.properties.is_some() {
        let gt = struct_type(name, schema, schema.description.as_deref(), seen);
        let result_name = gt.name.clone();
        return (result_name, vec![gt]);
    }
    if schema.dynamic {
        if let Some(AdditionalProperties::Schema(ap)) = &schema.additional_properties {
            let (it, n) = inner_type(&format!("{name}Entry"), ap, seen);
            return (format!("std::collections::BTreeMap<String, {it}>"), n);
        }
    }
    schema_to_type(schema, name, "", seen)
}

fn schema_to_type(
    schema: &Schema,
    parent: &str,
    field: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if schema.one_of.is_some() || schema.any_of.is_some() {
        return ("serde_json::Value".to_owned(), vec![]);
    }
    if matches!(schema.bitcoin_type.as_deref(), Some("hex")) {
        return ("String".to_owned(), vec![]);
    }
    if matches!(schema.bitcoin_type.as_deref(), Some("amount")) {
        return ("f64".to_owned(), vec![]);
    }
    match schema.primary_kind() {
        Some("string") => ("String".to_owned(), vec![]),
        Some("boolean") => ("bool".to_owned(), vec![]),
        Some("integer") => ("i64".to_owned(), vec![]),
        Some("number") => ("i64".to_owned(), vec![]),
        Some("null") => ("()".to_owned(), vec![]),
        Some("array") => array_field(schema, parent, field, seen),
        Some("object") => object_field(schema, parent, field, seen),
        _ => ("serde_json::Value".to_owned(), vec![]),
    }
}

fn array_field(
    schema: &Schema,
    parent: &str,
    field: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    let Some(items) = schema.items.as_ref().and_then(|i| i.primary()) else {
        return ("Vec<serde_json::Value>".to_owned(), vec![]);
    };
    if items.primary_kind() == Some("object") && items.properties.is_some() {
        let iname = format!("{parent}{}Item", to_pascal(field));
        let (tn, n) = inner_type(&iname, items, seen);
        return (format!("Vec<{tn}>"), n);
    }
    let (it, n) = schema_to_type(items, parent, field, seen);
    (format!("Vec<{it}>"), n)
}

fn object_field(
    schema: &Schema,
    parent: &str,
    field: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    let dynamic_or_map = schema.dynamic
        || matches!(&schema.additional_properties, Some(AdditionalProperties::Schema(_)))
            && !schema.has_props();
    if dynamic_or_map {
        if let Some(AdditionalProperties::Schema(ap)) = &schema.additional_properties {
            let (vt, n) = if ap.primary_kind() == Some("object") && ap.properties.is_some() {
                inner_type(&format!("{parent}{}", to_pascal(field)), ap, seen)
            } else {
                schema_to_type(ap, parent, field, seen)
            };
            return (format!("std::collections::BTreeMap<String, {vt}>"), n);
        }
        return ("std::collections::BTreeMap<String, serde_json::Value>".to_owned(), vec![]);
    }
    if schema.properties.is_some() {
        let nname = format!("{parent}{}", to_pascal(field));
        let gt = struct_type(&nname, schema, schema.description.as_deref(), seen);
        return (nname, vec![gt]);
    }
    ("serde_json::Value".to_owned(), vec![])
}

fn simple_type(schema: &Schema) -> Option<&'static str> {
    Some(match schema.primary_kind()? {
        "string" => "String",
        "boolean" => "bool",
        "number" | "integer" => "i64",
        _ => return None,
    })
}

fn verbose_suffix(condition: &str, index: usize) -> String {
    let c = condition.to_ascii_lowercase();
    let patterns: &[(&str, &[&str])] = &[
        ("VerboseZero",  &["verbose=false", "verbose=0", "verbose is not set", "verbose is false", "verbosity=0"]),
        ("VerboseOne",   &["verbose=true",  "verbose=1", "verbose is set to true", "verbose is set to 1", "verbosity=1"]),
        ("VerboseTwo",   &["verbose=2", "verbosity=2"]),
        ("VerboseThree", &["verbose=3", "verbosity=3"]),
    ];
    let normalised = c.replace(' ', "");
    for (label, needles) in patterns {
        if needles.iter().any(|n| normalised.contains(&n.replace(' ', ""))) {
            return (*label).to_owned();
        }
    }
    ["VerboseZero", "VerboseOne", "VerboseTwo", "VerboseThree", "VerboseFour"]
        [index.min(4)]
    .to_owned()
}

// ---------------------------------------------------------------------------
// Method emission
// ---------------------------------------------------------------------------

fn emit_method(m: &MethodOut) -> String {
    let mut out = String::new();
    let req: Vec<&ParamOut> = m.params.iter().filter(|p| p.required).collect();
    let opt: Vec<&ParamOut> = m.params.iter().filter(|p| !p.required).collect();

    let req_args = req
        .iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_type))
        .collect::<Vec<_>>()
        .join(", ");
    let bare_sig = if req_args.is_empty() {
        "&self".to_owned()
    } else {
        format!("&self, {req_args}")
    };

    out.push_str(&doc_block(
        &[
            &format!("`{}` — required arguments only.", m.method_name),
            "",
            &m.summary,
            "",
            &dedup_description(&m.summary, &m.description),
        ],
        "    ",
    ));
    out.push_str(&format!(
        "    pub async fn {}({bare_sig}) -> Result<{}> {{\n",
        m.snake, m.return_type
    ));
    out.push_str(&format!(
        "        self.client.call_raw(\"{}\", {}).await\n    }}\n",
        m.method_name,
        params_array_required_only(&req)
    ));

    if !opt.is_empty() {
        let opts_name = m.options_struct_name();
        let with_args = if req_args.is_empty() {
            format!("&self, opts: {opts_name}")
        } else {
            format!("&self, {req_args}, opts: {opts_name}")
        };
        out.push('\n');
        out.push_str(&doc_block(
            &[
                &format!(
                    "`{}` — with all optional arguments via [`{opts_name}`].",
                    m.method_name
                ),
                "",
                &m.summary,
            ],
            "    ",
        ));
        out.push_str(&format!(
            "    pub async fn {}_with({with_args}) -> Result<{}> {{\n",
            m.snake, m.return_type
        ));
        out.push_str(&format!(
            "        self.client.call_raw(\"{}\", {}).await\n    }}\n",
            m.method_name,
            params_array_with_opts(&req, &opt)
        ));
    }

    out
}

fn params_array_required_only(req: &[&ParamOut]) -> String {
    if req.is_empty() {
        return "&[(); 0] as &[()]".to_owned();
    }
    let items: Vec<String> = req.iter().map(|p| format!("json!({})", p.rust_name)).collect();
    format!("&[{}]", items.join(", "))
}

fn params_array_with_opts(req: &[&ParamOut], opt: &[&ParamOut]) -> String {
    let mut items: Vec<String> = Vec::with_capacity(req.len() + opt.len());
    for p in req {
        items.push(format!("json!({})", p.rust_name));
    }
    for p in opt {
        items.push(format!("json!(opts.{})", p.rust_name));
    }
    if items.is_empty() {
        return "&[(); 0] as &[()]".to_owned();
    }
    format!("&[{}]", items.join(", "))
}

fn emit_options_struct(m: &MethodOut) -> String {
    let name = m.options_struct_name();
    let mut s = String::new();
    s.push_str(&doc_block(
        &[
            &format!("Optional parameters for the [`{}`] JSON-RPC method.", m.method_name),
            "",
            "Every field is `None` by default; setting a field to `Some(_)` causes the value to \
             be sent as the corresponding positional argument. Unset fields are serialised as \
             JSON `null`, which Bitcoin Core treats as 'use the documented default'.",
            "",
            &format!("[`{}`]: ../methods/struct.Raw.html#method.{}_with", m.method_name, m.snake),
        ],
        "",
    ));
    s.push_str("#[derive(Clone, Debug, Default, serde::Serialize)]\n");
    s.push_str("#[serde(rename_all = \"camelCase\")]\n");
    s.push_str(&format!("pub struct {name} {{\n"));
    for p in m.params.iter().filter(|p| !p.required) {
        s.push_str(&doc_block(
            &[
                &p.description,
                "",
                &format!("Default in Bitcoin Core: `{}`.", format_default(&p.default)),
            ],
            "    ",
        ));
        if p.rust_name != normalise_rename(&p.rust_name) {
            // never hit — placeholder for future rename support
        }
        // Only rename when the wire name differs from the Rust ident; we don't carry the wire
        // name through to the option struct because the Options struct never goes on the wire
        // directly — its fields are serialised positionally. So no rename attribute needed.
        s.push_str(&format!("    pub {}: Option<{}>,\n", p.rust_name, p.rust_type));
    }
    s.push_str("}\n");
    s
}

fn normalise_rename(s: &str) -> String { s.to_owned() }

fn format_default(default: &Option<Value>) -> String {
    match default {
        None => "no default".to_owned(),
        Some(v) => match v {
            Value::String(s) => format!("'{s}'"),
            other => other.to_string(),
        },
    }
}

fn param_description(p: &Param) -> String {
    if !p.schema.description.as_deref().unwrap_or("").is_empty() {
        return p.schema.description.clone().unwrap_or_default().trim().to_owned();
    }
    p.description.trim().to_owned()
}

/// Map a JSON-Schema parameter onto a Rust type for the method signature.
///
/// Bitcoin Core's spec liberally uses `type: number` for fields that are semantically integers
/// (block height, verbosity level, conf targets). [`INTEGER_PARAM_NAMES`] enumerates names where
/// `i64` is correct in spite of the spec; everything else stays `f64` for safety.
fn param_type(schema: &Schema, name: Option<&str>) -> String {
    if matches!(schema.bitcoin_type.as_deref(), Some("amount")) {
        return "f64".to_owned();
    }
    if matches!(schema.bitcoin_type.as_deref(), Some("hex")) {
        return "String".to_owned();
    }
    if schema.one_of.is_some() || schema.any_of.is_some() {
        return "serde_json::Value".to_owned();
    }
    match schema.primary_kind() {
        Some("string") => "String".to_owned(),
        Some("boolean") => "bool".to_owned(),
        Some("integer") => "i64".to_owned(),
        Some("number") => {
            // Heuristic: integer-looking default → i64.
            if let Some(Value::Number(n)) = &schema.default {
                if n.is_i64() || n.is_u64() {
                    return "i64".to_owned();
                }
            }
            if let Some(name) = name {
                if INTEGER_PARAM_NAMES
                    .iter()
                    .any(|n| n.eq_ignore_ascii_case(name))
                {
                    return "i64".to_owned();
                }
            }
            "f64".to_owned()
        }
        Some("array") => {
            let item = match schema.items.as_ref().and_then(|i| i.primary()) {
                Some(items) => param_type(items, None),
                None => "serde_json::Value".to_owned(),
            };
            format!("Vec<{item}>")
        }
        Some("object") => "serde_json::Value".to_owned(),
        _ => "serde_json::Value".to_owned(),
    }
}

/// Parameter names that are integer-only across the entire RPC surface despite Core's spec
/// declaring them as `type: number`.
pub static INTEGER_PARAM_NAMES: &[&str] = &[
    "height", "verbosity", "verbose", "minconf", "maxconf", "conf_target", "nblocks", "blocks",
    "count", "num_blocks", "n", "version", "locktime", "port", "timeout", "millis",
    "block_timeout", "node_id", "rescan_height", "start_height", "stop_height", "depth", "index",
    "nout", "vout", "skip", "nodeid", "id", "uid",
];

/// Drop `summary` from the start of `description` if it's verbatim there. Bitcoin Core's spec
/// duplicates the summary into the description for almost every method.
fn dedup_description(summary: &str, description: &str) -> String {
    if summary.is_empty() || description.is_empty() {
        return description.to_owned();
    }
    if let Some(stripped) = description.strip_prefix(summary) {
        return stripped.trim_start_matches('\n').to_owned();
    }
    description.to_owned()
}

// ---------------------------------------------------------------------------
// Doc-comment helpers
// ---------------------------------------------------------------------------

fn doc_block(lines: &[&str], indent: &str) -> String {
    // Drop trailing empty lines so we don't end with `///` followed by the item — clippy's
    // `empty_line_after_doc_comments` complains about that.
    let mut filtered: Vec<&str> = lines.to_vec();
    while filtered.last().map(|s| s.is_empty()).unwrap_or(false) {
        filtered.pop();
    }
    let mut out = String::new();
    for line in filtered {
        if line.is_empty() {
            out.push_str(indent);
            out.push_str("///\n");
            continue;
        }
        for sub in line.lines() {
            out.push_str(indent);
            out.push_str("/// ");
            out.push_str(&esc_doc(sub));
            out.push('\n');
        }
    }
    out
}

/// Format a multi-line doc string for use as a top-level type doc-comment. Returns either an
/// empty string (no doc) or a sequence of `/// ...` lines ending in `\n`.
fn fmt_doc(doc: Option<&str>) -> String {
    let raw = match doc {
        None | Some("") => return String::new(),
        Some(d) => d,
    };
    let mut s = String::with_capacity(raw.len() + 16);
    if raw.starts_with("///") {
        // Already a `///`-prefixed block (e.g. from `method_doc`); make sure every line is a
        // doc-comment line. We re-prefix every interior line that doesn't already start with
        // `///`. This keeps the output safe regardless of upstream provenance.
        for (i, line) in raw.lines().enumerate() {
            if i > 0 {
                s.push('\n');
            }
            if line.starts_with("///") {
                s.push_str(line);
            } else {
                s.push_str("/// ");
                s.push_str(line);
            }
        }
        s.push('\n');
        return s;
    }
    for (i, line) in raw.lines().enumerate() {
        if i > 0 {
            s.push('\n');
        }
        s.push_str("/// ");
        s.push_str(&esc_doc(line));
    }
    s.push('\n');
    s
}

fn method_doc(method: &Method) -> String {
    let summary_lines: Vec<String> = method.summary.lines().map(esc_doc).collect();
    let body = summary_lines.join("\n/// > ");
    format!(
        "/// Result of the JSON-RPC method `{}`.\n///\n/// > {}\n/// >\n/// > {}",
        method.name, method.name, body
    )
}

/// Escape characters that have special meaning in rustdoc/Markdown.
///
/// Today: escape angle brackets so Bitcoin Core's `<wallet name>` etc. don't get parsed as
/// HTML tags by rustdoc.
fn esc_doc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    let mut in_token = false;
    for ch in s.chars() {
        match ch {
            '<' if !in_token => {
                in_token = true;
                out.push_str("\\<");
            }
            '>' if in_token => {
                in_token = false;
                out.push_str("\\>");
            }
            '\n' => {
                out.push('\n');
            }
            _ => out.push(ch),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// File helpers
// ---------------------------------------------------------------------------

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("write {}: {e}", path.display()))
}

fn emit_mod_rs(version: &str) -> String {
    format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Auto-generated bindings for Bitcoin Core `{version}`.\n//!\n\
         //! Generated by `rust-btc-codegen`. **Do not edit any file in this module by hand.**\n\
         //! Re-run `just codegen` from the workspace root to regenerate.\n//!\n\
         //! # Layout\n//!\n\
         //! * [`types`]   — return types for every RPC (raw, version-specific shape).\n\
         //! * [`options`] — `*Options` structs for methods with optional positional arguments.\n\
         //! * [`methods`] — `impl Raw<'_>` blocks calling `Client::call_raw`.\n//!\n\
         //! Hand-written `into_model()`-style wrappers live in `client_async::model::v{version}`,\n\
         //! which is *not* part of this module.\n\n\
         // Doc comments below are copied verbatim from Bitcoin Core's RPC help. They contain bare\n\
         // URLs and bracketed parameter references (e.g. `[minconf]`) that rustdoc cannot resolve\n\
         // as Rust intra-doc links. Silencing locally is preferable to mangling the upstream prose.\n\
         #![allow(rustdoc::bare_urls, rustdoc::broken_intra_doc_links)]\n\n\
         pub mod methods;\n\
         pub mod options;\n\
         pub mod types;\n\n\
         pub use options::*;\n\
         pub use types::*;\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedup_description_strips_leading_summary() {
        assert_eq!(
            dedup_description("Returns the tip.", "Returns the tip.\n\nMore detail."),
            "More detail."
        );
    }

    #[test]
    fn dedup_description_is_a_noop_when_no_overlap() {
        assert_eq!(
            dedup_description("Summary one.", "Different description."),
            "Different description."
        );
    }

    #[test]
    fn esc_doc_escapes_angle_brackets() {
        assert_eq!(esc_doc("see <wallet name> on disk"), "see \\<wallet name\\> on disk");
    }

    #[test]
    fn esc_doc_passes_through_plain_text() {
        assert_eq!(esc_doc("hello world"), "hello world");
    }

    #[test]
    fn integer_param_names_includes_well_known_counts() {
        for name in ["height", "verbosity", "minconf", "nblocks"] {
            assert!(
                INTEGER_PARAM_NAMES.iter().any(|n| n.eq_ignore_ascii_case(name)),
                "{name} should be flagged as integer-only"
            );
        }
    }

    #[test]
    fn verbose_suffix_zero_and_one() {
        assert_eq!(verbose_suffix("verbose=0", 0), "VerboseZero");
        assert_eq!(verbose_suffix("verbosity=1", 1), "VerboseOne");
        assert_eq!(verbose_suffix("verbose is set to true", 0), "VerboseOne");
    }

    #[test]
    fn word_lists_remain_consistent() {
        // Spot-check that some words we deliberately removed are still gone.
        use crate::names::METHOD_WORDS;
        assert!(!METHOD_WORDS.contains(&"blocks"));
        assert!(!METHOD_WORDS.contains(&"txout"));
        assert!(!METHOD_WORDS.contains(&"outset"));
    }
}

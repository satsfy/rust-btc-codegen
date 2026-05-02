// SPDX-License-Identifier: CC0-1.0

//! A small typed slice of the OpenRPC document — enough to drive codegen, no more.
//!
//! Bitcoin Core's spec is ~700 KB of JSON describing 169 methods; modelling all of OpenRPC is a
//! waste of effort. We deserialise only the fields we use, leave the rest as
//! [`serde_json::Value`], and let serde drop unknown keys silently.

use serde::Deserialize;
use serde_json::Value;

/// Top-level OpenRPC document.
#[derive(Debug, Deserialize)]
pub struct Spec {
    pub methods: Vec<Method>,
}

/// One JSON-RPC method as described by the spec.
#[derive(Debug, Deserialize)]
pub struct Method {
    pub name: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub params: Vec<Param>,
    pub result: ResultObj,
    /// Bitcoin Core groups methods by `help` category via this extension.
    #[serde(rename = "x-bitcoin-category", default = "default_category")]
    pub category: String,
}

fn default_category() -> String { "misc".to_owned() }

/// One parameter of a method.
#[derive(Debug, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub required: bool,
    pub schema: Schema,
}

/// Wrapper around a method's result schema.
#[derive(Debug, Deserialize)]
pub struct ResultObj {
    pub schema: Schema,
}

/// A JSON Schema shape, kept loose. We pull common fields out as named struct fields and
/// stash the rest of the JSON object in `extra` for ad-hoc lookups.
#[derive(Debug, Default, Deserialize)]
pub struct Schema {
    #[serde(rename = "type", default)]
    pub kind: Option<SchemaType>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub default: Option<Value>,
    #[serde(default)]
    pub properties: Option<serde_json::Map<String, Value>>,
    #[serde(default)]
    pub required: Option<Vec<String>>,
    #[serde(default)]
    pub items: Option<Items>,
    #[serde(rename = "additionalProperties", default)]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(rename = "oneOf", default)]
    pub one_of: Option<Vec<Schema>>,
    #[serde(rename = "anyOf", default)]
    pub any_of: Option<Vec<Schema>>,
    #[serde(rename = "x-bitcoin-type", default)]
    pub bitcoin_type: Option<String>,
    #[serde(rename = "x-bitcoin-object-dynamic", default)]
    pub dynamic: bool,
    #[serde(rename = "x-bitcoin-optional", default)]
    pub bitcoin_optional: bool,
    #[serde(rename = "x-bitcoin-condition", default)]
    pub condition: Option<String>,
}

/// JSON Schema's `type` field can be a single string or an array.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SchemaType {
    One(String),
    Many(Vec<String>),
}

impl SchemaType {
    pub fn primary(&self) -> &str {
        match self {
            SchemaType::One(s) => s.as_str(),
            SchemaType::Many(v) => v.first().map(String::as_str).unwrap_or(""),
        }
    }
}

/// JSON Schema's `additionalProperties` is `bool | object`. We don't actually inspect the bool
/// arm — only the object arm carries useful schema — but we still accept it during deserialise.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Schema(Box<Schema>),
    Bool(#[allow(dead_code)] bool),
}

/// JSON Schema's `items` field can be a single schema (the common case) or an array of schemas
/// (variadic items / tuple-typed arrays). The array form shows up in Bitcoin Core's spec for a
/// few RPCs (e.g. `createrawtransaction.outputs`); we treat the first entry as the canonical
/// item schema and fall back to a generic value type elsewhere.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Single(Box<Schema>),
    Tuple(Vec<Schema>),
}

impl Items {
    /// Returns the canonical item schema. Tuple-typed items collapse onto their first entry —
    /// good enough for codegen, and any caller needing per-position types can use `call_raw`.
    pub fn primary(&self) -> Option<&Schema> {
        match self {
            Items::Single(s) => Some(s),
            Items::Tuple(v) => v.first(),
        }
    }
}

impl Schema {
    pub fn is_simple(&self) -> bool {
        matches!(
            self.kind.as_ref().map(SchemaType::primary),
            Some("string" | "boolean" | "number" | "integer")
        ) && !self.has_props()
    }

    pub fn returns_null(&self) -> bool {
        matches!(self.kind.as_ref().map(SchemaType::primary), Some("null"))
    }

    pub fn has_props(&self) -> bool {
        self.properties
            .as_ref()
            .map(|m| m.values().any(Value::is_object))
            .unwrap_or(false)
    }

    pub fn primary_kind(&self) -> Option<&str> {
        self.kind.as_ref().map(SchemaType::primary)
    }
}

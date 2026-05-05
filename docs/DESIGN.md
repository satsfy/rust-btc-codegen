# Design

This repository is a one-shot code generator for Bitcoin Core JSON-RPC bindings. It reads
Bitcoin Core's OpenRPC export, lowers the parts of the schema that matter for the Rust client,
and writes versioned Rust modules that are copied into the `corepc-client` tree.

The generator is intentionally not part of the consumer crate's build. Generated files are
committed in the consumer so normal builds have no dependency on this repository, on the OpenRPC
JSON files, or on codegen being reproducible at build time.

## How To Read This System

This project is easiest to understand as a small compiler:

- Input language: Bitcoin Core's OpenRPC JSON export.
- Front end: `src/spec.rs`, which deserializes only the OpenRPC fields this tool needs.
- Middle layer: `src/codegen.rs::lower()`, which converts OpenRPC methods into generator-owned
  data structures.
- Back end: `Modules::write()`, which emits Rust source files for the consumer crate.
- Target runtime: `corepc-client`, not this repository.

The important boundary is that this repo does not execute JSON-RPC calls. It generates the raw
versioned call surface that `corepc-client` compiles and runs.

## Architectural Patterns

The code uses a few deliberate patterns:

- Thin CLI, library-owned work: `src/main.rs` only resolves versions and paths; `src/lib.rs` owns
  the reusable `generate()` path.
- Typed slice over full schema: `src/spec.rs` models only the OpenRPC subset needed for codegen and
  leaves the rest as ignored JSON.
- Lowered intermediate representation: `GenType`, `MethodOut`, `ParamOut`, and `Modules` are the
  generator's internal model, separate from the raw OpenRPC structs.
- Recursive schema lowering: object and array schemas recursively create nested Rust helper types.
- Conservative dynamic fallback: ambiguous shapes become `serde_json::Value` rather than an
  over-specific generated type.
- Deterministic output: sorted collections and category grouping make generated diffs reviewable.
- Split generated files: types, options, and methods are emitted separately so reviewers can audit
  response shape, optional argument surface, and dispatch shape independently.
- Curated naming: all-lowercase Bitcoin Core RPC names are split with maintained word lists rather
  than guessed with a general-purpose algorithm.
- Raw/model separation: this repo generates raw RPC bindings; semantic wrappers and runtime variant
  dispatch belong in the consumer's model layer.

## Repository Parts

`Cargo.toml`

Defines the `btc-codegen` binary crate. The manifest has an empty `[workspace]` table so this
directory stays outside the surrounding `corepc` workspace. The crate is private
(`publish = false`) and only depends on `serde` and `serde_json`.

`src/main.rs`

The CLI entry point. It accepts `btc-codegen <version>` or `btc-codegen all`, resolves specs under
`specs/`, and calls the library entry point for each selected version. Output is written under
`output/v{version}/`.

`src/lib.rs`

The programmatic entry point. `generate(spec_path, out_dir, version)` reads JSON, deserializes the
typed OpenRPC slice, creates the output directory, lowers the spec, writes all generated files, and
returns a small `Summary`.

`src/spec.rs`

The typed OpenRPC adapter. It models only the subset of OpenRPC and Bitcoin Core extensions that
the generator consumes. Unknown JSON fields are ignored by serde.

`src/names.rs`

Identifier normalization. Bitcoin Core RPC names are often all-lowercase compounds such as
`getblockheader`, so this module carries curated word lists and turns method names and field names
into Rust identifiers.

`src/codegen.rs`

The main lowering and emission engine. It converts `Spec` into an in-memory `Modules` bundle and
emits four Rust files: `mod.rs`, `types.rs`, `options.rs`, and `methods.rs`.

`specs/`

Committed OpenRPC JSON inputs. `v30_2_0_openrpc.json` is the complete current useful input in this
checkout: it contains 169 methods. `v29_3_0_openrpc.json` is a placeholder with one comment-shaped
method object, not a valid generator input. Because of that, `btc-codegen all` currently fails on
v29 even though `btc-codegen 30` works.

`output/`

Generated artifacts. The normal generated layout is `output/v30/{mod,types,options,methods}.rs`.
This directory is listed in `.gitignore`, but the current checkout has generated output files
tracked as well. Treat them as inspectable artifacts, not as the runtime source of truth.

`generate.py` and `methods_gen.py`

Legacy Python generation scripts. They reference a `codegen.py` module that is not present in this
repository, so the active implementation is the Rust crate. Keep these files in mind when reading
history, but do not assume they are part of the working generation path.

`justfile`

Convenience wrapper around the Rust binary: `just codegen 30`, `just codegen-all`, `just test`,
`just lint`, `just fmt`, and `just clean`.

`.github/workflows/ci.yml`

Runs `cargo test`, `cargo clippy --all-targets -- --deny warnings`, and `cargo fmt --all -- --check`
on pushes and pull requests.

## Runtime Shape

There is no runtime component in this repo. The generated code is intended to compile inside
`corepc-client`, where these names exist:

- `crate::client_async::raw::Raw`
- `crate::client_async::error::Result`
- `Raw::client.call_raw(...)`
- `serde`, `serde_json`, and generated module imports

The generated method surface is deliberately raw and version-specific. Higher-level model wrappers
belong in `corepc-client/src/client_async/model`, not here.

## Generation Flow

The active flow is:

1. `src/main.rs` resolves the version argument.
2. `find_spec()` picks the first `specs/v{version}_*_openrpc.json` match.
3. `src/lib.rs::generate()` reads and parses the JSON into `spec::Spec`.
4. `codegen::lower()` converts the spec into `Modules`.
5. `Modules::write()` emits:
   - `mod.rs`: module declarations and re-exports.
   - `types.rs`: response structs, tuple newtypes, aliases, and nested helper types.
   - `options.rs`: `*Options` structs for methods with optional positional arguments.
   - `methods.rs`: `impl<'a> Raw<'a>` methods that call `call_raw`.

For v30 in this checkout, `cargo run --quiet -- 30` reports:

- 290 generated types
- 169 generated RPC methods
- 92 generated options structs

## OpenRPC Model

`src/spec.rs` intentionally avoids a complete OpenRPC AST. It keeps only:

- top-level `methods`
- method `name`, `summary`, `description`, `params`, `result`
- Bitcoin Core category extension: `x-bitcoin-category`
- parameter `name`, `description`, `required`, and `schema`
- schema `type`, `description`, `default`, `properties`, `required`, `items`
- `additionalProperties`
- `oneOf` and `anyOf`
- Bitcoin Core extensions:
  - `x-bitcoin-type`
  - `x-bitcoin-object-dynamic`
  - `x-bitcoin-optional`
  - `x-bitcoin-condition`

Several schema fields are untagged enums because JSON Schema permits multiple representations:

- `type` can be a string or an array.
- `additionalProperties` can be a bool or a schema object.
- `items` can be a single schema or tuple-like array.

The generator usually takes the first or "primary" form when a schema allows several shapes. This
is a conscious simplification: the output should be useful for most raw client calls, while callers
can still fall back to `serde_json::Value` or `call_raw` for complex positional or variant cases.

## Lowering Model

The core internal model in `src/codegen.rs` is small:

- `GenType`: one emitted Rust type plus any nested helper types it needs.
- `MethodOut`: one RPC method after name/type/doc lowering.
- `ParamOut`: one parameter after Rust identifier and Rust type inference.
- `Modules`: all generated types and methods, with helpers to emit the four files.

`lower()` walks every `spec::Method`, does two things, and stores both:

1. Generate return-type artifacts, if the method returns something other than null.
2. Generate method metadata for method and options emission.

Generated type names are deduplicated with a `BTreeSet`. The generator does not do deep structural
deduplication of equivalent schemas across methods. Repeating similar shapes is preferred because
it keeps generated diffs local to the RPC that changed.

## Return Type Rules

Return type lowering is intentionally direct:

- `type: null` becomes `Result<()>` and emits no response type.
- Simple scalar returns become tuple newtypes, for example `pub struct GetBlockCount(pub i64);`.
- Object returns become named structs with fields derived from `properties`.
- Array returns become tuple newtypes around `Vec<T>`.
- Dynamic object/map returns become tuple newtypes around `BTreeMap<String, T>`.
- `oneOf` and `anyOf` return methods use `serde_json::Value` as the method return type.

For `oneOf` and `anyOf`, the generator may still emit typed variant structs such as
`GetBlockVerboseZero`, `GetBlockVerboseOne`, and so on. The raw method returns
`serde_json::Value` because the correct variant depends on runtime parameters such as verbosity.
Typed dispatch for those cases belongs in hand-written model wrappers.

Nested object and array item schemas are recursively emitted as helper types. The name is derived
from the parent type plus the field or item role, for example `DecodePsbtInputsItem`.

## Object Field Rules

When lowering object properties:

- Requiredness comes from the schema's `required` array.
- `x-bitcoin-optional` also forces an `Option<T>`.
- JSON field names are converted to Rust field names with `names::to_rust_field()`.
- `#[serde(rename = "...")]` is emitted when the Rust field name differs from the wire name.
- Empty object schemas become empty structs.
- Dynamic object schemas become `BTreeMap<String, ...>`.
- Unrepresentable or ambiguous shapes fall back to `serde_json::Value`.

There is one explicit special case: if a response schema is commentary-only and references
`decoderawtransaction`, the generated type aliases to `DecodeRawTransaction`. This handles an
upstream spec shape where the schema text points at an existing response form instead of spelling
out fields again.

## Parameter Type Rules

Parameter types are inferred separately from response types because Bitcoin Core's spec is loose
around numeric parameters.

General rules:

- `x-bitcoin-type = "amount"` becomes `f64`.
- `x-bitcoin-type = "hex"` becomes `String`.
- `oneOf` and `anyOf` become `serde_json::Value`.
- strings become `String`.
- booleans become `bool`.
- integers become `i64`.
- arrays become `Vec<item_type>`.
- objects become `serde_json::Value`.

For `type: number`, parameters use `f64` by default, but switch to `i64` if the default JSON number
is integer-shaped or the parameter name is listed in `INTEGER_PARAM_NAMES`. That table exists
because Bitcoin Core often marks heights, counts, verbosity levels, indices, and timeouts as
`number` even though they are semantically integral.

Response-side `number` currently lowers to `i64` unless `x-bitcoin-type = "amount"` says `f64`.
That is a pragmatic fit for many Core response fields, but it is also one of the places to audit
carefully when adding new specs.

## Optional Positional Arguments

Bitcoin Core RPC optional arguments are positional. Skipping an optional slot is represented by
sending JSON `null`, which means "use Core's default".

For every method with optional parameters, the generator emits two Rust methods:

- A required-only method, such as `send_raw_transaction(hex)`.
- A `_with` method, such as `send_raw_transaction_with(hex, opts)`.

The `*Options` struct has one `Option<T>` field per optional parameter and derives `Default`.
Each `_with` method serializes the required arguments first, then each optional field in positional
order via `json!(opts.field)`. `None` therefore becomes JSON `null`.

This avoids a combinatorial overload surface while still allowing callers to set late optional
parameters without manually building the raw JSON parameter array.

## Method Emission

`methods.rs` is an `impl<'a> Raw<'a>` block.

Methods are grouped by `x-bitcoin-category` and categories are sorted alphabetically. Methods
inside each category are sorted by wire name. Each method body is intentionally thin:

```rust
self.client.call_raw("getblockhash", &[json!(height)]).await
```

Methods with no parameters use `&[(); 0] as &[()]` as an empty slice. This gives `call_raw` a
serializable empty parameter list without inventing a local constant or helper.

The generated file imports:

- `serde_json::json`
- `super::options::*`
- `super::types::*`
- `crate::client_async::error::Result`
- `crate::client_async::raw::Raw`

## Naming Strategy

Bitcoin Core method names do not carry separators, so `src/names.rs` uses curated word lists:

- `METHOD_WORDS` for RPC names.
- `FIELD_WORDS` for response and parameter field names.

The algorithm sorts the words longest-first and greedily consumes known words. If no word matches
at a position, it consumes an unknown run until the next known word boundary. This keeps unknown
tokens readable instead of splitting them into characters.

Important conventions:

- Plurals are only listed when they cannot be reconstructed safely by appending `s`.
- Single-word forms are preferred over compound entries when that preserves better boundaries.
- Rust keywords such as `type` and `ref` get an underscore suffix.
- Existing snake_case and camelCase field names are handled before all-lowercase word splitting.

This module is a maintenance hotspot. When a new Bitcoin Core release adds an RPC noun that
generates awkward names, add it to the relevant word list and regenerate.

## Collision Handling

Some RPC method names can produce Rust type identifiers that collide with common prelude names or
traits. `RESERVED_TYPE_NAMES` rewrites exact collisions:

- `Send` to `SendResult`
- `Sync` to `SyncResult`
- `Drop` to `DropResult`
- `Box` to `BoxResult`
- `Vec` to `VecResult`
- `Option` to `OptionResult`
- `Result` to `ResultResponse`

The replacement is used consistently for both emitted type definitions and method return types.

## Documentation Emission

Generated doc comments mostly come from Bitcoin Core's RPC help text.

The generator:

- deduplicates descriptions when the description starts with the summary verbatim;
- escapes angle brackets so rustdoc does not treat placeholders as HTML;
- locally allows rustdoc `bare_urls` and `broken_intra_doc_links` in generated modules because
  upstream prose contains URLs and bracketed parameter references.

The generated docs are intentionally not heavily normalized. They should preserve enough upstream
wording that a reviewer can compare generated output to the Bitcoin Core source material.

## Dependencies

This crate intentionally has very few dependencies.

| Dependency | Where it is used | Why it exists |
| --- | --- | --- |
| `serde` with `derive` | `src/spec.rs`, generated `types.rs` | Deserializes the OpenRPC slice and provides generated response derives. |
| `serde_json` with `preserve_order` | `src/lib.rs`, `src/spec.rs`, `src/codegen.rs`, generated `methods.rs` | Parses specs, stores loose schema fragments as `Value`, preserves object order while parsing, and emits RPC parameter JSON through `json!`. |

Standard library usage:

- `std::fs` and `std::path` for CLI and file output.
- `BTreeSet` for deterministic type-name deduplication.
- `BTreeMap` for deterministic category grouping and generated dynamic maps.

Generated-code assumptions in the consumer:

- `serde::{Deserialize, Serialize}` is available.
- `serde_json::json` is available.
- `Raw<'a>` has access to `self.client.call_raw(method, params).await`.
- the consumer defines the `serde-deny-unknown-fields` feature if it wants strict response
  deserialization in generated types.

Tooling dependencies:

- Rust 1.75+ according to `Cargo.toml`.
- `just` for convenience recipes, though all recipes are thin wrappers around `cargo`.
- GitHub Actions uses `dtolnay/rust-toolchain`, `Swatinem/rust-cache`, clippy, and rustfmt.

## Tests And Verification

Tests live inline in `src/codegen.rs` and `src/names.rs`.

Current coverage focuses on:

- method name splitting;
- field name splitting;
- Rust keyword handling;
- plural/compound boundary cases;
- doc escaping;
- summary/description deduplication;
- integer parameter-name coverage;
- verbose variant suffix detection.

`cargo test` currently passes with 15 tests.

The stronger acceptance test is outside this repository: regenerate output, copy it into
`corepc-client`, and run the consumer's async client tests. This is where generated signatures,
imports, feature gates, and model wrappers are validated together.

## Operational Notes

Regenerate one complete version:

```sh
cargo run --release -- 30
```

or:

```sh
just codegen 30
```

Do not rely on `all` until every file under `specs/` is a valid OpenRPC input. In this checkout,
`cargo run -- all` fails because `specs/v29_3_0_openrpc.json` is a placeholder.

When adding a new Bitcoin Core version:

1. Add `specs/v{major}_{minor}_{patch}_openrpc.json`.
2. Run codegen for that major version.
3. Inspect `types.rs`, `options.rs`, and `methods.rs` separately.
4. Add or adjust naming words if generated identifiers are awkward.
5. Audit `number` fields and parameter integer heuristics.
6. Copy generated files into the consumer's versioned codegen module.
7. Wire the consumer's feature/module gates and model wrappers.

## Design Tradeoffs

The main design choices are conservative:

- Generate raw version-specific bindings, not stable semantic models.
- Keep codegen out of the consumer build.
- Prefer reviewable generated diffs over global schema deduplication.
- Fall back to `serde_json::Value` for ambiguous OpenRPC shapes.
- Use handwritten model wrappers for runtime-dependent response variants.
- Keep naming deterministic through curated word lists rather than clever NLP.
- Encode optional RPC parameters through `Options` structs instead of overloading many method
  shapes.

This keeps the generator small and inspectable. The cost is that maintainers must audit generated
diffs, keep the naming tables current, and manually model higher-level semantics in the consumer
crate when raw JSON-RPC shape is not enough.

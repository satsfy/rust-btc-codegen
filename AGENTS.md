# AGENTS.md

This file gives repository-specific instructions for automated coding agents and human
contributors using agents. Keep it short, factual, and operational. General project design belongs
in `docs/DESIGN.md`; user-facing usage belongs in `README.md`.

## What This Repo Is

`rust-btc-codegen` is a private Rust code generator for Bitcoin Core JSON-RPC bindings. It reads
OpenRPC JSON specs from `specs/` and writes versioned generated Rust modules under `output/v{N}/`.

The generated files are consumed by `corepc-client`; this repo is not a runtime dependency of the
client and should not be added as a build dependency.

## First Files To Read

- `README.md`: user-facing purpose and operations.
- `docs/DESIGN.md`: architecture, lowering rules, tradeoffs, and maintenance notes.
- `src/main.rs`: CLI version/spec resolution.
- `src/lib.rs`: public `generate()` entry point.
- `src/spec.rs`: typed slice of OpenRPC consumed by the generator.
- `src/codegen.rs`: schema lowering and file emission.
- `src/names.rs`: method and field identifier normalization.

## Useful Commands

Run from the repository root:

```sh
cargo test
cargo clippy --all-targets -- --deny warnings
cargo fmt --all -- --check
cargo run --quiet -- 30
```

Convenience wrappers:

```sh
just test
just lint
just fmt
just codegen 30
```

Do not assume `cargo run -- all` or `just codegen-all` works unless every file in `specs/` is a
valid OpenRPC input. In this checkout, `specs/v29_3_0_openrpc.json` is a placeholder and causes
`all` generation to fail.

## Generated Output Rules

- The active generator is the Rust crate in `src/`.
- `generate.py` and `methods_gen.py` are legacy scripts; they reference a missing `codegen.py`.
- Do not hand-edit generated files in `output/v*/` as a fix. Change the generator or spec, then
  regenerate.
- `output/` is regenerable and ignored in `.gitignore`, though this checkout may have tracked
  output files for inspection.
- Generated output intended for production is copied into the consumer repo, not built from here.

## Editing Guidance

- Keep edits scoped to the generator, specs, docs, or tooling relevant to the task.
- Preserve deterministic output ordering. Prefer `BTreeMap`/`BTreeSet` when adding ordered
  generated surfaces.
- Prefer explicit fallbacks to `serde_json::Value` for ambiguous schema shapes.
- Keep generated diffs reviewable; avoid global type deduplication unless there is a strong reason.
- Add new RPC nouns to `METHOD_WORDS` or `FIELD_WORDS` when generated identifiers become awkward.
- Audit `type: number` handling when adding or updating specs, especially response fields and
  parameters that are semantically integers.
- Keep docs ASCII unless an existing file clearly requires otherwise.

## Verification Expectations

For generator changes, run at least:

```sh
cargo test
cargo run --quiet -- 30
```

For naming, schema lowering, or emission changes, also run:

```sh
cargo clippy --all-targets -- --deny warnings
cargo fmt --all -- --check
```

The strongest validation is in the consumer workspace after copying regenerated files:

```sh
cargo test -p corepc-client --features client-async
```

## Known Gotchas

- `serde_json` is built with `preserve_order`; do not remove that casually, because generated
  schema traversal and review diffs depend on stable behavior.
- Methods with `oneOf` or `anyOf` results often return `serde_json::Value` from the raw method even
  when typed variant structs are emitted.
- Optional RPC arguments are positional. `_with` methods serialize every optional slot, and `None`
  becomes JSON `null`.
- Some names intentionally use collision rewrites in `RESERVED_TYPE_NAMES`.
- Rustdoc warnings from upstream Bitcoin Core prose are allowed at the generated module boundary.

## When Updating This File

Put agent-facing workflow instructions here:

- commands agents should run;
- files agents should inspect first;
- repo-specific constraints;
- generation and verification steps;
- known traps that are not obvious from code.

Do not put broad style manifestos, long architecture explanations, or user-facing tutorials here.
Use `docs/DESIGN.md` and `README.md` for those.

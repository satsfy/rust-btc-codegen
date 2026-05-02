# rust-btc-codegen

A small, self-contained Rust tool that generates Rust bindings for the Bitcoin Core JSON-RPC
API from Bitcoin Core's own OpenRPC export.

This repository's goal is to be consumed by the
[`corepc-client`](https://github.com/rust-bitcoin/corepc) async production client as a
**source of generated code that is committed to the consumer's tree**. The tool runs only when
a maintainer regenerates bindings — the consumer crate never shells out to it at build time.

## What it produces

For each Bitcoin Core version `N` that has a spec under `specs/`, the tool writes four files
under `output/v{N}/`:

```
output/v{N}/
├── mod.rs       — module entry point + re-exports
├── types.rs     — return types (one struct per RPC response shape)
├── options.rs   — *Options structs for RPCs with optional positional arguments
└── methods.rs   — `impl<'a> Raw<'a> { ... }` blocks dispatching via Client::call_raw
```

Those four files are then copied into
`corepc-client/src/client_async/codegen/v{N}/`. They compile against the consumer's
`Client::call_raw` plumbing and require nothing else from this repo at runtime.

## Why three output files?

A single 5,000-line `generated.rs` makes diffs unreviewable. Splitting by concern lets a
reviewer auditing a new Core version skim each surface independently:

* `types.rs`   answers "did the response shape change?"
* `options.rs` answers "did optional parameters change?"
* `methods.rs` answers "did dispatch shape change?"

Most Core releases touch one of those without the others.

## Optional arguments

Bitcoin Core's RPC takes optional arguments **positionally**, with `null` as the
"use default" sentinel. The tool emits **two methods per RPC that has any optional parameters**:

```rust
// Required-only — uses Core's defaults for everything optional.
client.raw().send_raw_transaction(hex).await?;

// With-options — every optional field is `Option<T>`; `None` serialises as JSON `null`,
// which Core treats as "use the documented default".
client.raw().send_raw_transaction_with(
    hex,
    SendRawTransactionOptions { max_fee_rate: Some(0.5), ..Default::default() },
).await?;
```

## Operations manual

### Regenerate bindings

From the corepc workspace root:

```bash
just codegen          # regenerate every version
just codegen 30       # regenerate one version
```

Both recipes shell into this directory, run `cargo run --release -- <version|all>`, and copy
the resulting `output/v{N}/` tree into `corepc-client/src/client_async/codegen/v{N}/`.

Directly from this directory:

```bash
just codegen 30       # writes to ./output/v30/, no copying
just codegen-all
just test             # runs the Rust unit tests
just lint             # cargo clippy --deny warnings
just clean
```

### Add a new Bitcoin Core version

1. Apply Bitcoin Core's spec-export patch (a `bitcoin-cli getopenrpc` extension) to a checkout
   matching the version you want, build it, and copy the resulting JSON into
   `specs/v{major}_{minor}_{patch}_openrpc.json`.
2. Regenerate from the corepc workspace root: `just codegen {major}`.
3. Add the version feature in `corepc-client/Cargo.toml` (e.g. `"31_0" = []`) and gate the
   new module in `client_async/codegen/mod.rs` and `client_async/model/mod.rs`.

### Add an `into_model()` wrapper for an RPC

Hand-written model wrappers do **not** belong in this repo — they live in
`corepc-client/src/client_async/model/v{N}.rs`. The generated raw surface gives you
`client.raw().some_rpc()`; a model wrapper is a 5-line function on `Client` that delegates to
that and runs `into_model()`.

## Repository layout

```
.
├── README.md
├── Cargo.toml         ← self-contained crate manifest (empty `[workspace]` table keeps it
│                        out of the corepc Cargo workspace)
├── justfile           ← codegen / codegen-all / test / lint / clean
├── src/
│   ├── main.rs        ← CLI: `btc-codegen <version|all>`
│   ├── lib.rs         ← public `generate(spec, out_dir, version)` entry point
│   ├── names.rs       ← word lists + identifier conversion (PascalCase / snake_case)
│   ├── spec.rs        ← typed slice of OpenRPC JSON we need to read
│   └── codegen.rs     ← schema → Rust translation; emits the four output files
├── specs/             ← committed OpenRPC specs, one per Bitcoin Core release
│   ├── v29_3_0_openrpc.json
│   └── v30_2_0_openrpc.json
└── output/            ← generated artefacts (git-ignored; regenerable)
    └── v30/
        ├── mod.rs
        ├── types.rs
        ├── options.rs
        └── methods.rs
```

## Design notes

### Codegen as a one-way dependency

The generator is **not** a build dependency of `corepc-client`. The generated `.rs` files are
committed to the consumer's tree so:

* `cargo build` works on a fresh checkout without this repo present.
* PR reviewers can see exactly what ships.
* Reproducibility is git's job, not ours.

The cost is that every spec change requires a maintainer to run `just codegen` and commit the
diff. That is intentional — it forces a human to read the diff before it ships.

### Type-name conventions

The generator carries a curated word list (`METHOD_WORDS`, `FIELD_WORDS` in `src/names.rs`)
used by a longest-match peeling scan to convert all-lowercase compounds. Two design rules drive
the contents:

1. **Plurals are listed only when they cannot be reconstructed by appending `s`.** Words ending
   in a consonant + `s` (`blocks`, `txs`, `times`) are *omitted* because keeping them shadows
   real word boundaries — `blocks` + `tats` would otherwise win over the correct `block` +
   `stats` split in `getblockstats`.
2. **Single-word forms preferred over compound forms.** `txout` is split into `tx` + `out` so
   we get `GetTxOut`, matching `corepc-types`. We pay a one-off cost for any compound that
   cannot decompose cleanly (`prevout` is its own entry).

When a new Core release adds a noun the splitter doesn't know about, the output will look
"off" — add the noun to `METHOD_WORDS` / `FIELD_WORDS` and regenerate.

### Integer vs float for `type: number`

Bitcoin Core's OpenRPC export uses `type: number` for many fields that are semantically
integers (block height, verbosity level, conf targets). `INTEGER_PARAM_NAMES` in `codegen.rs`
maintains a list of parameter names that are integer-only across the entire RPC surface;
matches use `i64`. Anything else stays `f64`.

A future improvement is to patch Bitcoin Core's OpenRPC generator to emit `type: integer`
directly, at which point this list collapses to `[]`.

### Doc-comment escaping

Bitcoin Core's docstrings contain bare URLs, bracketed parameter references (`[minconf]`),
and angle-bracketed placeholders (`<wallet name>`). We escape the angle brackets so rustdoc
renders them literally and silence rustdoc's `bare_urls` and `broken_intra_doc_links` lints
at the generated module boundary, since those warnings are properties of the upstream prose
rather than mistakes in our codegen.

### Reserved type-name collisions

A handful of method names produce PascalCase types that collide with std prelude items —
notably `send` → `Send` (which would shadow the `Send` trait in the methods file). The
`RESERVED_TYPE_NAMES` table in `codegen.rs` rewrites these to non-colliding idents
(`SendResult`) for both the emitted struct and the method's `Result<...>` return-type
expression.

## Running the tests

```bash
cargo test                    # unit tests for name conversion and codegen helpers
```

The strongest acceptance test is "the consumer crate's tests still pass after a regen" — that
is, `cargo test -p corepc-client --features client-async` from the corepc workspace root.

# Proof of Concept for Bitcoin Core OpenRPC spec corepc-types codegen

This repo is still being developed!

To use, type:

```bash
python codegen.py -i specs/v30_2_0_openrpc.json -o client.rs -c 30
```

For Bitcoin Core v30 for example.

## What it does

Codegen — runs `corepc-codegen` against `specs/vN_*_openrpc.json`, producing a single `output/vN_generated.rs` containing every struct the spec implies.

## Operations manual — adding a new Bitcoin Core version

When a new Bitcoin Core release (say **v32**) ships:

### 1. Obtain the OpenRPC spec

Generating the OpenRPC spec requires running branch [this PR](https://github.com/bitcoin/bitcoin/pull/34683) on Bitcoin Core target version that adds a new command called `bitcoin-cli getopenrpcinfo`.

Run on Bitcoin Core itself:

```sh
bitcoin-cli getopenrpcinfo > openrpc_spec.json
```

Copy `openrpc_spec.json` over onto corepc folder as `specs/v<major>_<minor>_<patch>_openrpc.json`.

# Codegen Rust Types from OpenRPC Spec of Bitcoin Core RPC API

Runs code generation against OpenRPC sample, producing a single `<output_folder>/<structs>` containing every struct the spec implies.

This repo is still being developed!

To use, type:

```bash
python codegen.py -i specs/v30_2_0_openrpc.json -o client.rs -c 30
```

## Adding a new Bitcoin Core version

When a new Bitcoin Core release (say **v32**) ships, obtain the OpenRPC spec by generating the OpenRPC spec requires running branch [this PR](https://github.com/bitcoin/bitcoin/pull/34683) on Bitcoin Core target version that adds a new command called `bitcoin-cli getopenrpcinfo`.

Run on Bitcoin Core itself:

```sh
bitcoin-cli getopenrpcinfo > openrpc_spec.json
```

Copy `openrpc_spec.json` over onto corepc folder as `specs/v<major>_<minor>_<patch>_openrpc.json`.

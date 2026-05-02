# Recipes for `rust-btc-codegen`.
#
# All real logic lives in the `btc-codegen` Cargo crate beside this file. This justfile
# is just a thin convenience wrapper over `cargo run -- ...`.

set export

REPO_DIR := `git rev-parse --show-toplevel`

default:
    @just --list

# Generate Rust bindings for a single Bitcoin Core version (e.g. `just codegen 30`).
#
# Reads the matching spec from specs/v{VER}_*_openrpc.json and writes the four files
# (`mod.rs`, `types.rs`, `options.rs`, `methods.rs`) to output/v{VER}/.
codegen version:
    cargo run --release --quiet --manifest-path "{{REPO_DIR}}/Cargo.toml" -- {{version}}

# Generate bindings for every spec in specs/.
codegen-all:
    cargo run --release --quiet --manifest-path "{{REPO_DIR}}/Cargo.toml" -- all

# Run the codegen crate's own unit tests.
test:
    cargo test --manifest-path "{{REPO_DIR}}/Cargo.toml"

# Lint the codegen crate.
lint:
    cargo clippy --manifest-path "{{REPO_DIR}}/Cargo.toml" --all-targets -- --deny warnings

# Remove generated artefacts.
clean:
    rm -rf "{{REPO_DIR}}/output"
    cargo clean --manifest-path "{{REPO_DIR}}/Cargo.toml"

fmt:
    cargo fmt --manifest-path "{{REPO_DIR}}/Cargo.toml" --all
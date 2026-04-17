#!/bin/bash
# Compare corepc-types structs against the Bitcoin Core OpenRPC spec.
# Usage: ./spec_compare/run.sh <version|all>

set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <version|all>"
    exit 1
fi

if [[ "$1" == "all" ]]; then
    VERSIONS=(24 25 26 27 28 29 30)
else
    VERSIONS=("$1")
fi

mkdir -p "$DIR/output"

for VER in "${VERSIONS[@]}"; do
    SPEC=$(ls "$DIR/specs"/v${VER}_*_openrpc.json 2>/dev/null | head -1)
    if [[ -z "$SPEC" ]]; then
        echo "No spec for v${VER}, skipping."
        continue
    fi

    GEN="$DIR/output/v${VER}_generated.rs"
    INLINED="$DIR/output/v${VER}_inlined.rs"
    RPT="$DIR/output/v${VER}_compare.txt"

    echo "--- v${VER} ---"

    python3 "$DIR/codegen.py" -i "$SPEC" -o "$DIR/output" -c "$VER" --single-file
    mv "$DIR/output/generated.rs" "$GEN"

    python3 "$DIR/inline_corepc_version.py" --all-versions --types-dir "$ROOT/types/src" \
        --up-to-version "$VER" -o "$INLINED" 2>/dev/null

    python3 "$DIR/compare_types.py" --all --version "$VER" \
        --repo "$INLINED" --spec "$GEN" > "$RPT" 2>&1

    REPO_N=$(grep -c '^pub struct' "$INLINED")
    SPEC_N=$(grep -c '^pub struct' "$GEN")
    echo "  ${SPEC_N} spec, ${REPO_N} repo structs"
done

echo ""
echo "Done. Reports in spec_compare/output/"
for VER in "${VERSIONS[@]}"; do
    RPT="$DIR/output/v${VER}_compare.txt"
    if [[ -f "$RPT" ]]; then
        echo ""
        echo "######################################################################"
        echo "# v${VER} COMPARISON"
        echo "######################################################################"
        cat "$RPT"
    fi
done

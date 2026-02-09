#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$ROOT_DIR/crates/verange-proof/tests/fixtures"

mkdir -p "$OUT_DIR"

echo "Export helper placeholder."
echo "Expected workflow: run Java branch generators, then write JSON vectors into:"
echo "  $OUT_DIR"
echo "Current fixture file: $OUT_DIR/java_compat_vectors.json"

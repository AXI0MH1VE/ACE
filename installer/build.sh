#!/usr/bin/env bash
set -euo pipefail
echo "Building AxiomHive CLI + Tauri app..."
cargo build
(cd src-tauri && cargo build)
echo "Packaging public assets..."
mkdir -p dist/public
cp -r public/* dist/public/
echo "Done. Debug artifacts in target/ and src-tauri/target/."

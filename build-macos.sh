#!/usr/bin/env bash
set -euo pipefail
command -v node >/dev/null || { echo "Node.js is required."; exit 1; }
command -v cargo >/dev/null || { echo "Rust is required. Install from https://rustup.rs"; exit 1; }
npm install
npm run build:web
npx tauri build --bundles dmg,app

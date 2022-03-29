#!/bin/bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path"

# from setup_web.sh

# Pre-requisites:
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo update -p wasm-bindgen

# morf

FOLDER_NAME=${PWD##*/}
CRATE_NAME="rs_ray_tracing_v2"

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f "web/pkg/${CRATE_NAME}_bg.wasm"

echo "Building rust…"

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  wasm-pack build --target web --release --out-dir "web/pkg"

echo "Finished: web/${CRATE_NAME}.wasm"

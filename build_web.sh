#!/bin/bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path"

# from setup_web.sh

# Pre-requisites:
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo update -p wasm-bindgen

# For local tests with `./start_server`:
cargo install basic-http-server

# morf

FOLDER_NAME=${PWD##*/}
CRATE_NAME="rs_ray_tracing_v2"

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f "web/${CRATE_NAME}_bg.wasm"

echo "Building rust…"
BUILD=release
cargo build -p "${CRATE_NAME}" --release --lib --target wasm32-unknown-unknown

# Get the output directory (in the workspace it is in another location)
TARGET=$(cargo metadata --format-version=1 | jq --raw-output .target_directory)

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME}.wasm"
wasm-bindgen "${TARGET}/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}" \
  --out-dir web --no-modules --no-typescript

echo "Optimizing wasm…"
# to get wasm-opt:  apt/brew/dnf install binaryen
wasm-opt "web/${CRATE_NAME}_bg.wasm" -O2 --fast-math -o "web/${CRATE_NAME}_bg.wasm" # add -g to get debug symbols

echo "Finished: web/${CRATE_NAME}.wasm"

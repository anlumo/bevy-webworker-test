#!/bin/sh

set -e

cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-name bevy_worker --out-dir wasm --target no-modules target/wasm32-unknown-unknown/debug/bevy-webworker-test.wasm
python3 -m http.server --directory wasm

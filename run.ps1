cargo build --target wasm32-unknown-unknown
if ($lastexitcode -ne 0) {
    throw ("Exec: " + $errorMessage)
}

wasm-bindgen --out-name bevy_worker --out-dir wasm --target no-modules target/wasm32-unknown-unknown/debug/bevy-webworker-test.wasm
if ($lastexitcode -ne 0) {
    throw ("Exec: " + $errorMessage)
}

python3 server.py

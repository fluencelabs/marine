#!/bin/sh

cargo update
fce build --release
cd wasm/curl
cargo update
fce build --release
cd ../local_storage
cargo update
fce build --release

cd ..
rm artifacts/modules/*
cp ../../../target/wasm32-wasi/release/wasm_curl.wasm artifacts/modules
cp ../../../target/wasm32-wasi/release/wasm_local_storage.wasm artifacts/modules
cp ../../../target/wasm32-wasi/release/site-storage.wasm artifacts/modules
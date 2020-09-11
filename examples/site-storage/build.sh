#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir

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
cp ../../../target/wasm32-wasi/release/curl.wasm artifacts/modules
cp ../../../target/wasm32-wasi/release/local_storage.wasm artifacts/modules
cp ../../../target/wasm32-wasi/release/site-storage.wasm artifacts/modules
#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cd facade
cargo update
fce build --release
cd ../curl
cargo update
fce build --release
cd ../local_storage
cargo update
fce build --release

cd ..
rm -f artifacts/*
cp ../../target/wasm32-wasi/release/curl.wasm artifacts/
cp ../../target/wasm32-wasi/release/local_storage.wasm artifacts/
cp ../../target/wasm32-wasi/release/facade.wasm artifacts/

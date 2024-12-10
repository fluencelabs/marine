#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo run  --release -p marine -- build --release

rm artifacts/* || true
mkdir -p artifacts

cp ../../target/wasm32-wasip1/release/sqlite_test.wasm artifacts/
wget https://github.com/fluencelabs/sqlite/releases/download/v0.14.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo update
fce build --release

rm artifacts/*
cp ../../target/wasm32-wasi/release/sqlite_test.wasm artifacts/
wget https://github.com/fluencelabs/sqlite/releases/download/v0.8.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

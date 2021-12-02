#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo update --aggressive
marine build --release

rm artifacts/* || true
mkdir -p artifacts

cp target/wasm32-wasi/release/producer.wasm artifacts/

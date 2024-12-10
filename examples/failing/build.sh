#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo run  --release -p marine -- build --release

rm artifacts/* || true
mkdir -p artifacts

cp ../../target/wasm32-wasip1/release/failing.wasm artifacts/

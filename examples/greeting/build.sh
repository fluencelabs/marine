#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo update
fce build --release

rm artifacts/*
cp ../../target/wasm32-wasi/release/greeting.wasm artifacts/
cp ../../target/wasm32-wasi/release/greeting_cp.wasm artifacts/

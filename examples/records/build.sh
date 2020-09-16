#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
cd effector
cargo update
fce build --release
cd ../pure
cargo update
fce build --release

cd ..
rm artifacts/*
cp ../../target/wasm32-wasi/release/records_effector.wasm artifacts/
cp ../../target/wasm32-wasi/release/records_pure.wasm artifacts/

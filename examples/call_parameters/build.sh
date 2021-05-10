#!/bin/sh

cargo update --aggressive;
marine build --release;

rm -f artifacts/* || true;
mkdir -p artifacts;

cp ../../target/wasm32-wasi/release/call_parameters.wasm artifacts/;

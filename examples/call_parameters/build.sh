#!/bin/sh

cargo run  --release -p marine -- build --release;

rm -f artifacts/* || true;
mkdir -p artifacts;

cp ../../target/wasm32-wasip1/release/call_parameters.wasm artifacts/;

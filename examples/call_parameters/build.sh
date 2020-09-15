#!/bin/sh

cargo update
fce build --release

rm -f artifacts/*
cp ../../target/wasm32-wasi/release/call_parameters.wasm artifacts/

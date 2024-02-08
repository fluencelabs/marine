#! /usr/bin/env bash

# Build all tests
for test in *; do
  if [[ -d $test ]]; then
    echo "Building $test"
    cd $test
    cargo run  --release -p marine -- build --release
    mkdir artifacts -p
    cp ../../../../target/wasm32-wasi/release/$test_*.wasm artifacts/
    cd -
  fi
done

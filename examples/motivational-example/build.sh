#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
(
  cd shrek || exit;
  cargo run  --release -p marine -- build --release;
)

(
  cd donkey || exit;
  cargo run  --release -p marine -- build --release;
)

rm -f artifacts/* || true
mkdir -p artifacts

cp ../../target/wasm32-wasip1/release/shrek.wasm artifacts/
cp ../../target/wasm32-wasip1/release/donkey.wasm artifacts/

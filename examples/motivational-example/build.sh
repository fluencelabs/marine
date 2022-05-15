#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
(
  cd shrek || exit;
  cargo update --aggressive;
  marine build --release;
)

(
  cd donkey || exit;
  cargo update --aggressive;
  marine build --release;
)

rm -f artifacts/* || true
mkdir -p artifacts

cp ../../target/wasm32-wasi/release/shrek.wasm artifacts/
cp ../../target/wasm32-wasi/release/donkey.wasm artifacts/

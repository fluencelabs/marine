#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
(
  cd local_storage || exit;
  marine build --release;
)

(
  cd curl_adapter || exit;
  marine build --release;
)

(
  cd facade || exit;
  marine build --release;
)

rm -f artifacts/* || true
mkdir -p artifacts

cp ../../target/wasm32-wasi/release/local_storage.wasm artifacts/
cp ../../target/wasm32-wasi/release/curl_adapter.wasm artifacts/
cp ../../target/wasm32-wasi/release/facade.wasm artifacts/

#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
(
  cd local_storage || exit;
  cargo run  --release -p marine -- build --release;
)

(
  cd curl_adapter || exit;
  cargo run  --release -p marine -- build --release;
)

(
  cd facade || exit;
  cargo run  --release -p marine -- build --release;
)

rm -f artifacts/* || true
mkdir -p artifacts

export MARINE_TARGET=wasm32-wasip1
cp ../../target/$MARINE_TARGET/release/local_storage.wasm artifacts/
cp ../../target/$MARINE_TARGET/release/curl_adapter.wasm artifacts/
cp ../../target/$MARINE_TARGET/release/facade.wasm artifacts/

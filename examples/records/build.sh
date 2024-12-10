#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
(
  cd effector || exit;
  cargo run  --release -p marine -- build --release;
)

(
  cd pure || exit;
  cargo run  --release -p marine -- build --release;
)

rm artifacts/* || true
mkdir -p artifacts

export MARINE_PROJECT=wasm32-wasip1
cp ../../target/$MARINE_TARGET/release/records_effector.wasm artifacts/
cp ../../target/$MARINE_TARGET/release/records_pure.wasm artifacts/

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

cp ../../target/wasm32-wasip1/release/ipfs_effector.wasm artifacts/
cp ../../target/wasm32-wasip1/release/ipfs_pure.wasm artifacts/

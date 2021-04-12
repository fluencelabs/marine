#!/bin/sh

# This script builds all tests
(
  cd arguments_passing || exit;
  cargo update;
  fce build --release;
  rm artifacts/*;
)

(
  cd arrays_passing || exit;
  cargo update;
  fce build --release;
  rm artifacts/*;
)

(
  cd records_passing || exit;
  cargo update;
  fce build --release;
  rm artifacts/*;
)

cp ../../../target/wasm32-wasi/release/arguments_passing_effector.wasm arguments_passing/artifacts/
cp ../../../target/wasm32-wasi/release/arguments_passing_pure.wasm arguments_passing/artifacts/
cp ../../../target/wasm32-wasi/release/arrays_passing_effector.wasm arrays_passing/artifacts/
cp ../../../target/wasm32-wasi/release/arrays_passing_pure.wasm arrays_passing/artifacts/
cp ../../../target/wasm32-wasi/release/records_passing_effector.wasm records_passing/artifacts/
cp ../../../target/wasm32-wasi/release/records_passing_pure.wasm records_passing/artifacts/

#/bin/sh

# This script builds all tests
(
  cd records_allocation || exit;
  cargo update --aggressive;
  marine build --release;
  rm artifacts/* || true;
  mkdir artifacts
)

cp ../../../target/wasm32-wasi/release/records_allocation.wasm records_allocation/artifacts/

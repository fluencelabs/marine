#/bin/sh

# This script builds all tests
(
  cd lilo_after_2gb || exit;
  marine build --release;
  rm artifacts/* || true;
  mkdir artifacts
)

cp ../../../target/wasm32-wasi/release/lilo_after_2gb.wasm lilo_after_2gb/artifacts/

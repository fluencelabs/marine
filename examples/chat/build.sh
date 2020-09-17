#!/bin/sh

cd user-list
cargo update
fce build --release
cd ../history
cargo update
fce build --release

cd ..
rm -f artifacts/user-list.wasm
rm -f artifacts/history.wasm
cp ../../target/wasm32-wasi/release/user-list.wasm artifacts/
cp ../../target/wasm32-wasi/release/history.wasm artifacts/
cp ../../target/wasm32-wasi/release/user-list.wasm /home/diemust/git/fce/examples/chat/node_modules/fluence-playground/bin/.fluence/services/modules/
cp ../../target/wasm32-wasi/release/history.wasm /home/diemust/git/fce/examples/chat/node_modules/fluence-playground/bin/.fluence/services/modules

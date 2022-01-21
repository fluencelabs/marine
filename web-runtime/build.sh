#! /bin/bash

wasm-pack build -d marine-web-pkg --target web
wasm-pack build -d marine-node-pkg --target nodejs
wasm-pack build --no-typescript --release -d marine-any-pkg
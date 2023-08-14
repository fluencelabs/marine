#! /bin/bash

wasm-pack build -d marine-js-pkg --target web --features tracing-wasm
(cd npm-package || exit 1; npm i; npm run build)


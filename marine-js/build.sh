#! /bin/bash

wasm-pack build -d marine-web-pkg --target web

MARINE_JS_JS_DEST=npm-package/src/snippets/marine-web-runtime-6faa67b8af9cc173/
mkdir -p $MARINE_JS_JS_DEST
cp marine-js.js $MARINE_JS_JS_DEST

#! /bin/bash

wasm-pack build -d marine-js-pkg --target web

MARINE_JS_JS_DEST=npm-package/src/snippets/marine-js-6faa67b8af9cc173/
mkdir -p $MARINE_JS_JS_DEST
cp marine-js.js $MARINE_JS_JS_DEST

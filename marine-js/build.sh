#! /bin/bash

wasm-pack build -d marine-js-pkg --target web

MARINE_JS_JS_DEST=npm-package/src/snippets/marine-js-backend-ed2dc7cac6484845/
mkdir -p $MARINE_JS_JS_DEST
cp marine-js.js $MARINE_JS_JS_DEST

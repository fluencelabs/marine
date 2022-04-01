#!/bin/sh

(
    cd ../../npm-package
    npm i 
    npm run build
)

npm i
npm run install-local
npm run build
npm run test
#!/bin/sh

(
    cd ../../npm-package
    npm i 
    npm run build
)

npm i
npm install-local
npm run build
npm run test
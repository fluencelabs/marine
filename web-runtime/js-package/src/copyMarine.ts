#! /usr/bin/env node

import fs from 'fs';
import path from 'path';

const firstArgument = process.argv[2];

if (!firstArgument) {
    console.log(`Expected exactly 1 argument, got 0. Usage: ${path.basename(process.argv[1])} <destination directory>`);
    process.exit(1);
}

let destPath = firstArgument;
if (!path.isAbsolute(destPath)) {
    destPath = path.join(process.cwd(), destPath);
}

const wasmName = 'marine-js.wasm';
const packageName = '@fluencelabs/marine-js';

const modulePath = require.resolve(packageName);
const source = path.join(path.dirname(modulePath), wasmName);
const dest = path.join(destPath, wasmName);

console.log('ensure directory exists: ', destPath);
fs.mkdirSync(destPath, { recursive: true });

console.log('copying marine-js wasm');
console.log('from: ', source);
console.log('to: ', dest);
fs.copyFileSync(source, dest);

console.log('done!');

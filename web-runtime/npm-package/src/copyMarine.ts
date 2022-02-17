#! /usr/bin/env node

/*
 * Copyright 2022 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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

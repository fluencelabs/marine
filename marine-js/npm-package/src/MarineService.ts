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

import { WASI } from '@wasmer/wasi';
import bindingsRaw from '@wasmer/wasi/lib/bindings/browser.js';
import { defaultImport } from 'default-import';
import { WasmFs } from '@wasmer/wasmfs';
import { init } from './marine_js.js';
import type { MarineServiceConfig, Env } from './config.js';
import { JSONArray, JSONObject, LogFunction, LogLevel, logLevels } from './types.js';

const binding = defaultImport(bindingsRaw);

let cachegetUint8Memory0: any = null;

function getUint8Memory0(wasm: any) {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(wasm: any, ptr: any, len: any) {
    return decoder.decode(getUint8Memory0(wasm).subarray(ptr, ptr + len));
}

type Awaited<T> = T extends PromiseLike<infer U> ? U : T;

type ControlModuleInstance = Awaited<ReturnType<typeof init>> | 'not-set' | 'terminated';

const decoder = new TextDecoder();

export class MarineService {
    private env: Env = {};

    private _controlModuleInstance: ControlModuleInstance = 'not-set';

    constructor(
        private readonly controlModule: WebAssembly.Module,
        private readonly service: Array<{name: string, wasm_bytes: Uint8Array}>,
        private readonly serviceId: string,
        private logFunction: LogFunction,
        marineServiceConfig?: MarineServiceConfig,
        env?: Env,
    ) {
        this.env = {
            WASM_LOG: 'off',
            ...env,
        };
    }

    async init(): Promise<void> {
        // BEGIN OF OLD CODE
/*
        // wasi is needed to run marine modules with marine-js
        const wasi = new WASI({
            args: [],
            env: this.env,
            bindings: {
                ...binding,
                fs: new WasmFs().fs,
            },
        });

        const cfg: any = {
            exports: undefined,
        };

        const wasiImports = hasWasiImports(this.serviceModule) ? wasi.getImports(this.serviceModule) : {};

        const serviceInstance = await WebAssembly.instantiate(this.serviceModule, {
            ...wasiImports,
            host: {
                log_utf8_string: (levelRaw: any, target: any, offset: any, size: any) => {
                    let wasm = cfg.exports;

                    const level = rawLevelToTypes(levelRaw);
                    if (level === null) {
                        return;
                    }

                    const message = getStringFromWasm0(wasm, offset, size);
                    this.logFunction({
                        service: this.serviceId,
                        message,
                        level,
                    });
                },
            },
        });
        //wasi.start(serviceInstance);
        //cfg.exports = serviceInstance.exports;
*/
/// END OF OLD CODE

        const controlModuleInstance = await init(this.controlModule);

        //const customSections = WebAssembly.Module.customSections(this.serviceModule, 'interface-types');
        //const itCustomSections = new Uint8Array(customSections[0]);
        let rawResult = controlModuleInstance.register_module(this.service);

        let result: any;
        try {
            result = JSON.parse(rawResult);
            this._controlModuleInstance = controlModuleInstance;
            return result;
        } catch (ex) {
            throw 'register_module result parsing error: ' + ex + ', original text: ' + rawResult;
        }
    }

    terminate(): void {
        this._controlModuleInstance = 'not-set';
    }

    call(functionName: string, args: JSONArray | JSONObject, callParams: any): unknown {
        if (this._controlModuleInstance === 'not-set') {
            throw new Error('Not initialized');
        }

        if (this._controlModuleInstance === 'terminated') {
            throw new Error('Terminated');
        }

        // facade module is the last module of the service
        const facade_name = this.service[this.service.length - 1].name;
        const argsString = JSON.stringify(args);
        const rawRes = this._controlModuleInstance.call_module(facade_name, functionName, argsString);
        return JSON.parse(rawRes);
    }
}

function hasWasiImports(module: WebAssembly.Module): boolean {
    const imports = WebAssembly.Module.imports(module);
    const firstWasiImport = imports.find((x) => {
        return x.module === 'wasi_snapshot_preview1' || x.module === 'wasi_unstable';
    });
    return firstWasiImport !== undefined;
}

const rawLevelToTypes = (rawLevel: any): LogLevel | null => {
    switch (rawLevel) {
        case 1:
            return 'error';
        case 2:
            return 'warn';
        case 3:
            return 'info';
        case 4:
            return 'debug';
        case 5:
            return 'trace';
    }

    return null;
};

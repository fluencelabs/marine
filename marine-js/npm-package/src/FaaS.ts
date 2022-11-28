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
import bindings from '@wasmer/wasi/lib/bindings/browser';
import { WasmFs } from '@wasmer/wasmfs';
import { init } from './marine_js';
import type { FaaSConfig, Env } from './config';
import type { JSONArray, JSONObject, LogFunction } from './types';

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

type MarineInstance = Awaited<ReturnType<typeof init>> | 'not-set' | 'terminated';

const decoder = new TextDecoder();

export class FaaS {
    private env: Env = {};

    private _marineInstance: MarineInstance = 'not-set';

    constructor(
        private readonly controlModule: WebAssembly.Module,
        private readonly serviceModule: WebAssembly.Module,
        private readonly serviceId: string,
        private logFunction: LogFunction,
        faaSConfig?: FaaSConfig,
        env?: Env,
    ) {
        this.env = {
            WASM_LOG: 'off',
            ...env,
        };
    }

    async init(): Promise<void> {
        // wasi is needed to run marine modules with marine-js
        const wasi = new WASI({
            args: [],
            env: this.env,
            bindings: {
                ...bindings,
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
                log_utf8_string: (level: any, target: any, offset: any, size: any) => {
                    let wasm = cfg.exports;
                    const message = getStringFromWasm0(wasm, offset, size);
                    this.logFunction({
                        service: this.serviceId,
                        message,
                        level,
                    });
                },
            },
        });
        wasi.start(serviceInstance);
        cfg.exports = serviceInstance.exports;

        const controlModuleInstance = await init(this.controlModule);

        const customSections = WebAssembly.Module.customSections(this.serviceModule, 'interface-types');
        const itCustomSections = new Uint8Array(customSections[0]);
        let rawResult = controlModuleInstance.register_module(this.serviceId, itCustomSections, serviceInstance);

        let result: any;
        try {
            result = JSON.parse(rawResult);
            this._marineInstance = controlModuleInstance;
            return result;
        } catch (ex) {
            throw 'register_module result parsing error: ' + ex + ', original text: ' + rawResult;
        }
    }

    terminate(): void {
        this._marineInstance = 'not-set';
    }

    call(function_name: string, args: JSONArray | JSONObject, callParams: any): unknown {
        if (this._marineInstance === 'not-set') {
            throw new Error('Not initialized');
        }

        if (this._marineInstance === 'terminated') {
            throw new Error('Terminated');
        }

        const argsString = JSON.stringify(args);
        const rawRes = this._marineInstance.call_module(this.serviceId, function_name, argsString);
        const jsonRes: { result: unknown; error: string } = JSON.parse(rawRes);
        if (jsonRes.error) {
            throw new Error(`marine-js failed with: ${jsonRes.error}`);
        }

        return jsonRes.result;
    }
}

function hasWasiImports(module: WebAssembly.Module): boolean {
    const imports = WebAssembly.Module.imports(module);
    const firstWasiImport = imports.find((x) => {
        return x.module === 'wasi_snapshot_preview1' || x.module === 'wasi_unstable';
    });
    return firstWasiImport !== undefined;
}

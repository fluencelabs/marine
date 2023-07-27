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

import {WASI, WASIEnv} from '@wasmer/wasi';
import bindingsRaw from '@wasmer/wasi/lib/bindings/browser.js';
import { defaultImport } from 'default-import';
import { WasmFs } from '@wasmer/wasmfs';
import { init } from './marine_js.js';
import type {MarineServiceConfig, Env, Args} from './config.js';
import { JSONArray, JSONObject, LogFunction, LogLevel, logLevels } from './types.js';

const binding = defaultImport(bindingsRaw);

type Awaited<T> = T extends PromiseLike<infer U> ? U : T;

type ControlModuleInstance = Awaited<ReturnType<typeof init>> | 'not-set' | 'terminated';

const decoder = new TextDecoder();

export class MarineService {
    private env: Env = {};

    private _controlModuleInstance: ControlModuleInstance = 'not-set';

    constructor(
        private readonly controlModule: WebAssembly.Module,
        private readonly serviceId: string,
        private logFunction: LogFunction,
        private serviceConfig: MarineServiceConfig,
        env?: Env,
    ) {

        this.serviceConfig.modules_config.forEach(module => {
            module.config.wasi.envs = {
                WASM_LOG: 'off',            // general default
                ...env,                     // overridden by global envs
                ...module.config.wasi.envs, // overridden by module-wise envs
            }
        })
    }

    async init(): Promise<void> {
        const controlModuleInstance = await init(this.controlModule);

        controlModuleInstance.register_module(this.serviceConfig, this.logFunction);
        this._controlModuleInstance = controlModuleInstance;
    }

    terminate(): void {
        this._controlModuleInstance = 'not-set';
    }

    call(functionName: string, args: JSONArray | JSONObject, callParams?: CallParameters): unknown {
        if (this._controlModuleInstance === 'not-set') {
            throw new Error('Not initialized');
        }

        if (this._controlModuleInstance === 'terminated') {
            throw new Error('Terminated');
        }

        // facade module is the last module of the service
        const facade_name = this.serviceConfig.modules_config[this.serviceConfig.modules_config.length - 1].import_name;
        const argsString = JSON.stringify(args);
        const rawRes = this._controlModuleInstance.call_module(facade_name, functionName, argsString, callParams);
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

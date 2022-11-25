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

import { Env, FaaSConfig } from './config';
import { FaaS } from './FaaS';
import { IFluenceAppService, JSONArray, JSONObject, LogFunction } from './types';

const asArray = (buf: SharedArrayBuffer | Buffer) => {
    return new Uint8Array(buf);
};

export class FluenceAppService implements IFluenceAppService {
    private faasInstances = new Map<string, FaaS>();
    private controlModule?: WebAssembly.Module;

    constructor(private logFunction: LogFunction) {}

    async init(controlModuleWasm: SharedArrayBuffer | Buffer): Promise<void> {
        this.controlModule = await WebAssembly.compile(asArray(controlModuleWasm));
    }

    async createService(
        wasm: SharedArrayBuffer | Buffer,
        serviceId: string,
        faaSConfig?: FaaSConfig,
        envs?: Env,
    ): Promise<void> {
        if (!this.controlModule) {
            throw new Error('MarineJS is not initialized. To initialize call `init` function');
        }

        const service = await WebAssembly.compile(asArray(wasm));
        const faas = new FaaS(this.controlModule, service, serviceId, this.logFunction, faaSConfig, envs);
        await faas.init();
        this.faasInstances.set(serviceId, faas);
    }

    async terminate(): Promise<void> {
        this.faasInstances.forEach((val, key) => {
            val.terminate();
        });
    }

    async callService(
        serviceId: string,
        functionName: string,
        args: JSONArray | JSONObject,
        callParams: any,
    ): Promise<unknown> {
        const faas = this.faasInstances.get(serviceId);
        if (!faas) {
            throw new Error(`service with id=${serviceId} not found`);
        }

        return faas.call(functionName, args, callParams);
    }
}

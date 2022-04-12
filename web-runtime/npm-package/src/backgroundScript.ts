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

import { expose } from 'threads';
import { Envs, FaaSConfig } from './config';
import { FaaS } from './FaaS';
import { IFluenceAppService } from './IFluenceAppService';

const faasInstances = new Map<string, FaaS>();
let controlModule: WebAssembly.Module;

const asArray = (buf: SharedArrayBuffer | Buffer) => {
    return new Uint8Array(buf);
};

const toExpose: IFluenceAppService = {
    init: async (controlModuleWasm: SharedArrayBuffer | Buffer): Promise<void> => {
        controlModule = await WebAssembly.compile(asArray(controlModuleWasm));
    },
    createService: async (
        wasm: SharedArrayBuffer | Buffer,
        serviceId: string,
        faaSConfig?: FaaSConfig,
        envs?: Envs,
    ): Promise<void> => {
        const service = await WebAssembly.compile(asArray(wasm));
        const faas = new FaaS(controlModule, service, serviceId, faaSConfig, envs);
        await faas.init();
        faasInstances.set(serviceId, faas);
    },
    terminate: async (): Promise<void> => {
        faasInstances.forEach((val, key) => {
            val.terminate();
        });
    },
    callService: async (serviceId: string, functionName: string, args: string, callParams: any): Promise<string> => {
        const faas = faasInstances.get(serviceId);
        if (!faas) {
            throw new Error(`service with id=${serviceId} not found`);
        }

        return faas.call(functionName, args, callParams);
    },
};

expose(toExpose);

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

import { FaaSConfig, Env } from './config';
import { IFluenceAppService } from './IFluenceAppService';
import { isBrowser, isNode } from 'browser-or-node';
import { Thread, ModuleThread, spawn, Worker } from 'threads';
import { JSONArray, JSONObject, JSONValue } from './types';

export const defaultNames = {
    avm: {
        file: 'avm.wasm',
        package: '@fluencelabs/avm',
    },
    marine: {
        file: 'marine-js.wasm',
        package: '@fluencelabs/marine-js',
    },
    workerScriptPath: {
        web: './marine-js.web.js',
        node: './marine-js.node.js',
    },
};

export class FluenceAppService implements IFluenceAppService {
    private _worker?: ModuleThread<IFluenceAppService>;
    private _workerPath: string;

    constructor(workerScriptPath?: string) {
        if (workerScriptPath) {
            this._workerPath = workerScriptPath;
        }
        // check if we are running inside the browser and instantiate worker with the corresponding script
        else if (isBrowser) {
            this._workerPath = defaultNames.workerScriptPath.web;
        }
        // check if we are running inside nodejs and instantiate worker with the corresponding script
        else if (isNode) {
            this._workerPath = defaultNames.workerScriptPath.node;
        } else {
            throw new Error('Unknown environment');
        }
    }

    async init(controlModule: SharedArrayBuffer | Buffer): Promise<void> {
        if (this._worker) {
            return;
        }

        this._worker = await spawn<IFluenceAppService>(new Worker(this._workerPath));
        await this._worker.init(controlModule);
    }

    createService(
        serviceModule: SharedArrayBuffer | Buffer,
        serviceId: string,
        faaSConfig?: FaaSConfig,
        envs?: Env,
    ): Promise<void> {
        if (!this._worker) {
            throw 'Worker is not initialized';
        }

        return this._worker.createService(serviceModule, serviceId, faaSConfig, envs);
    }

    callService(
        serviceId: string,
        functionName: string,
        args: JSONArray | JSONObject,
        callParams: any,
    ): Promise<unknown> {
        if (!this._worker) {
            throw 'Worker is not initialized';
        }

        return this._worker.callService(serviceId, functionName, args, callParams);
    }

    async terminate(): Promise<void> {
        if (!this._worker) {
            return;
        }

        await this._worker.terminate();
        await Thread.terminate(this._worker);
    }
}

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

import { FaaSConfig, Envs } from './config';
import { IFluenceAppService } from './IFluenceAppService';
import { isBrowser, isNode } from 'browser-or-node';
import { Thread, ModuleThread, spawn, Worker } from 'threads';
import { Buffer } from 'buffer';

export const defaultNames = {
    avm: {
        name: 'avm.wasm',
        package: '@fluencelabs/avm',
    },
    marine: {
        name: 'marine-js.wasm',
        package: '@fluencelabs/marine-js',
    },
    script: {
        web: './marine-js.web.js',
        node: './marine-js.node.js',
    },
};

export const bufferToSharedArrayBuffer = (buffer: Buffer): SharedArrayBuffer | Buffer => {
    // only convert to shared buffers if necessary CORS headers have been set:
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements
    if (isBrowser && eval('crossOriginIsolated')) {
        const sab = new SharedArrayBuffer(buffer.length);
        const tmp = new Uint8Array(sab);
        tmp.set(buffer, 0);
        return sab;
    } else {
        return buffer;
    }
};

export const loadWasmFromServer = async (fileName: string): Promise<Buffer> => {
    if (!isBrowser) {
        throw new Error('Files can be loaded from url only in browser environment');
    }

    const fullUrl = window.location.origin + '/' + fileName;
    const res = await fetch(fullUrl);
    const ab = await res.arrayBuffer();
    return Buffer.from(ab);
};

export const loadWasmFromNpmPackage = async (packageName: string, fileName: string): Promise<Buffer> => {
    if (!isNode) {
        throw new Error('Files can be loaded from npm packages only in nodejs environment');
    }

    // eval('require') is needed so that
    // webpack will complain about missing dependencies for web target
    const r = eval('require');
    const path = r('path');
    const fs = r('fs').promises;
    const packagePath = r.resolve(packageName);
    const filePath = path.join(path.dirname(packagePath), fileName);
    return await fs.readFile(filePath);
};

export const loadWasm = async (args: { name: string; package: string }): Promise<SharedArrayBuffer | Buffer> => {
    let buffer: Buffer;
    // check if we are running inside the browser and instantiate worker with the corresponding script
    if (isBrowser) {
        buffer = await loadWasmFromServer(args.name);
    }
    // check if we are running inside nodejs and instantiate worker with the corresponding script
    else if (isNode) {
        buffer = await loadWasmFromNpmPackage(args.package, args.name);
    } else {
        throw new Error('Unknown environment');
    }

    return bufferToSharedArrayBuffer(buffer);
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
            this._workerPath = defaultNames.script.web;
        }
        // check if we are running inside nodejs and instantiate worker with the corresponding script
        else if (isNode) {
            this._workerPath = defaultNames.script.node;
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
        envs?: Envs,
    ): Promise<void> {
        if (!this._worker) {
            throw 'Worker is not initialized';
        }

        return this._worker.createService(serviceModule, serviceId, faaSConfig, envs);
    }

    callService(serviceId: string, functionName: string, args: string, callParams: any): Promise<string> {
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

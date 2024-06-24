/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import { init } from './marine_js.js';
import type {MarineServiceConfig, Env} from './config.js';
import { CallParameters, JSONArray, JSONObject, JSONValue, LogFunction } from './types.js';

type Awaited<T> = T extends PromiseLike<infer U> ? U : T;

type ControlModuleInstance = Awaited<ReturnType<typeof init>> | 'not-set' | 'terminated';

export class MarineService {
    private env: Env = {};

    private _controlModuleInstance: ControlModuleInstance = 'not-set';

    constructor(
        private readonly controlModule: WebAssembly.Module,
        private readonly serviceId: string,
        private logFunction: LogFunction,
        private serviceConfig: MarineServiceConfig,
        private modules: { [x: string]: Uint8Array },
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

        await controlModuleInstance.register_module(this.serviceConfig, this.modules, this.logFunction);
        this._controlModuleInstance = controlModuleInstance;
    }

    terminate(): void {
        this._controlModuleInstance = 'not-set';
    }

    async call(functionName: string, args: JSONArray | JSONObject, callParams: CallParameters): Promise<JSONValue> {
        if (this._controlModuleInstance === 'not-set') {
            throw new Error('Not initialized');
        }

        if (this._controlModuleInstance === 'terminated') {
            throw new Error('Terminated');
        }

        // facade module is the last module of the service
        const facade_name = this.serviceConfig.modules_config[this.serviceConfig.modules_config.length - 1].import_name;
        const argsString = JSON.stringify(args);
        const rawRes = await this._controlModuleInstance.call_module(facade_name, functionName, argsString, callParams);
        return JSON.parse(rawRes);
    }
}

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

export type JSONValue = string | number | boolean | { [x: string]: JSONValue } | Array<JSONValue>;
export type JSONArray = Array<JSONValue>;
export type JSONObject = { [x: string]: JSONValue };

export type LogFunction = (message: LogMessage) => void;

export interface LogMessage {
    service: string;
    message: string;
    level: LogLevel;
}

export enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Trace = 4,
    Debug = 5,
}

export type IFluenceAppService = {
    init: (controlModuleWasm: SharedArrayBuffer | Buffer) => Promise<void>;
    createService: (
        serviceModule: SharedArrayBuffer | Buffer,
        serviceId: string,
        faaSConfig?: FaaSConfig,
        envs?: Env,
    ) => Promise<void>;
    terminate: () => Promise<void>;
    callService: (
        serviceId: string,
        functionName: string,
        args: JSONArray | JSONObject,
        callParams: any,
    ) => Promise<unknown>;
};

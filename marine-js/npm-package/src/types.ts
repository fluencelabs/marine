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

export type JSONValue = string | number | boolean | { [x: string]: JSONValue } | Array<JSONValue>;
export type JSONArray = Array<JSONValue>;
export type JSONObject = { [x: string]: JSONValue };

export type LogFunction = (message: LogMessage) => void;

export interface LogMessage {
    service: string;
    message: string;
    level: LogLevel;
}

export const logLevels = ['trace', 'debug', 'info', 'warn', 'error'] as const;

export type LogLevel = typeof logLevels[number];

export const isLogLevel = (unknown: unknown): unknown is LogLevel => logLevels.some((val): boolean => unknown === val);

export const logLevelToEnv = (level: LogLevel): { WASM_LOG: LogLevel } => {
    return {
        WASM_LOG: level,
    };
};

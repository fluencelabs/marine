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
export type CallParameters = {
    /// Parameters of the particle that caused this call.
    particle: ParticleParameters,

    /// Id of the current service.
    service_id: string,

    /// Id of the service creator.
    service_creator_peer_id: string,

    /// PeerId of the peer who hosts this service.
    host_id: string,

    /// PeerId of the worker who hosts this service.
    worker_id: string,

    /// Security tetraplets which described origin of the arguments.
    tetraplets: Array<Array<SecurityTetraplet>>,
}

export type ParticleParameters = {
    /// Id of the particle which execution resulted a call this service.
    id: string,

    /// Peer id of the AIR script initiator.
    init_peer_id: string,

    /// Unix timestamp of the particle start time.
    timestamp: number,

    /// Time to live for this particle in milliseconds.
    ttl: number,

    /// AIR script in this particle.
    script: string,

    /// Signature made by particle initiator -- init_peer_id.
    signature: Array<number>,

    /// particle.signature signed by host_id -- used for FS access.
    token: string,
}

export type SecurityTetraplet = {
    /// Id of a peer where corresponding value was set.
    peer_pk: string,

    /// Id of a service that set corresponding value.
    service_id: string,

    /// Name of a function that returned corresponding value.
    function_name: string,

    /// Value was produced by applying this `lens` to the output from `call_service`.
    lens: string,
}

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

export const defaultCallParameters: CallParameters = {
    particle: {
        id: "",
        init_peer_id: "",
        timestamp: 0,
        ttl: 0,
        script: "",
        signature: [],
        token: "",
    },
    host_id: "",
    service_creator_peer_id: "",
    service_id: "",
    tetraplets: [],
    worker_id: ""
}

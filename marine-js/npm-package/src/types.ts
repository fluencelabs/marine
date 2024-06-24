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

/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// Internal API if marine-web. Only these functions are used for interacting with WebAssembly.Instance
// None of the functions below performs error-checking
// It is caller's responsibility to ensure that the instance is valid and has all the exports and required memory size

/**
 * Calls an export function from wasm module
 *
 * @param {WebAssembly.Instance} instance instance which will be called
 * @param {string} export_name name of the export
 * @param {string} args JSON array of args
 * @returns {string} JSON array of results
 * */
export function call_export(instance, export_name, args) {
    let parsed_args = JSON.parse(args);
    let prepared_args = [];
    for (let arg_index = 0; arg_index < parsed_args.length; arg_index++) {
        let arg = parsed_args[arg_index];
        if (arg["I32"] !== undefined) {
            prepared_args.push(arg["I32"])
        }
        if (arg["I64"] !== undefined) {;
            let val = BigInt(arg["I64"]);
            prepared_args.push(val)
        }
        if (arg["F32"] !== undefined) {
            prepared_args.push(arg["F32"])
        }
        if (arg["F64"] !== undefined) {
            prepared_args.push(arg["F64"])
        }
    }

    let result = instance.exports[export_name](...prepared_args);

    let json_result = "[]";
    if (result !== undefined) {
        if (typeof result == "bigint") {
            json_result = "[" + result.toString() + "]"
        } else {
            json_result = "[" + JSON.stringify(result) + "]"
        }
    }

    return json_result
}

/**
 * Gets size of the wasm memory
 *
 * @param {WebAssembly.Instance} instance instance which will be called
 * @returns {number} size of data
 * */
export function get_memory_size(instance) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    return buf.byteLength
}

/**
 * Reads one byte from wasm memory
 *
 * @param {WebAssembly.Instance} instance instance which will be used
 * @param {number} offset offset in wasm memory
 * @returns {number} byte from wasm memory
 * */
export function read_byte(instance, offset) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    return buf[offset];
}

/**
 * Writes one byte to wasm memory
 *
 * @param {WebAssembly.Instance} instance instance which will be used
 * @param {number} offset offset in wasm memory
 * @param {number} value value to write in memory
 * */
export function write_byte(instance, offset, value) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    buf[offset] = value;
}

/**
 * Writes byte range to wasm memory
 *
 * @param {WebAssembly.Instance} instance instance which will be used
 * @param {number} offset offset in wasm memory
 * @param {Uint8Array} slice array with bytes to write into memory
 * */
export function write_byte_range(instance, offset, slice) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    for (let byte_index = 0; byte_index < slice.length; byte_index++) {
        buf[offset + byte_index] = slice[byte_index]
    }
}

/**
 * Reads byte range from wasm memory
 *
 * @param {WebAssembly.Instance} instance instance which will be used
 * @param {number} offset offset in wasm memory
 * @param {Uint8Array} slice array to place read bytes
 * */
export function read_byte_range(instance, offset, slice) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    for (let byte_index = 0; byte_index < slice.length; byte_index++) {
        slice[byte_index] = buf[offset + byte_index];
    }
}

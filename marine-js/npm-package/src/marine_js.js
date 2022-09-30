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

// This is patched generated by wasm-pack file

import {
    call_export,
    read_byte,
    write_byte,
    get_memory_size,
    read_byte_range,
    write_byte_range,
} from './snippets/marine-js-6faa67b8af9cc173/marine-js.js';

export async function init(module) {
    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) { return heap[idx]; }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachedUint8Memory0 = new Uint8Array();

    function getUint8Memory0() {
        if (cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    let WASM_VECTOR_LEN = 0;

    const cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
            return cachedTextEncoder.encodeInto(arg, view);
        }
        : function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        });

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachedInt32Memory0 = new Int32Array();

    function getInt32Memory0() {
        if (cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    function passArray8ToWasm0(arg, malloc) {
        const ptr = malloc(arg.length * 1);
        getUint8Memory0().set(arg, ptr / 1);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }
    /**
     * Registers a module inside web-runtime.
     *
     * # Arguments
     *
     * * `name` - name of module to register
     * * `wit_section_bytes` - bytes of "interface-types" custom section from wasm file
     * * `instance` - `WebAssembly::Instance` made from target wasm file
     *
     * # Return value
     *
     * JSON object with field "error". If error is empty, module is registered.
     * otherwise, it contains error message.
     * @param {string} name
     * @param {Uint8Array} wit_section_bytes
     * @param {any} wasm_instance
     * @returns {string}
     */
    function register_module(name, wit_section_bytes, wasm_instance) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(wit_section_bytes, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.register_module(retptr, ptr0, len0, ptr1, len1, addHeapObject(wasm_instance));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }

    /**
     *  Calls a function from a module.
     *
     * # Arguments
     *
     * * module_name - name of registered module
     * * function_name - name of the function to call
     * * args - JSON array of function arguments
     *
     * # Return value
     *
     * JSON object with fields "error" and "result". If "error" is empty string,
     * "result" contains a function return value. Otherwise, "error" contains error message.
     * @param {string} module_name
     * @param {string} function_name
     * @param {string} args
     * @returns {string}
     */
    function call_module(module_name, function_name, args) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(module_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(function_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(args, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            wasm.call_module(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }

    function getArrayU8FromWasm0(ptr, len) {
        return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
    }

    function getImports() {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_writebyterange_783b310f6d87c4b8 = function(arg0, arg1, arg2, arg3) {
            write_byte_range(getObject(arg0), arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3));
        };
        imports.wbg.__wbg_readbyte_fb03559551e0b655 = function(arg0, arg1) {
            const ret = read_byte(getObject(arg0), arg1 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_readbyterange_0aaccd59853091e1 = function(arg0, arg1, arg2, arg3) {
            read_byte_range(getObject(arg0), arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3));
        };
        imports.wbg.__wbg_getmemorysize_0d0685486c307a71 = function(arg0) {
            const ret = get_memory_size(getObject(arg0));
            return ret;
        };
        imports.wbg.__wbg_new_693216e109162396 = function() {
            const ret = new Error();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
            const ret = getObject(arg1).stack;
            const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        };
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbg_writebyte_e07e58ec23d965ab = function(arg0, arg1, arg2) {
            write_byte(getObject(arg0), arg1 >>> 0, arg2);
        };
        imports.wbg.__wbg_callexport_5fee3906368c5b71 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
            const ret = call_export(getObject(arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5));
            const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };

        return imports;
    }

    function initMemory(imports, maybe_memory) {

    }

    function finalizeInit(instance, module) {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        cachedInt32Memory0 = new Int32Array();
        cachedUint8Memory0 = new Uint8Array();

        // calls main() function. Used to set up
        wasm.__wbindgen_start();
        return wasm;
    }

    async function init(wasmModule) {
        const imports = getImports();
        initMemory(imports);
        const instance = await WebAssembly.instantiate(wasmModule, imports);

        return finalizeInit(instance, module);
    }

    await init(module);

    return {
        wasm: wasm,
        register_module,
        call_module,
    };
}

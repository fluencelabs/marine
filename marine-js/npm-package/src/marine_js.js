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

import { create_wasi, generate_wasi_imports, bind_to_instance } from './snippets/marine-js-backend-8985bcc66aeb2a35/js/wasi_bindings.js';

export async function init(module) {
    let wasm;

    const heap = new Array(128).fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

        heap[idx] = obj;
        return idx;
    }

    function getObject(idx) { return heap[idx]; }

    function dropObject(idx) {
        if (idx < 132) return;
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

    let cachedFloat64Memory0 = null;

    function getFloat64Memory0() {
        if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
            cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
        }
        return cachedFloat64Memory0;
    }

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


    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;

        return real;
    }

    function logError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            let error = (function () {
                try {
                    return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
                } catch(_) {
                    return "<failed to stringify thrown value>";
                }
            }());
            console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
            throw e;
        }
    }

    let stack_pointer = 128;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }


    function __wbg_adapter_20(arg0, arg1, arg2) {
        try {
            _assertNum(arg0);
            _assertNum(arg1);
            const ret = wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0bd40dc488bb7714(arg0, arg1, addBorrowedObject(arg2));
            return takeObject(ret);
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function getInt32Memory0() {
        if (cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function _assertNum(n) {
        if (typeof(n) !== 'number') throw new Error('expected a number argument');
    }

    function _assertBoolean(n) {
        if (typeof(n) !== 'boolean') {
            throw new Error('expected a boolean argument');
        }
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
    /**
     * Registers a module inside web-runtime.
     *
     * # Arguments
     *
     * * `name` - name of module to register
     * * `wasm_bytes` - wasm file bytes
     *
     * # Return value
     *
     * JSON object with field "error". If error is empty, module is registered.
     * otherwise, it contains error message.
     * @param {string} name
     * @param {Uint8Array} wasm_bytes
     * @returns {string}
     */
    function register_module(name, wasm_bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(wasm_bytes, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.register_module(retptr, ptr0, len0, ptr1, len1);
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

        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_number_new = function(arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_createwasi_9079145d98d65af4 = function() {
            return logError(function() {
                const ret = create_wasi();
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_generatewasiimports_6af0910cd0fc37fa = function() {
            return logError(function(arg0, arg1) {
                const ret = generate_wasi_imports(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_bindtoinstance_84132e959c03c76d = function() {
            return logError(function(arg0, arg1) {
                bind_to_instance(getObject(arg0), getObject(arg1));
            }, arguments);
        };

        imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "number" ? obj : undefined;

            if (!isLikeNone(ret)) {
                _assertNum(ret);
            }

            getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        };

        imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_debug_8db2eed1bf6c1e2a = function() {
            return logError(function(arg0) {
                console.debug(getObject(arg0));
            }, arguments);
        };

        imports.wbg.__wbg_error_fe807da27c4a4ced = function() {
            return logError(function(arg0) {
                console.error(getObject(arg0));
            }, arguments);
        };

        imports.wbg.__wbg_info_9e6db45ac337c3b5 = function() {
            return logError(function(arg0) {
                console.info(getObject(arg0));
            }, arguments);
        };

        imports.wbg.__wbg_log_7bb108d119bafbc1 = function() {
            return logError(function(arg0) {
                console.log(getObject(arg0));
            }, arguments);
        };

        imports.wbg.__wbg_log_d047cf0648d2678e = function() {
            return logError(function(arg0, arg1) {
                console.log(getObject(arg0), getObject(arg1));
            }, arguments);
        };

        imports.wbg.__wbg_warn_e57696dbb3977030 = function() {
            return logError(function(arg0) {
                console.warn(getObject(arg0));
            }, arguments);
        };

        imports.wbg.__wbg_new_b525de17f44a8943 = function() {
            return logError(function() {
                const ret = new Array();
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_get_27fe3dac1c4d0224 = function() {
            return logError(function(arg0, arg1) {
                const ret = getObject(arg0)[arg1 >>> 0];
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_length_e498fbc24f9c1d4f = function() {
            return logError(function(arg0) {
                const ret = getObject(arg0).length;
                _assertNum(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_push_49c286f04dd3bf59 = function() {
            return logError(function(arg0, arg1) {
                const ret = getObject(arg0).push(getObject(arg1));
                _assertNum(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_byteLength_f4d9013afe43ad2f = function() {
            return logError(function(arg0) {
                const ret = getObject(arg0).byteLength;
                _assertNum(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_newwithargs_d66a68ef9c159f0d = function() {
            return logError(function(arg0, arg1, arg2, arg3) {
                const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_bind_73b3f819cbc666e0 = function() {
            return logError(function(arg0, arg1, arg2) {
                const ret = getObject(arg0).bind(getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_new_f9876326328f45ed = function() {
            return logError(function() {
                const ret = new Object();
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_new_537b7341ce90bb31 = function() {
            return logError(function(arg0) {
                const ret = new Uint8Array(getObject(arg0));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_newwithlength_b56c882b57805732 = function() {
            return logError(function(arg0) {
                const ret = new Uint8Array(arg0 >>> 0);
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_newwithbyteoffsetandlength_9fb2f11355ecadf5 = function() {
            return logError(function(arg0, arg1, arg2) {
                const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_subarray_7526649b91a252a6 = function() {
            return logError(function(arg0, arg1, arg2) {
                const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_length_27a2afe8ab42b09f = function() {
            return logError(function(arg0) {
                const ret = getObject(arg0).length;
                _assertNum(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_set_17499e8aa4003ebd = function() {
            return logError(function(arg0, arg1, arg2) {
                getObject(arg0).set(getObject(arg1), arg2 >>> 0);
            }, arguments);
        };

        imports.wbg.__wbg_getindex_0184568f38dea826 = function() {
            return logError(function(arg0, arg1) {
                const ret = getObject(arg0)[arg1 >>> 0];
                _assertNum(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_setindex_e050e698161bd575 = function() {
            return logError(function(arg0, arg1, arg2) {
                getObject(arg0)[arg1 >>> 0] = arg2;
            }, arguments);
        };

        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };

        imports.wbg.__wbg_new_64f7331ea86b0949 = function() {
            return handleError(function(arg0, arg1) {
                const ret = new WebAssembly.Instance(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_exports_ff0a0a2b2c092053 = function() {
            return logError(function(arg0) {
                const ret = getObject(arg0).exports;
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_new_3086807366ac3008 = function() {
            return handleError(function(arg0) {
                const ret = new WebAssembly.Module(getObject(arg0));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_instanceof_Memory_25684ccf3e250ca1 = function() {
            return logError(function(arg0) {
                let result;

                try {
                    result = getObject(arg0) instanceof WebAssembly.Memory;
                } catch {
                    result = false;
                }

                const ret = result;
                _assertBoolean(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function() {
            return logError(function(arg0) {
                const ret = getObject(arg0).buffer;
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_apply_5435e78b95a524a6 = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = Reflect.apply(getObject(arg0), getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_get_baf4855f9a986186 = function() {
            return handleError(function(arg0, arg1) {
                const ret = Reflect.get(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_set_6aa458a4ebdb65cb = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
                _assertBoolean(ret);
                return ret;
            }, arguments);
        };

        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };

        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_closure_wrapper2118 = function() {
            return logError(function(arg0, arg1, arg2) {
                const ret = makeMutClosure(arg0, arg1, 199, __wbg_adapter_20);
                return addHeapObject(ret);
            }, arguments);
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

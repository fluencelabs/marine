import { create_wasi, generate_wasi_imports, bind_to_instance } from "./snippets/marine-js-backend-e278c0aac863cc1b/js/wasi_bindings.js";

export async function init(module) {
    let wasm;

    const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

    if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

    let cachedUint8Memory0 = null;

    function getUint8Memory0() {
        if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    const heap = new Array(128).fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

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

    let WASM_VECTOR_LEN = 0;

    const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

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
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

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
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    let cachedInt32Memory0 = null;

    function getInt32Memory0() {
        if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    let cachedFloat64Memory0 = null;

    function getFloat64Memory0() {
        if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
            cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
        }
        return cachedFloat64Memory0;
    }

    let cachedBigInt64Memory0 = null;

    function getBigInt64Memory0() {
        if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
            cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
        }
        return cachedBigInt64Memory0;
    }

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

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

    let stack_pointer = 128;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }
    function __wbg_adapter_44(arg0, arg1, arg2) {
        try {
            const ret = wasm.wasm_bindgen__convert__closures__invoke1_mut_ref__h09e18c7fd0b9a0d3(arg0, arg1, addBorrowedObject(arg2));
            return takeObject(ret);
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function __wbg_adapter_47(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures__invoke1_mut__h4e03b323c96da859(arg0, arg1, addHeapObject(arg2));
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    }
    function __wbg_adapter_94(arg0, arg1, arg2, arg3) {
        wasm.wasm_bindgen__convert__closures__invoke2_mut__h55dcffe8da6071b7(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
    }

    /**
     */
    function main() {
        wasm.main();
    }

    /**
     * Registers a module inside web-runtime.
     *
     * # Arguments
     *
     * * `config` - description of wasm modules with names, wasm bytes and wasi parameters
     * * `log_fn` - function to direct logs from wasm modules
     *
     * # Return value
     *
     * Nothing. An error is signaled via exception.
     * @param {any} config
     * @param {object} modules
     * @param {Function} log_fn
     * @returns {Promise<void>}
     */
    function register_module(config, modules, log_fn) {
        const ret = wasm.register_module(addHeapObject(config), addHeapObject(modules), addHeapObject(log_fn));
        return takeObject(ret);
    }

    /**
     *  Calls a function from a module.
     *
     * # Arguments
     *
     * * module_name - name of registered module
     * * function_name - name of the function to call
     * * args - JSON array of function arguments
     * * call_parameters - an object representing call paramters, with the structure defined by fluence network
     * # Return value
     *
     * JSON array of values. An error is signaled via exception.
     * @param {string} module_name
     * @param {string} function_name
     * @param {string} args
     * @param {any} call_parameters
     * @returns {Promise<string>}
     */
    function call_module(module_name, function_name, args, call_parameters) {
        const ptr0 = passStringToWasm0(module_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(function_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(args, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.call_module(ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(call_parameters));
        return takeObject(ret);
    }

    async function __wbg_load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    function __wbg_get_imports() {
        const imports = {};
        imports.wbg = {};

        imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
            const ret = new Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_is_object = function(arg0) {
            const val = getObject(arg0);
            const ret = typeof val === "object" && val !== null;
            return ret;
        };

        imports.wbg.__wbg_getwithrefkey_5e6d9547403deab8 = function(arg0, arg1) {
            const ret = getObject(arg0)[getObject(arg1)];
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        };

        imports.wbg.__wbindgen_in = function(arg0, arg1) {
            const ret = getObject(arg0) in getObject(arg1);
            return ret;
        };

        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };

        imports.wbg.__wbg_isArray_74fb723e24f76012 = function(arg0) {
            const ret = Array.isArray(getObject(arg0));
            return ret;
        };

        imports.wbg.__wbg_length_1009b1af0c481d7b = function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        };

        imports.wbg.__wbg_get_f01601b5a68d10e3 = function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_next_6529ee0cca8d57ed = function() {
            return handleError(function(arg0) {
                const ret = getObject(arg0).next();
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_done_5fe336b092d60cf2 = function(arg0) {
            const ret = getObject(arg0).done;
            return ret;
        };

        imports.wbg.__wbg_value_0c248a78fdc8e19f = function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_keys_fb74497f0cb41c2a = function(arg0) {
            const ret = Object.keys(getObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_is_string = function(arg0) {
            const ret = typeof (getObject(arg0)) === "string";
            return ret;
        };

        imports.wbg.__wbg_get_7b48513de5dc5ea4 = function() {
            return handleError(function(arg0, arg1) {
                const ret = Reflect.get(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "string" ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };

        imports.wbg.__wbg_length_1d25fa9e4ac21ce7 = function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        };

        imports.wbg.__wbg_newwithlength_6c2df9e2f3028c43 = function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_new_92e51c6bd6a29ed7 = function() {
            return handleError(function(arg0) {
                const ret = new WebAssembly.Module(getObject(arg0));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_new_bfd4534b584a9593 = function() {
            const ret = new Map();
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_d257c6f2da008627 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_createwasi_b2f38d9d47d4b768 = function(arg0) {
            const ret = create_wasi(takeObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_call_5da1969d7cd31ccd = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_iterator_db7ca081358d4fb2 = function() {
            const ret = Symbol.iterator;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_is_function = function(arg0) {
            const ret = typeof (getObject(arg0)) === "function";
            return ret;
        };

        imports.wbg.__wbg_call_90c26b09837aba1c = function() {
            return handleError(function(arg0, arg1) {
                const ret = getObject(arg0).call(getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_next_9b877f231f476d01 = function(arg0) {
            const ret = getObject(arg0).next;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_self_f0e34d89f33b99fd = function() {
            return handleError(function() {
                const ret = self.self;
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_window_d3b084224f4774d7 = function() {
            return handleError(function() {
                const ret = window.window;
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_globalThis_9caa27ff917c6860 = function() {
            return handleError(function() {
                const ret = globalThis.globalThis;
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_global_35dfdd59a4da3e74 = function() {
            return handleError(function() {
                const ret = global.global;
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_newnoargs_c62ea9419c21fbac = function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_buffer_a448f833075b71ba = function(arg0) {
            const ret = getObject(arg0).buffer;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_new_8f67e318f15d7254 = function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_2357bf09366ee480 = function(arg0, arg1, arg2) {
            getObject(arg0).set(getObject(arg1), arg2 >>> 0);
        };

        imports.wbg.__wbg_newwithbyteoffsetandlength_d0482f893617af71 = function(arg0, arg1, arg2) {
            const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_byteLength_6a6065cdf205025b = function(arg0) {
            const ret = getObject(arg0).byteLength;
            return ret;
        };

        imports.wbg.__wbg_getindex_a3c0cf390273b9e3 = function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return ret;
        };

        imports.wbg.__wbg_log_53ed96ea72ace5e9 = function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbg_error_93b671ae91baaee7 = function(arg0, arg1) {
            console.error(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbg_warn_52c5b3e773c3a056 = function(arg0, arg1) {
            console.warn(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbg_new_9fb8d994e1c0aaac = function() {
            const ret = new Object();
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_841ac57cff3d672b = function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        };

        imports.wbg.__wbg_new_ffc6d4d085022169 = function() {
            const ret = new Array();
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_push_901f3914205d44de = function(arg0, arg1) {
            const ret = getObject(arg0).push(getObject(arg1));
            return ret;
        };

        imports.wbg.__wbg_apply_4d8fcf4ddc362ff9 = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = Reflect.apply(getObject(arg0), getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_error_c9309504864e78b5 = function(arg0, arg1) {
            console.error(getObject(arg0), getObject(arg1));
        };

        imports.wbg.__wbg_new_60f57089c7563e81 = function(arg0, arg1) {
            try {
                var state0 = {
                    a: arg0,
                    b: arg1
                };

                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;

                    try {
                        return __wbg_adapter_94(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };

                const ret = new Promise(cb0);
                return addHeapObject(ret);
            } finally {
                state0.a = state0.b = 0;
            }
        };

        imports.wbg.__wbg_log_576ca876af0d4a77 = function(arg0, arg1) {
            console.log(getObject(arg0), getObject(arg1));
        };

        imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
            const ret = getObject(arg0) == getObject(arg1);
            return ret;
        };

        imports.wbg.__wbindgen_boolean_get = function(arg0) {
            const v = getObject(arg0);
            const ret = typeof v === "boolean" ? (v ? 1 : 0) : 2;
            return ret;
        };

        imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "number" ? obj : undefined;
            getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        };

        imports.wbg.__wbg_isSafeInteger_f93fde0dca9820f8 = function(arg0) {
            const ret = Number.isSafeInteger(getObject(arg0));
            return ret;
        };

        imports.wbg.__wbg_setindex_a3212cb3f385777e = function(arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = arg2;
        };

        imports.wbg.__wbg_String_88810dfeb4021902 = function(arg0, arg1) {
            const ret = String(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };

        imports.wbg.__wbg_entries_9e2e2aa45aa5094a = function(arg0) {
            const ret = Object.entries(getObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_number_new = function(arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
            const v = getObject(arg1);
            const ret = typeof v === "bigint" ? v : undefined;
            getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        };

        imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
            const ret = getObject(arg0) === getObject(arg1);
            return ret;
        };

        imports.wbg.__wbg_newwithargs_7224eea6ed10c1a3 = function(arg0, arg1, arg2, arg3) {
            const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_bind_35f74f38f3af7264 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).bind(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_subarray_2e940e41c0f5a1d9 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_generatewasiimports_54404d2280114c11 = function(arg0, arg1) {
            const ret = generate_wasi_imports(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_759f75cd92b612d2 = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_log_1d3ae0273d8f4f8a = function(arg0) {
            console.log(getObject(arg0));
        };

        imports.wbg.__wbg_new_d306a62ce13a731d = function() {
            return handleError(function(arg0, arg1) {
                const ret = new WebAssembly.Instance(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_bindtoinstance_245f057fb347228a = function(arg0, arg1) {
            bind_to_instance(getObject(arg0), getObject(arg1));
        };

        imports.wbg.__wbg_exports_384cd3ab54ce4d9e = function(arg0) {
            const ret = getObject(arg0).exports;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_instanceof_Uint8Array_bced6f43aed8c1aa = function(arg0) {
            let result;

            try {
                result = getObject(arg0) instanceof Uint8Array;
            } catch (_) {
                result = false;
            }

            const ret = result;
            return ret;
        };

        imports.wbg.__wbg_instanceof_ArrayBuffer_e7d53d51371448e2 = function(arg0) {
            let result;

            try {
                result = getObject(arg0) instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }

            const ret = result;
            return ret;
        };

        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };

        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = takeObject(arg0).original;

            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }

            const ret = false;
            return ret;
        };

        imports.wbg.__wbg_then_3ab08cd4fbb91ae9 = function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_queueMicrotask_4d890031a6a5a50c = function(arg0) {
            queueMicrotask(getObject(arg0));
        };

        imports.wbg.__wbg_queueMicrotask_adae4bc085237231 = function(arg0) {
            const ret = getObject(arg0).queueMicrotask;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_resolve_6e1c6553a82f85b7 = function(arg0) {
            const ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_closure_wrapper1507 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 133, __wbg_adapter_44);
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_closure_wrapper1731 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 133, __wbg_adapter_47);
            return addHeapObject(ret);
        };

        return imports;
    }

    function __wbg_init_memory(imports, maybe_memory) {

    }

    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        __wbg_init.__wbindgen_wasm_module = module;
        cachedBigInt64Memory0 = null;
        cachedFloat64Memory0 = null;
        cachedInt32Memory0 = null;
        cachedUint8Memory0 = null;

        wasm.__wbindgen_start();
        return wasm;
    }

    function initSync(module) {
        if (wasm !== undefined) return wasm;

        const imports = __wbg_get_imports();

        __wbg_init_memory(imports);

        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }

        const instance = new WebAssembly.Instance(module, imports);

        return __wbg_finalize_init(instance, module);
    }

    async function __wbg_init(input) {
        if (wasm !== undefined) return wasm;

        if (typeof input === 'undefined') {
            input = new URL('marine_js_bg.wasm', import.meta.url);
        }
        const imports = __wbg_get_imports();

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        __wbg_init_memory(imports);

        const { instance, module } = await __wbg_load(await input, imports);

        return __wbg_finalize_init(instance, module);
    }

    await __wbg_init(module);

    return {
        wasm: wasm,
        register_module,
        call_module,
    };
}
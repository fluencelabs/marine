import {
    call_export,
    read_byte,
    write_byte,
    get_memory_size,
} from './snippets/marine-web-runtime-6faa67b8af9cc173/marine-js.js';

export async function init(module) {
    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) {
        return heap[idx];
    }

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

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
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

    let cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString =
        typeof cachedTextEncoder.encodeInto === 'function'
            ? function (arg, view) {
                  return cachedTextEncoder.encodeInto(arg, view);
              }
            : function (arg, view) {
                  const buf = cachedTextEncoder.encode(arg);
                  view.set(buf);
                  return {
                      read: arg.length,
                      written: buf.length,
                  };
              };

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0()
                .subarray(ptr, ptr + buf.length)
                .set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7f) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, (len = offset + arg.length * 3));
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachegetInt32Memory0 = null;
    function getInt32Memory0() {
        if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory0;
    }

    function passArray8ToWasm0(arg, malloc) {
        const ptr = malloc(arg.length * 1);
        getUint8Memory0().set(arg, ptr / 1);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }
    /**
     * @param {string} name
     * @param {Uint8Array} wit_section_bytes
     * @param {any} wasm_instance
     */
    function register_module(name, wit_section_bytes, wasm_instance) {
        var ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(wit_section_bytes, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        wasm.register_module(ptr0, len0, ptr1, len1, addHeapObject(wasm_instance));
    }

    /**
     * @param {string} module_name
     * @param {string} function_name
     * @param {string} args
     * @returns {string}
     */
    function call_module(module_name, function_name, args) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passStringToWasm0(module_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(function_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            var ptr2 = passStringToWasm0(args, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            wasm.call_module(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }

    async function init(wasmModule) {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_new_693216e109162396 = function () {
            var ret = new Error();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function (arg0, arg1) {
            var ret = getObject(arg1).stack;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_error_09919627ac0992f5 = function (arg0, arg1) {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        };
        imports.wbg.__wbindgen_object_drop_ref = function (arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbg_callexport_cb1a6ee1197892bd = function (arg0, arg1, arg2, arg3, arg4, arg5) {
            var ret = call_export(getObject(arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5));
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_getmemorysize_385fa0bd4e2d9ff6 = function (arg0) {
            var ret = get_memory_size(getObject(arg0));
            return ret;
        };
        imports.wbg.__wbg_writebyte_81064940ca9059c1 = function (arg0, arg1, arg2) {
            write_byte(getObject(arg0), arg1 >>> 0, arg2);
        };
        imports.wbg.__wbg_readbyte_63aea980ce35d833 = function (arg0, arg1) {
            var ret = read_byte(getObject(arg0), arg1 >>> 0);
            return ret;
        };

        const instance = await WebAssembly.instantiate(wasmModule, imports);
        wasm = instance.exports;

        // strange line from autogenerated code. No idea why it's needed
        init.__wbindgen_wasm_module = module;

        return wasm;
    }

    await init(module);

    return {
        wasm: wasm,
        register_module,
        call_module,
    };
}

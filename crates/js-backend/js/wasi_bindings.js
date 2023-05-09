import { WASI } from "@wasmer/wasi"
import { WasmFs } from "@wasmer/wasmfs"
import bindingsRaw from '@wasmer/wasi/lib/bindings/browser.js';
import { defaultImport } from 'default-import';

const bindings = defaultImport(bindingsRaw);

export function create_wasi() {
    const env = {"A": "B"};
    return new WASI({
        args: [],
        env: env,
        bindings: {
            ...bindings,
            fs: new WasmFs().fs,
        },
    })
}

export function generate_wasi_imports(module, wasi) {
    return hasWasiImports(module) ? wasi.getImports(module) : {};
}

export function bind_to_instance(wasi, instance) {
    wasi.setMemory(instance.exports["memory"]);
}

function hasWasiImports(module) {
    const imports = WebAssembly.Module.imports(module);
    const firstWasiImport = imports.find((x) => {
        return x.module === 'wasi_snapshot_preview1' || x.module === 'wasi_unstable';
    });
    return firstWasiImport !== undefined;
}
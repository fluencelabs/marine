import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';
import bindings from '@wasmer/wasi/lib/bindings/browser';
import { init } from './marine_web_runtime';
import { InitConfig, MarineConfig } from './config';

type LogLevel = 'info' | 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'off';

type LogImport = {
    log_utf8_string: (level: any, target: any, offset: any, size: any) => void;
};

type ImportObject = {
    host: LogImport;
};

type HostImportsConfig = {
    exports: any;
};

const logFunction = (level: LogLevel, message: string) => {
    switch (level) {
        case 'error':
            console.error(message);
            break;
        case 'warn':
            console.warn(message);
            break;
        case 'info':
            console.info(message);
            break;
        case 'debug':
        case 'trace':
            console.log(message);
            break;
    }
};

let cachegetUint8Memory0: any = null;

function getUint8Memory0(wasm: any) {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(wasm: any, ptr: any, len: any) {
    return decoder.decode(getUint8Memory0(wasm).subarray(ptr, ptr + len));
}

/// Returns import object that describes host functions called by AIR interpreter
function newImportObject(cfg: HostImportsConfig): ImportObject {
    return {
        host: log_import(cfg),
    };
}

function log_import(cfg: HostImportsConfig): LogImport {
    return {
        log_utf8_string: (level: any, target: any, offset: any, size: any) => {
            let wasm = cfg.exports;

            try {
                let str = getStringFromWasm0(wasm, offset, size);
                let levelStr: LogLevel;
                switch (level) {
                    case 1:
                        levelStr = 'error';
                        break;
                    case 2:
                        levelStr = 'warn';
                        break;
                    case 3:
                        levelStr = 'info';
                        break;
                    case 4:
                        levelStr = 'debug';
                        break;
                    case 6:
                        levelStr = 'trace';
                        break;
                    default:
                        return;
                }
                logFunction(levelStr, str);
            } finally {
            }
        },
    };
}

type Awaited<T> = T extends PromiseLike<infer U> ? U : T;

type MarineInstance = Awaited<ReturnType<typeof init>> | 'not-set' | 'terminated';

const decoder = new TextDecoder();

export class FluenceAppService {
    private _marineInstance: MarineInstance = 'not-set';
    private _serviceId: string | null = null;

    async init(config: InitConfig): Promise<void> {
        this._serviceId = config.serviceId;
        const marineModule = await WebAssembly.compile(new Uint8Array(config.marine));
        const serviceModule = await WebAssembly.compile(new Uint8Array(config.service));

        // wasi is needed to run AVM with marine-js
        const wasmFs = new WasmFs();
        const wasi = new WASI({
            args: [],
            env: {},
            bindings: {
                ...bindings,
                fs: wasmFs.fs,
            },
        });

        const cfg = {
            exports: undefined,
        };

        const serviceInstance = await WebAssembly.instantiate(serviceModule, {
            ...wasi.getImports(serviceModule),
            ...newImportObject(cfg),
        });
        wasi.start(serviceInstance);
        // @ts-ignore
        cfg.exports = serviceInstance.exports;

        const marineInstance = await init(marineModule);

        const customSections = WebAssembly.Module.customSections(serviceModule, 'interface-types');
        const itCustomSections = new Uint8Array(customSections[0]);
        let rawResult = marineInstance.register_module('avm', itCustomSections, serviceInstance);

        let result: any;
        try {
            result = JSON.parse(rawResult);
            // @ts-ignore
            this._marineInstance = marineInstance;
        } catch (ex) {
            throw 'register_module result parsing error: ' + ex + ', original text: ' + rawResult;
        }
    }

    async terminate(): Promise<void> {
        this._marineInstance = 'not-set';
    }

    async call(function_name: string, args: string, callParams: any): Promise<string> {
        if (this._marineInstance === 'not-set' || this._serviceId === null) {
            throw new Error('Not initialized');
        }

        if (this._marineInstance === 'terminated') {
            throw new Error('Terminated');
        }

        return this._marineInstance.call_module(this._serviceId, function_name, args);
    }
}

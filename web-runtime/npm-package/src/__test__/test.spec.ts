import { init } from '..';
import fs from 'fs';
import path from 'path';
import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';
import bindings from '@wasmer/wasi/lib/bindings/browser';

const createModule = async (path: string) => {
    const file = fs.readFileSync(path);
    return await WebAssembly.compile(file);
};

const invokeJson = (air: any, prevData: any, data: any, paramsToPass: any, callResultsToPass: any) => {
    return JSON.stringify([
        air,
        Array.from(prevData),
        Array.from(data),
        paramsToPass,
        Array.from(Buffer.from(JSON.stringify(callResultsToPass))),
    ]);
};

const b = (s: string) => {
    return Buffer.from(s);
};

const defaultAvmFileName = 'avm.wasm';
const avmPackageName = '@fluencelabs/avm';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const _wasmFs = new WasmFs();

const _wasi = new WASI({
    // Arguments passed to the Wasm Module
    // The first argument is usually the filepath to the executable WASI module
    // we want to run.
    args: [],

    // Environment variables that are accesible to the WASI module
    env: {},

    // Bindings that are used by the WASI Instance (fs, path, etc...)
    bindings: {
        ...bindings,
        fs: _wasmFs.fs,
    },
});

describe('Tests', () => {
    it('should work', async () => {
        const fluencePath = eval('require').resolve(avmPackageName);
        const avmPath = path.join(path.dirname(fluencePath), defaultAvmFileName);
        const controlModule = await createModule(path.join(__dirname, '../marine-js.wasm'));

        const avmModule = await createModule(avmPath);
        const marineInstance = await init(controlModule);

        const avmInstance = await WebAssembly.instantiate(avmModule, {
            ..._wasi.getImports(avmModule),
            host: {
                log_utf8_string: (level: any, target: any, offset: any, size: any) => {
                    console.log('logging, logging, logging');
                },
            },
        });
        _wasi.start(avmInstance);

        const customSections = WebAssembly.Module.customSections(avmModule, 'interface-types');
        const itcustomSections = new Uint8Array(customSections[0]);
        let result = marineInstance.register_module('avm', itcustomSections, avmInstance);
        expect(result).toEqual("{\"error\":\"\"}");

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        const params = { init_peer_id: vmPeerId, current_peer_id: vmPeerId };
        const json = invokeJson(s, b(''), b(''), params, {});
        let res: any = marineInstance.call_module('avm', 'invoke', json);
        res = JSON.parse(res);

        console.log(res);

        expect(res.error).toEqual("");
        expect(res.result).toMatchObject({
            ret_code: 0,
            error_message: '',
        });
    });
});

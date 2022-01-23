import init from '../marine_web_runtime';
import fs from 'fs';
import path from 'path';
import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';
import nodeBindings from '@wasmer/wasi/lib/bindings/node';

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
        ...nodeBindings,
        fs: _wasmFs.fs,
    },
});

describe('Tests', () => {
    it('should work', async () => {
        const fluencePath = eval('require').resolve(avmPackageName);
        // const avmPath = path.join(path.dirname(fluencePath), defaultAvmFileName);
        const avmPath = path.join(__dirname, '../', defaultAvmFileName);
        const avmModule = await createModule(avmPath);

        const controlModule = await createModule(path.join(__dirname, '../marine-js.wasm'));
        const control = await init(controlModule);

        const avmInstance = await WebAssembly.instantiate(avmModule, {
            ..._wasi.getImports(avmModule),
            host: {
                log_utf8_string: (level: any, target: any, offset: any, size: any) => {
                    console.log('logging, logging, logging');
                },
            },
        });
        const customSections = WebAssembly.Module.customSections(avmModule, 'interface-types');
        const itcustomSections = new Uint8Array(customSections[0]);
        control.register_module('avm', itcustomSections, avmInstance);

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        const params = { initPeerId: vmPeerId, currentPeerId: vmPeerId };
        let res: any = control.call_module('avm', 'invoke', invokeJson(s, b(''), b(''), params, []));
        res = res.result;

        expect(res).toMatchObject({
            retCode: 0,
            errorMessage: '',
        });
    });
});

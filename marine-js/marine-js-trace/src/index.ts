
import { promises as fsPromises } from 'fs';
import * as path from 'path';

import { parseArgs, } from 'node:util';
import {MarineService} from "@fluencelabs/marine-js/MarineService";
import {defaultCallParameters} from "@fluencelabs/marine-js/types"
import {MarineServiceConfig, ModuleDescriptor} from "@fluencelabs/marine-js/config"


import {serializeAvmArgs, RunParameters, CallResultsArray} from "@fluencelabs/avm"

const require = createRequire(import.meta.url);

import * as url from "url";
import {JSONArray, JSONObject} from "@fluencelabs/marine-js/types";
import {createRequire} from "module";

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const MARINE_WASM = "../../npm-package/dist/marine-js.wasm"
const examplesDir = path.join(__dirname, '../../../examples');
const wasmTestsDir = path.join(__dirname, '../../../marine/tests/wasm_tests');

const defaultModuleConfig = {
    logger_enabled: true,
    logging_mask: 0,
    wasi: {
        envs: {
            WASM_LOG: 'off'
        },
        preopened_files: new Set<string>(),
        mapped_dirs: new Map<String, string>()
    }
}

const b = (s: string) => {
    return Buffer.from(s);
};

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fsPromises.readFile(fullPath);
    const module = await WebAssembly.compile(buffer);
    return module;
};

const loadWasmBytes = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fsPromises.readFile(fullPath);
    return new Uint8Array(buffer);
};

const analysePerformance = () => {
    const entries = performance
        .getEntries()
        .filter((entry, index, array) => entry.entryType === 'measure')
        .sort((lhs, rhs) => lhs.duration - rhs.duration)
    for (const entry of entries) {
        console.log(`${entry.name} ${entry.duration.toFixed(3)}ms`)
    }
}

const dontLog = () => {};

const createModuleDescriptor = (name: string): ModuleDescriptor  => {
    return {
        import_name: name,
        config: defaultModuleConfig,
    }
}
const createSimpleService = (name: string): MarineServiceConfig => {
    return {
        modules_config: [createModuleDescriptor(name)]
    }
};

const avmPackagePath = require.resolve('@fluencelabs/avm');
const avm_path = path.join(path.dirname(avmPackagePath), 'avm.wasm');
const greeting_path = path.join(examplesDir,'./greeting/artifacts/greeting.wasm')

const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

const avm_args = serializeAvmArgs(
    {
        currentPeerId: vmPeerId,
        initPeerId: vmPeerId,
        timestamp: Date.now(),
        ttl: 10000,
    },
    s,
    b(''),
    b(''),
    [],
);

type TraceParams = {
    wasmPath: string,
    functionName: string,
    serviceArgs: JSONArray | JSONObject,
};


const parse_args = (): TraceParams => {
    const args = parseArgs({
        options: {
            'wasm-path': {type: 'string', short: "w", },
            'func-name': {type: 'string', short: "f", },
            'args': {type: 'string', short: "a" }
        },
        strict: true,
        tokens: true,
    });
    const wasmPath = args.values['wasm-path'];
    const funcName = args.values['func-name'];
    const serviceArgs = args.values['args'];

    if (wasmPath === undefined) {
        throw "--wasm-path is a required argument";
    }

    if (funcName === undefined) {
        throw "--func-name is a required argument";
    }

    if (serviceArgs === undefined) {
        throw "--args is a required argument";
    }

    return {
        wasmPath: wasmPath,
        functionName: funcName,
        serviceArgs: JSON.parse(serviceArgs)
    }
}

const run_wasm = async (params: TraceParams) => {

    const greeting = await loadWasmBytes(params.wasmPath);

    const config = createSimpleService('greeting');
    const modules = {greeting: greeting}

    performance.mark("start marine init");

    const marine = await loadWasmModule(path.join(__dirname, MARINE_WASM));
    const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
    await marineService.init();
    performance.measure("Marine initialization", "start marine init" );
    performance.mark("start marine call")
    const res = marineService.call(params.functionName, params.serviceArgs, defaultCallParameters);
    performance.measure("Marine call", "start marine call")
    performance.measure("Total init + call", "start marine init");
}

const main = async () => {
    const args = parse_args();
    await run_wasm(args);
    analysePerformance()
}

await main()




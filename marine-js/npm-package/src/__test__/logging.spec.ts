import { jest } from '@jest/globals';
import * as fs from 'fs';
import * as path from 'path';
import * as url from 'url';
import { MarineService } from '../MarineService.js';
import {defaultCallParameters, LogLevel} from '../types.js';
import {Env, MarineModuleConfig, MarineServiceConfig, ModuleDescriptor} from "../config.js";

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const examplesDir = path.join(__dirname, '../../../../examples');

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fs.promises.readFile(fullPath);
    return WebAssembly.compile(buffer);
};

const loadWasmBytes = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    return await fs.promises.readFile(fullPath);
};

const createModuleConfig = (envs: Env): MarineModuleConfig => {
    return {
        logger_enabled: true,
        logging_mask: 5,
        wasi: {
            envs: envs,
            preopened_files: new Set<string>(),
            mapped_dirs: new Map<String, string>()
        }
    }
}

const createModuleDescriptor = (name: string, envs: Env): ModuleDescriptor  => {
    return {
        import_name: name,
        config: createModuleConfig(envs),
    }
}
const createSimpleService = (name: string, envs: Env): MarineServiceConfig => {
    return {
        modules_config: [createModuleDescriptor(name, envs)]
    }
};


describe.each([
    // force column layout
    ['error' as const],
    ['warn' as const],
    ['info' as const],
    ['debug' as const],
    ['trace' as const],
])('WASM logging tests', (level: LogLevel) => {
    it('Testing logging level', async () => {
        // arrange
        const logger = jest.fn();

        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm')
        );
        const config = createSimpleService('srv', {WASM_LOG: level});
        const modules = { 'srv': new Uint8Array(greeting)}

        const marineService = new MarineService(marine, 'srv', logger, config, modules);
        await marineService.init();

        // act
        const res = marineService.call('log_' + level, [], defaultCallParameters);

        // assert
        expect(res).toBe(null);
        expect(logger).toBeCalledTimes(1);
        expect(logger).toHaveBeenNthCalledWith(1, { level, message: level, service: 'srv' });
    });
});

describe.each([
    // force column layout
    [{}],
    [{ WASM_LOG: 'off' }],
])('WASM logging tests for level "off"', (env) => {
    it('Testing logging level by passing env: %0', async () => {
        // arrange
        const logger = jest.fn();

        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const config = createSimpleService('srv', {WASM_LOG: 'off'});
        const modules = { 'srv': new Uint8Array(greeting)}

        const marineService = new MarineService(marine, 'srv', logger, config, modules);
        await marineService.init();

        // act
        const res1 = marineService.call('log_error', [], defaultCallParameters);
        const res2 = marineService.call('log_warn', [], defaultCallParameters);
        const res3 = marineService.call('log_info', [], defaultCallParameters);
        const res4 = marineService.call('log_debug', [], defaultCallParameters);
        const res5 = marineService.call('log_trace', [], defaultCallParameters);

        // assert
        expect(res1).toBe(null);
        expect(res2).toBe(null);
        expect(res3).toBe(null);
        expect(res4).toBe(null);
        expect(res5).toBe(null);

        expect(logger).toBeCalledTimes(0);
    });
});

import fs from 'fs';
import path from 'path';
import { FaaS } from '../FaaS';

const examplesDir = path.join(__dirname, '../../../../examples');

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fs.promises.readFile(fullPath);
    const module = await WebAssembly.compile(buffer);
    return module;
};

describe.each([
    // force column layout
    ['error', 'error'],
    ['warn', 'warn'],
    ['info', 'info'],
    ['debug', 'log'],
    ['trace', 'log'],
])('WASM logging tests', (level, fn) => {
    it('Testing logging level', async () => {
        // arrange
        // @ts-ignore
        console[fn] = jest.fn();
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const faas = new FaaS(marine, greeting, 'srv', undefined, { WASM_LOG: level });
        await faas.init();

        // act
        const res = faas.call('log_' + level, [], undefined);

        // assert
        expect(res).toBe(null);
        // @ts-ignore
        expect(console[fn]).toBeCalledTimes(1);
        // @ts-ignore
        expect(console[fn]).toHaveBeenNthCalledWith(1, '[marine service "srv"]: ' + level);
    });
});

describe.each([
    // force column layout
    [undefined],
    [{ WASM_LOG: 'off' }],
])('WASM logging tests for level "off"', (env) => {
    it('Testing logging level by passing env: %0', async () => {
        // arrange
        console.error = jest.fn();
        console.warn = jest.fn();
        console.debug = jest.fn();
        console.trace = jest.fn();
        console.info = jest.fn();
        console.log = jest.fn();

        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const faas = new FaaS(marine, greeting, 'srv', undefined, env);
        await faas.init();

        // act
        const res1 = faas.call('log_error', [], undefined);
        const res2 = faas.call('log_warn', [], undefined);
        const res3 = faas.call('log_info', [], undefined);
        const res4 = faas.call('log_debug', [], undefined);
        const res5 = faas.call('log_trace', [], undefined);

        // assert
        expect(res1).toBe(null);
        expect(res2).toBe(null);
        expect(res3).toBe(null);
        expect(res4).toBe(null);
        expect(res5).toBe(null);

        expect(console.error).toBeCalledTimes(0);
        expect(console.warn).toBeCalledTimes(0);
        expect(console.debug).toBeCalledTimes(0);
        expect(console.trace).toBeCalledTimes(0);
        expect(console.info).toBeCalledTimes(0);
        expect(console.log).toBeCalledTimes(0);
    });
});

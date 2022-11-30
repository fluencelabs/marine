import fs from 'fs';
import path from 'path';
import { MarineService } from '../MarineService';
import { LogLevel } from '../types';

const examplesDir = path.join(__dirname, '../../../../examples');

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fs.promises.readFile(fullPath);
    const module = await WebAssembly.compile(buffer);
    return module;
};

describe.each([
    // force column layout
    ['error', LogLevel.Error],
    ['warn', LogLevel.Warn],
    ['info', LogLevel.Info],
    ['debug', LogLevel.Debug],
    ['trace', LogLevel.Trace],
])('WASM logging tests', (level, resLevel) => {
    it('Testing logging level', async () => {
        // arrange
        const logger = jest.fn();

        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const marineService = new MarineService(marine, greeting, 'srv', logger, undefined, { WASM_LOG: level });
        await marineService.init();

        // act
        const res = marineService.call('log_' + level, [], undefined);

        // assert
        expect(res).toBe(null);
        expect(logger).toBeCalledTimes(1);
        expect(logger).toHaveBeenNthCalledWith(1, { level: resLevel, message: level, service: 'srv' });
    });
});

describe.each([
    // force column layout
    [undefined],
    [{ WASM_LOG: 'off' }],
])('WASM logging tests for level "off"', (env) => {
    it('Testing logging level by passing env: %0', async () => {
        // arrange
        const logger = jest.fn();

        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const marineService = new MarineService(marine, greeting, 'srv', logger, undefined, env);
        await marineService.init();

        // act
        const res1 = marineService.call('log_error', [], undefined);
        const res2 = marineService.call('log_warn', [], undefined);
        const res3 = marineService.call('log_info', [], undefined);
        const res4 = marineService.call('log_debug', [], undefined);
        const res5 = marineService.call('log_trace', [], undefined);

        // assert
        expect(res1).toBe(null);
        expect(res2).toBe(null);
        expect(res3).toBe(null);
        expect(res4).toBe(null);
        expect(res5).toBe(null);

        expect(logger).toBeCalledTimes(0);
    });
});

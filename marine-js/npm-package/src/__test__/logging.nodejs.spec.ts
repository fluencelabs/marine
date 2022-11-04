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

describe('WASM logging tests in nodejs', () => {
    it('Testing logging level', async () => {
        // arrange
        // @ts-ignore
        process.stderr.write = jest.fn();
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const faas = new FaaS(marine, greeting, 'srv', undefined, { WASM_LOG: 'info' });
        await faas.init();

        // act
        const res = faas.call('log_info', [], undefined);

        // assert
        // @ts-ignore
        expect(process.stderr.write).toBeCalledTimes(1);
        // @ts-ignore
        expect(process.stderr.write).toHaveBeenNthCalledWith(1, '[marine service "srv"]: info');
    });
});

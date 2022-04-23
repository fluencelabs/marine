import fs from 'fs';
import path from 'path';
import download from 'download';
import { FaaS } from '../FaaS';
import { callAvm } from '@fluencelabs/avm';

const examplesDir = path.join(__dirname, '../../../../examples');

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fs.promises.readFile(fullPath);
    const module = await WebAssembly.compile(buffer);
    return module;
};

describe.each([
    // force column layout
    'off',
    'info',
    'warn',
    'error',
    'debug',
    'trace',
])('Testing logging level', (level) => {
    test('Testing logging level', async () => {
        // arrange
        // console.log = jest.fn();
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const faas = new FaaS(marine, greeting, 'srv', undefined, { WASM_LOG: level });
        await faas.init();

        // act
        faas.call('logging', '{}', undefined);

        // assert
        expect(console.log).toBeCalledTimes(1);
        expect(console.log).toHaveBeenNthCalledWith(1, level);
    });
});

import fs from 'fs';
import path from 'path';
import tmp from 'tmp';
import download from 'download';
import { FaaS } from '../FaaS';
import { callAvm } from '@fluencelabs/avm';

const fsPromises = fs.promises;

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fsPromises.readFile(fullPath);
    const module = await WebAssembly.compile(buffer);
    return module;
};

const redisDownloadUrl = 'https://github.com/fluencelabs/redis/releases/download/v0.14.0_w/redis.wasm';
const sqliteDownloadUrl = 'https://github.com/fluencelabs/sqlite/releases/download/v0.14.0_w/sqlite3.wasm';

const examplesDir = path.join(__dirname, '../../../../examples');

let tempFile: tmp.FileResult;

describe('Fluence app service tests', () => {
    beforeEach(() => {
        tempFile = tmp.fileSync();
    });

    afterEach(() => {
        tempFile?.removeCallback();
    });

    it('Testing greeting service', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const faas = new FaaS(marine, greeting, 'srv');
        await faas.init();

        // act
        const res = JSON.parse(faas.call('greeting', '{"name": "test"}', undefined));

        // assert
        expect(res).toMatchObject({
            error: '',
            result: 'Hi, test',
        });
    });

    it('Testing greeting service', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const faas = new FaaS(marine, greeting, 'srv');
        await faas.init();

        // act
        const greetingRecordResult = JSON.parse(faas.call('greeting_record', '{}', undefined));

        // assert
        expect(greetingRecordResult).toMatchObject({
            error: '',
            result: {
                str: 'Hello, world!',
                num: 42,
            },
        });
    });

    it('Running avm through FaaS infrastructure', async () => {
        // arrange
        const avmPackagePath = require.resolve('@fluencelabs/avm');
        const avm = await loadWasmModule(path.join(path.dirname(avmPackagePath), 'avm.wasm'));
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));

        const testAvmFaaS = new FaaS(marine, avm, 'avm');
        await testAvmFaaS.init();

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = await callAvm(
            (arg: string) => testAvmFaaS.call('invoke', arg, undefined),
            vmPeerId,
            vmPeerId,
            s,
            b(''),
            b(''),
            [],
        );
        await testAvmFaaS.terminate();

        // assert
        expect(res).toMatchObject({
            retCode: 0,
            errorMessage: '',
        });
    });

    it('Testing sqlite wasm', async () => {
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(sqliteDownloadUrl);
        const sqlite = await WebAssembly.compile(buf);

        const marine = new FaaS(control, sqlite, 'sqlite');
        await marine.init();

        const result1 = marine.call('sqlite3_open_v2', a(':memory:', 6, ''), undefined);
        console.log(result1);
    });
});

const a = (...args: any[]) => {
    console.log(JSON.stringify(args));
    return JSON.stringify(args);
};

import fs from 'fs';
import path from 'path';
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

const redisDownloadUrl = 'https://github.com/fluencelabs/redis/releases/download/v0.10.0_w/redis.wasm';
const sqliteDownloadUrl = 'https://github.com/fluencelabs/sqlite/releases/download/v0.15.0_w/sqlite3.wasm';

const examplesDir = path.join(__dirname, '../../../../examples');

describe('Fluence app service tests', () => {
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

        let result;

        result = doCall(marine, 'sqlite3_open_v2', ':memory:', 6, '');
        const dbHandle = result.db_handle;
        result = doCall(marine, 'sqlite3_exec', dbHandle, 'CREATE VIRTUAL TABLE users USING FTS5(body)', 0, 0);
        result = doCall(
            marine,
            'sqlite3_exec',
            dbHandle,
            "INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')",
            0,
            0,
        );
        result = doCall(marine, 'sqlite3_exec', dbHandle, "SELECT * FROM users WHERE users MATCH 'A* OR B*'", 0, 0);

        console.log(result);

        // TODO:
        expect(1).toBe(2);
    });

    it('Testing redis wasm', async () => {
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(redisDownloadUrl);
        const redis = await WebAssembly.compile(buf);

        const marine = new FaaS(control, redis, 'redis');
        await marine.init();

        const result1 = doCall(marine, 'invoke', 'SET A 10');
        const result2 = doCall(marine, 'invoke', 'SADD B 20');
        const result3 = doCall(marine, 'invoke', 'GET A');
        const result4 = doCall(marine, 'invoke', 'SMEMBERS B');
        const result5 = doCall(
            marine,
            'invoke',
            "eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0",
        );

        expect(result1).toBe('+OK\r\n');
        expect(result2).toBe(':1\r\n');
        expect(result3).toBe('$2\r\n10\r\n');
        expect(result4).toBe('*1\r\n$2\r\n20\r\n');
        expect(result5).toBe(':93\r\n');
    });
});

const doCall = (marine: FaaS, fn: string, ...args: any[]) => {
    const argsStr = JSON.stringify(args);
    const rawRes = marine.call(fn, argsStr, undefined);
    const res = JSON.parse(rawRes);
    if (res.error && res.error.length > 0) {
        throw new Error(`call failed args: ${argsStr}, res: ${rawRes}`);
    }
    return res.result;
};

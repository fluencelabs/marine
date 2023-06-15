import { jest } from '@jest/globals';
import { defaultImport } from 'default-import';
import { promises as fsPromises } from 'fs';
import { createRequire } from 'module';
import * as path from 'path';
import * as url from 'url';
import downloadRaw from 'download';
import { MarineService } from '../MarineService.js';
import { callAvm } from '@fluencelabs/avm';
import { JSONArray, JSONObject } from '../types.js';
import exp = require("constants");

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const require = createRequire(import.meta.url);
const download = defaultImport(downloadRaw);

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

const loadWasmBytes = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fsPromises.readFile(fullPath);
    return new Uint8Array(buffer);
};

const redisDownloadUrl = 'https://github.com/fluencelabs/redis/releases/download/v0.15.0_w/redis.wasm';
const sqliteDownloadUrl = 'https://github.com/fluencelabs/sqlite/releases/download/sqlite-wasm-v0.18.1/sqlite3.wasm';

const examplesDir = path.join(__dirname, '../../../../examples');
const wasmTestsDir = path.join(__dirname, '../../../../marine/tests/wasm_tests');

const dontLog = () => {};

describe('Fluence app service tests', () => {
    it('Testing greeting service', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const marineService = new MarineService(marine, [{name: 'srv', wasm_bytes: greeting}], 'srv', dontLog);
        await marineService.init();

        // act
        const res = marineService.call('greeting', ['test'], undefined);

        // assert
        expect(res).toBe('Hi, test');
    });

    it('Testing greeting service with object args', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const marineService = new MarineService(marine, [{name: 'srv', wasm_bytes: greeting}], 'srv', dontLog);
        await marineService.init();

        // act
        const res = marineService.call('greeting', { name: 'test' }, undefined);

        // assert
        expect(res).toBe('Hi, test');
    });

    it('Testing greeting service with records', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const marineService = new MarineService(marine, [{name: 'srv', wasm_bytes: greeting}], 'srv', dontLog);
        await marineService.init();

        // act
        const greetingRecordResult = marineService.call('greeting_record', [], undefined);
        const voidResult: any = marineService.call('void_fn', [], undefined);

        // assert
        expect(greetingRecordResult).toMatchObject({
            str: 'Hello, world!',
            num: 42,
        });
        expect(voidResult).toStrictEqual(null);
    });

    it('Testing multi-module service', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const donkey = await loadWasmBytes(
            path.join(examplesDir, './motivational-example/artifacts/donkey.wasm'),
        );
        const shrek = await loadWasmBytes(
            path.join(examplesDir, './motivational-example/artifacts/shrek.wasm'),
        );

        let service = [
            {name: "donkey", wasm_bytes: donkey},
            {name: "shrek", wasm_bytes: shrek}
        ];

        const marineService = new MarineService(marine, service, 'srv', dontLog);
        await marineService.init();

        // act
        const call_result = marineService.call('greeting', ["test"], undefined);

        // assert
        expect(call_result).toMatchObject(["Shrek: hi, test", "Donkey: hi, test"]);
    });

    it('Running avm through Marine infrastructure', async () => {
        // arrange
        const avmPackagePath = require.resolve('@fluencelabs/avm');
        const avm = await loadWasmBytes(path.join(path.dirname(avmPackagePath), 'avm.wasm'));
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));

        const testAvmInMarine = new MarineService(marine, [{name: 'avm', wasm_bytes: avm}], 'avm', dontLog);
        await testAvmInMarine.init();

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = await callAvm(
            (args: JSONArray | JSONObject): unknown => testAvmInMarine.call('invoke', args, undefined),
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
        await testAvmInMarine.terminate();

        // assertMarine
        expect(res).toMatchObject({
            retCode: 0,
            errorMessage: '',
        });
    });

    it('Testing sqlite wasm', async () => {
        jest.setTimeout(10000);
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(sqliteDownloadUrl);

        const marine = new MarineService(control, [{name: 'sqlite', wasm_bytes: buf}], 'sqlite', dontLog);
        await marine.init();

        let result: any;

        result = marine.call('sqlite3_open_v2', [':memory:', 6, ''], undefined);
        const dbHandle = result.db_handle;
        result = marine.call(
            'sqlite3_exec',
            [dbHandle, 'CREATE VIRTUAL TABLE users USING FTS5(body)', 0, 0],
            undefined,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });

        result = marine.call(
            'sqlite3_exec',
            [dbHandle, "INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')", 0, 0],
            undefined,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });

        result = marine.call(
            'sqlite3_exec',
            [dbHandle, "SELECT * FROM users WHERE users MATCH 'A* OR B*'", 0, 0],
            undefined,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });
    });

    it.skip('Testing redis wasm', async () => {
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(redisDownloadUrl);
       // const redis = await WebAssembly.compile(buf);

        const marine = new MarineService(control, [{name: "redis", wasm_bytes: buf}], 'redis', dontLog);
        await marine.init();

        const result1 = marine.call('invoke', ['SET A 10'], undefined);
        const result2 = marine.call('invoke', ['SADD B 20'], undefined);
        const result3 = marine.call('invoke', ['GET A'], undefined);
        const result4 = marine.call('invoke', ['SMEMBERS B'], undefined);
        const result5 = marine.call(
            'invoke',
            ["eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0"],
            undefined,
        );

        expect(result1).toBe('+OK\r\n');
        expect(result2).toBe(':1\r\n');
        expect(result3).toBe('$2\r\n10\r\n');
        expect(result4).toBe('*1\r\n$2\r\n20\r\n');
        expect(result5).toBe(':93\r\n');
    });

    it('Testing service which fails', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const failing = await loadWasmBytes(path.join(examplesDir, './failing/artifacts/failing.wasm'));

        const marineService = new MarineService(marine, [{name: 'srv', wasm_bytes: failing}], 'srv', dontLog);
        await marineService.init();


        expect(() => marineService.call('failing', [], undefined))
            .toThrow(new Error("engine error: Execution error: `call-core 6` failed while calling the local or import function `failing`"));

    });

    it('Checking error when calling non-existent function', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(path.join(examplesDir, './failing/artifacts/failing.wasm'));


        const marineService = new MarineService(marine, [{name: "srv", wasm_bytes: greeting}], 'srv', dontLog);
        await marineService.init();

        // act
        try {
            await marineService.call('do_not_exist', [], undefined);
            // should never succeed
            expect(true).toBe(false);
        } catch (e) {
            // assert
            expect(e).toBeInstanceOf(Error);
            expect((e as Error).message).toBe(
                'function with name `do_not_exist` is missing',
            );
        }
    });

    it('Checking arguments passing', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const arguments_passing_pure = await loadWasmBytes(path.join(wasmTestsDir, "./arguments_passing/artifacts/arguments_passing_pure.wasm"))
        const arguments_passing_effector = await loadWasmBytes(path.join(wasmTestsDir, "./arguments_passing/artifacts/arguments_passing_effector.wasm"))

        let service = [
            {
                name: "arguments_passing_effector",
                wasm_bytes: arguments_passing_effector
            },
            {
                name: "arguments_passing_pure",
                wasm_bytes: arguments_passing_pure
            }
        ]

        const marineService = new MarineService(marine, service, 'srv', dontLog);
        await marineService.init();

        const test = (func_name: string) => {
            const expected_result = [
                0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0,
                0, 65, 1, 153, 154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110, 99, 101,
                19, 55, 0, 1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0,
                0, 0, 0, 0, 65, 1, 153, 154, 64, 34, 51, 51, 51, 51, 51, 51, 102, 108, 117, 101, 110,
                99, 101, 19, 55];

            const args1 = {
                "arg_0": 0,
                "arg_1": 1,
                "arg_2": 2,
                "arg_3": 3,
                "arg_4": 4,
                "arg_5": 5,
                "arg_6": 6,
                "arg_7": 7,
                "arg_8": 8.1,
                "arg_9": 9.1,
                "arg_10": "fluence",
                "arg_11": [0x13, 0x37],
            };
            const result1 = marineService.call(func_name, args1, null);
            expect(result1).toBe(expected_result)

            let args2 = [
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8.1,
                9.1,
                "fluence",
                [0x13, 0x37]
            ];
            const result2 = marineService.call(func_name, args2, null)
            expect(result2).toBe(expected_result);
        };

        test("all_types");
        test("all_types_ref")
    });
});

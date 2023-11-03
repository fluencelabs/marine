import { jest } from '@jest/globals';
import { defaultImport } from 'default-import';
import { promises as fsPromises } from 'fs';
import { createRequire } from 'module';
import * as path from 'path';
import * as url from 'url';
import downloadRaw from 'download';
import { MarineService } from '../MarineService.js';
import { callAvm } from '@fluencelabs/avm';
import {JSONArray, JSONObject, CallParameters, SecurityTetraplet, defaultCallParameters} from '../types.js';
import {MarineServiceConfig, Env, Args, ModuleDescriptor} from '../config.js';
import exp = require("constants");

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const require = createRequire(import.meta.url);
const download = defaultImport(downloadRaw);

// PeerId and secret key are deterministically derived from seed "host_peer_id"
const VM_PEER_ID = "12D3KooWHgS5pbwe87KjDHbwRwQgrhXX8KU4f8FMU2qrVQ35fF1t";
const VM_SECRET_KEY = Uint8Array.from([208, 43, 27, 28, 203, 241, 229, 251, 222, 32, 195, 215, 64, 54, 104, 141, 35, 147, 166, 11, 1, 231, 42, 78, 56, 169, 76, 66, 90, 183, 216, 217])
const VM_KEY_FORMAT = 0 // Ed25519

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

const redisDownloadUrl = 'https://github.com/fluencelabs/redis/releases/download/v0.15.0_w/redis.wasm';
const sqliteDownloadUrl = 'https://github.com/fluencelabs/sqlite/releases/download/sqlite-wasm-v0.18.1/sqlite3.wasm';

const examplesDir = path.join(__dirname, '../../../../examples');
const wasmTestsDir = path.join(__dirname, '../../../../marine/tests/wasm_tests');

const dontLog = () => {};


const createModuleDescriptor = (name: string): ModuleDescriptor  => {
    return {
        import_name: name,
        config: defaultModuleConfig,
    }
}
const createSimpleServiceConfig = (name: string): MarineServiceConfig => {
    return {
        modules_config: [createModuleDescriptor(name)]
    }
};

describe('Fluence app service tests', () => {
    it('Testing greeting service', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const config = createSimpleServiceConfig('greeting');
        const modules = {greeting: greeting}

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();

        // act
        const res = marineService.call('greeting', ['test'], defaultCallParameters);

        // assert
        expect(res).toBe('Hi, test');
    });

    it('Testing greeting service with object args', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const config = createSimpleServiceConfig('greeting');
        const modules = {greeting: greeting}

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();

        // act
        const res = marineService.call('greeting', { name: 'test' }, defaultCallParameters);

        // assert
        expect(res).toBe('Hi, test');
    });

    it('Testing greeting service with records', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const config = createSimpleServiceConfig('greeting');
        const modules = {greeting: greeting}

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();

        // act
        const greetingRecordResult = marineService.call('greeting_record', [], defaultCallParameters);
        const voidResult: any = marineService.call('void_fn', [], defaultCallParameters);

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


        const config = {
            modules_config: [
                createModuleDescriptor('donkey'),
                createModuleDescriptor('shrek')
            ]
        };

        const modules = {
            donkey: donkey,
            shrek: shrek,
        }
        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();

        // act
        const call_result = marineService.call('greeting', ["test"], defaultCallParameters);

        // assert
        expect(call_result).toMatchObject(["Shrek: hi, test", "Donkey: hi, test"]);
    });

    it('Running avm through Marine infrastructure', async () => {
        // arrange
        const avmPackagePath = require.resolve('@fluencelabs/avm');
        const avm = await loadWasmBytes(path.join(path.dirname(avmPackagePath), 'avm.wasm'));
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));

        const config = createSimpleServiceConfig('avm');
        const modules = {avm: avm}

        const testAvmInMarine = new MarineService(marine, 'avm', dontLog, config, modules);
        await testAvmInMarine.init();

        const s = `(seq
            (par 
                (call "${VM_PEER_ID}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${VM_PEER_ID}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = await callAvm(
            (args: JSONArray | JSONObject): unknown => testAvmInMarine.call('invoke', args, defaultCallParameters),
            {
                keyFormat: VM_KEY_FORMAT,
                particleId: "",
                secretKeyBytes: VM_SECRET_KEY,
                currentPeerId: VM_PEER_ID,
                initPeerId: VM_PEER_ID,
                timestamp: Date.now(),
                ttl: 10000
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

        const config = createSimpleServiceConfig('sqlite');
        const modules = {sqlite: new Uint8Array(buf)}

        const marine = new MarineService(control, 'sqlite', dontLog, config, modules);
        await marine.init();

        let result: any;

        result = marine.call('sqlite3_open_v2', [':memory:', 6, ''], defaultCallParameters);
        const dbHandle = result.db_handle;
        result = marine.call(
            'sqlite3_exec',
            [dbHandle, 'CREATE VIRTUAL TABLE users USING FTS5(body)', 0, 0],
            defaultCallParameters,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });

        result = marine.call(
            'sqlite3_exec',
            [dbHandle, "INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')", 0, 0],
            defaultCallParameters,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });

        result = marine.call(
            'sqlite3_exec',
            [dbHandle, "SELECT * FROM users WHERE users MATCH 'A* OR B*'", 0, 0],
            defaultCallParameters,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });
    });

    it.skip('Testing redis wasm', async () => {
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(redisDownloadUrl);

        const config = createSimpleServiceConfig('redis');
        const modules = {redis: new Uint8Array(buf)};

        const marine = new MarineService(control, 'redis', dontLog, config, modules);
        await marine.init();

        const result1 = marine.call('invoke', ['SET A 10'], defaultCallParameters);
        const result2 = marine.call('invoke', ['SADD B 20'], defaultCallParameters);
        const result3 = marine.call('invoke', ['GET A'], defaultCallParameters);
        const result4 = marine.call('invoke', ['SMEMBERS B'], defaultCallParameters);
        const result5 = marine.call(
            'invoke',
            ["eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0"],
            defaultCallParameters,
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

        const config = createSimpleServiceConfig('failing');
        const modules = {failing: failing};

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();


        expect(() => marineService.call('failing', [], defaultCallParameters))
            .toThrow(new Error("engine error: Execution error: `call-core 6` failed while calling the local or import function `failing`: Unrecognized error: Failed to apply func"));

    });

    it('Checking error when calling non-existent function', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmBytes(path.join(examplesDir, './failing/artifacts/failing.wasm'));

        const config = createSimpleServiceConfig('greeting');
        const modules = {greeting: greeting}

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();

        // act
        try {
            await marineService.call('do_not_exist', [], defaultCallParameters);
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

        const config = {
            modules_config: [
                createModuleDescriptor('arguments_passing_effector'),
                createModuleDescriptor('arguments_passing_pure'),
            ]
        }

        const modules = {
            arguments_passing_effector: arguments_passing_effector,
            arguments_passing_pure: arguments_passing_pure,
        }

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
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
            const result1 = marineService.call(func_name, args1, defaultCallParameters);
            expect(result1).toStrictEqual(expected_result)

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
            const result2 = marineService.call(func_name, args2, defaultCallParameters)
            expect(result2).toStrictEqual(expected_result);
        };

        test("all_types");
        test("all_ref_types")
    });

    it('Checking call_parameters passing', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const call_parameters_wasm = await loadWasmBytes(path.join(examplesDir, './call_parameters/artifacts/call_parameters.wasm'),)

        const config= createSimpleServiceConfig('call_parameters');
        const modules = {call_parameters: call_parameters_wasm};

        const marineService = new MarineService(marine, 'srv', dontLog, config, modules);
        await marineService.init();

        const init_peer_id_2 = "init_peer_id";
        const service_id = "service_id";
        const service_creator_peer_id = "service_creator_peer_id";
        const host_id = "host_id";
        const particle_id = "particle_id";

        const tetraplet: SecurityTetraplet = {
                function_name: "some_func_name",
                json_path: "some_json_path",
                peer_pk: "peer_pk",
                service_id: "service_id"
        }

        const tetraplets = [[tetraplet]]

        const call_parameters: CallParameters = {
            init_peer_id: init_peer_id_2,
            service_id: service_id,
            service_creator_peer_id: service_creator_peer_id,
            host_id: host_id,
            particle_id: particle_id,
            tetraplets: tetraplets,
        };

        const expected_result =
            "init_peer_id\n" +
            "service_id\n" +
            "service_creator_peer_id\n" +
            "host_id\n" +
            "particle_id\n" +
            "[[SecurityTetraplet { peer_pk: \"peer_pk\", service_id: \"service_id\", function_name: \"some_func_name\", json_path: \"some_json_path\" }]]";

        const result1 = marineService.call("call_parameters", [], call_parameters);
        expect(result1).toStrictEqual(expected_result)

    });
});

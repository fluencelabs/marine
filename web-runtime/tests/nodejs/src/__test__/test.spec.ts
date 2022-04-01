import {
    BackgroundFaaSConsumer,
    loadWasm,
    defaultNames,
    runAvm,
    bufferToSharedArrayBuffer,
} from '@fluencelabs/marine-js';
import fs from 'fs';
import path from 'path';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

describe('Nodejs integration tests', () => {
    it('Smoke test', async () => {
        const testRunner = new BackgroundFaaSConsumer();
        try {
            // arrange
            const avm = await loadWasm(defaultNames.avm);
            const marine = await loadWasm(defaultNames.marine);
            // WebAssembly.compile(new Uint8Array(avm));
            // const marine = fs.readFileSync(__dirname + '/../../node_modules/@fluencelabs/avm/dist/avm.wasm');
            // const marinePath = require.resolve('@fluencelabs/marine-js');
            // const marine = fs.readFileSync(path.join(path.dirname(marinePath), 'marine-js.wasm'));
            // const avmPath = require.resolve('@fluencelabs/avm');
            // const avm = fs.readFileSync(path.join(path.dirname(avmPath), 'avm.wasm'));

            // const marineSab = bufferToSharedArrayBuffer(marine);
            // const avmSab = bufferToSharedArrayBuffer(avm);

            // WebAssembly.compile(new Uint8Array(marineSab));
            // WebAssembly.compile(new Uint8Array(avmSab));
            await testRunner.init(marine);
            await testRunner.createService(avm, 'avm');

            const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

            // act
            const params = { initPeerId: vmPeerId, currentPeerId: vmPeerId };
            const res = await runAvm(testRunner, s, b(''), b(''), params, []);
            await testRunner.terminate();

            // assert
            expect(res).toMatchObject({
                retCode: 0,
                errorMessage: '',
            });
        } finally {
            testRunner.terminate();
        }
    });
});

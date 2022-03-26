import fs from 'fs';
import path from 'path';
import { runAvm } from '../avmHelpers';
import { FluenceAppService } from '../FluenceAppService';

const fsPromises = fs.promises;

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

describe('Fluence app service tests', () => {
    it('Running avm through FaaS infrastructure', async () => {
        // arrange
        const avmPackagePath = require.resolve('@fluencelabs/avm');
        const avmFilePath = path.join(path.dirname(avmPackagePath), 'avm.wasm');
        const avmBuffer = await fsPromises.readFile(avmFilePath);
        const avm = await WebAssembly.compile(avmBuffer);

        const marineFilePath = path.join(__dirname, '../../dist/marine-js.wasm');
        const marineBuffer = await fsPromises.readFile(marineFilePath);
        const marine = await WebAssembly.compile(marineBuffer);

        const testAvmFaaS = new FluenceAppService(marine, avm, 'avm');
        await testAvmFaaS.init();

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const params = { initPeerId: vmPeerId, currentPeerId: vmPeerId };
        const res = await runAvm(testAvmFaaS, s, b(''), b(''), params, []);
        await testAvmFaaS.terminate();

        // assert
        expect(res).toMatchObject({
            retCode: 0,
            errorMessage: '',
        });
    });
});

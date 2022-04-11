import { FluenceAppService, loadWasm, defaultNames } from '@fluencelabs/marine-js';
import { callAvm } from '@fluencelabs/avm';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

describe('Nodejs integration tests', () => {
    it('Smoke test', async () => {
        const testRunner = new FluenceAppService();
        try {
            // arrange
            const avm = await loadWasm(defaultNames.avm);
            const marine = await loadWasm(defaultNames.marine);
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
            const res = await callAvm(
                (arg: string) => testRunner.callService('avm', 'invoke', arg, undefined),
                vmPeerId,
                vmPeerId,
                s,
                b(''),
                b(''),
                [],
            );
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

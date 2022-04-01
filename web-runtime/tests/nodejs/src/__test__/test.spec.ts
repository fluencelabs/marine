import { BackgroundFaaSConsumer, loadWasm, defaultNames, runAvm } from '@fluencelabs/marine-js';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

describe('Nodejs integration tests', () => {
    it('Smoke test', async () => {
        // arrange
        const avm = await loadWasm(defaultNames.avm);
        const marine = await loadWasm(defaultNames.marine);
        const testRunner = new BackgroundFaaSConsumer();
        await testRunner.init(marine);
        await testRunner.createService(avm, 'avm');

        // await testRunner.init('off');

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
    });
});

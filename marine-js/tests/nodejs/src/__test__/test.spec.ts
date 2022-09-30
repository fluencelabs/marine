import { FluenceAppService, loadDefaults, defaultNames } from '@fluencelabs/marine-js';
import { callAvm } from '@fluencelabs/avm';
import {JSONArray, JSONObject} from "../../../../../../aquavm/avm/client/src";

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

describe('Nodejs integration tests', () => {
    it('Smoke test', async () => {
        const testRunner = new FluenceAppService();
        try {
            // arrange
            const wasms = await loadDefaults();
            await testRunner.init(wasms.marine);
            await testRunner.createService(wasms.avm, 'avm');

            const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

            // act
            const res = await callAvm(
                (args: JSONArray | JSONObject): unknown => testRunner.callService('avm', 'invoke', args, undefined),
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

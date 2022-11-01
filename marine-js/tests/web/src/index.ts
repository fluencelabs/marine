import { FluenceAppService, loadWasmFromServer, defaultNames } from '@fluencelabs/marine-js';
import {callAvm, JSONValue} from '@fluencelabs/avm';
import { toUint8Array } from 'js-base64';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return toUint8Array(s);
};

const main = async () => {
    const testRunner = new FluenceAppService();
    const avm = await loadWasmFromServer(defaultNames.avm.file);
    const marine = await loadWasmFromServer(defaultNames.marine.file);
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
        (args: JSONArray | JSONValue) => testRunner.callService('avm', 'invoke', args, undefined),
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

    return res;
};

// @ts-ignore
window.MAIN = main;

import { FluenceAppService, loadWasm, defaultNames } from '@fluencelabs/marine-js';
import { callAvm } from '@fluencelabs/avm';
import { toUint8Array } from 'js-base64';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return toUint8Array(s);
};

const main = async () => {
    const testRunner = new FluenceAppService();
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

    return res;
};

// @ts-ignore
window.MAIN = main;

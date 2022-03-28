import { CallResultsArray, InterpreterResult, CallRequest } from '@fluencelabs/avm-runner-interface';
import { FluenceAppService } from './FluenceAppService';

const decoder = new TextDecoder();

type FaasCall =
    | ((function_name: string, args: string, callParams: any) => string)
    | ((function_name: string, args: string, callParams: any) => Promise<string>);

export const runAvm = async (
    faasCall: FaasCall,
    air: string,
    prevData: Uint8Array,
    data: Uint8Array,
    params: {
        initPeerId: string;
        currentPeerId: string;
    },
    callResults: CallResultsArray,
): Promise<InterpreterResult> => {
    try {
        const callResultsToPass: any = {};
        for (let [k, v] of callResults) {
            callResultsToPass[k] = {
                ret_code: v.retCode,
                result: v.result,
            };
        }

        const paramsToPass = {
            init_peer_id: params.initPeerId,
            current_peer_id: params.currentPeerId,
        };

        const avmArg = JSON.stringify([
            air,
            Array.from(prevData),
            Array.from(data),
            paramsToPass,
            Array.from(Buffer.from(JSON.stringify(callResultsToPass))),
        ]);

        const rawResult = await faasCall('invoke', avmArg, undefined);

        let result: any;
        try {
            result = JSON.parse(rawResult);
        } catch (ex) {
            throw 'call_module result parsing error: ' + ex + ', original text: ' + rawResult;
        }

        if (result.error !== '') {
            throw 'call_module returned error: ' + result.error;
        }

        result = result.result;

        const callRequestsStr = decoder.decode(new Uint8Array(result.call_requests));
        let parsedCallRequests;
        try {
            if (callRequestsStr.length === 0) {
                parsedCallRequests = {};
            } else {
                parsedCallRequests = JSON.parse(callRequestsStr);
            }
        } catch (e) {
            throw "Couldn't parse call requests: " + e + '. Original string is: ' + callRequestsStr;
        }

        let resultCallRequests: Array<[key: number, callRequest: CallRequest]> = [];
        for (const k in parsedCallRequests) {
            const v = parsedCallRequests[k];

            let arguments_;
            let tetraplets;
            try {
                arguments_ = JSON.parse(v.arguments);
            } catch (e) {
                throw "Couldn't parse arguments: " + e + '. Original string is: ' + arguments_;
            }

            try {
                tetraplets = JSON.parse(v.tetraplets);
            } catch (e) {
                throw "Couldn't parse tetraplets: " + e + '. Original string is: ' + tetraplets;
            }

            resultCallRequests.push([
                k as any,
                {
                    serviceId: v.service_id,
                    functionName: v.function_name,
                    arguments: arguments_,
                    tetraplets: tetraplets,
                },
            ]);
        }
        return {
            retCode: result.ret_code,
            errorMessage: result.error_message,
            data: result.data,
            nextPeerPks: result.next_peer_pks,
            callRequests: resultCallRequests,
        };
    } catch (e) {
        return {
            retCode: -1,
            errorMessage: 'marine-js call failed, ' + e,
        } as any;
    }
};

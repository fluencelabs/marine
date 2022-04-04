import { expose } from 'threads';
import { Envs, FaaSConfig } from './config';
import { FaaS } from './FaaS';
import { IFluenceAppService } from './types';

const faasInstances = new Map<string, FaaS>();
let controlModule: WebAssembly.Module;

const toExpose: IFluenceAppService = {
    init: async (marineWasm: SharedArrayBuffer): Promise<void> => {
        controlModule = await WebAssembly.compile(new Uint8Array(marineWasm));
    },
    createService: async (
        wasm: SharedArrayBuffer,
        serviceId: string,
        faaSConfig?: FaaSConfig,
        envs?: Envs,
    ): Promise<void> => {
        const service = await WebAssembly.compile(new Uint8Array(wasm));
        const faas = new FaaS(controlModule, service, serviceId, faaSConfig, envs);
        await faas.init();
        faasInstances.set(serviceId, faas);
    },
    terminate: async (): Promise<void> => {
        faasInstances.forEach((val, key) => {
            val.terminate();
        });
    },
    callService: async (serviceId: string, functionName: string, args: string, callParams: any): Promise<string> => {
        const faas = faasInstances.get(serviceId);
        if (!faas) {
            throw new Error(`service with id=${serviceId} not found`);
        }

        return faas.call(functionName, args, callParams);
    },
};

expose(toExpose);

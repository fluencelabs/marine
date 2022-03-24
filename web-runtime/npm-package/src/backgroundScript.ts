import { expose } from 'threads';
import { Envs, FaaSConfig } from './config';
import { FluenceAppService } from './FluenceAppService';
import { BackgroundFaaS } from './types';

const instances = new Map<string, FluenceAppService>();
let marineModule: WebAssembly.Module;

const toExpose: BackgroundFaaS = {
    init: async (marineWasm: SharedArrayBuffer): Promise<void> => {
        marineModule = await WebAssembly.compile(new Uint8Array(marineWasm));
    },
    createService: async (
        wasm: SharedArrayBuffer,
        serviceId: string,
        faaSConfig?: FaaSConfig,
        envs?: Envs,
    ): Promise<void> => {
        const service = await WebAssembly.compile(new Uint8Array(wasm));
        const faas = new FluenceAppService(marineModule, service, serviceId, faaSConfig, envs);
        instances.set(serviceId, faas);
    },
    terminate: async (): Promise<void> => {
        instances.forEach((val, key) => {
            val.terminate();
        });
    },
    callService: async (serviceId: string, functionName: string, args: string, callParams: any): Promise<string> => {
        const faas = instances.get(serviceId);
        if (!faas) {
            throw new Error(`service with id=${serviceId} not found`);
        }

        return faas.call(functionName, args, callParams);
    },
};

expose(toExpose);

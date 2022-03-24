import { expose } from 'threads';
import { FluenceAppService } from './FluenceAppService';
import { MarineJsExpose } from './types';

const instances = new Map<string, FluenceAppService>();

const toExpose: MarineJsExpose = {
    init: instance.init.bind(instance),
    terminate: instance.terminate.bind(instance),
    call: instance.call.bind(instance),
};

expose(toExpose);

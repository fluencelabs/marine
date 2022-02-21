import * as AvmRunnerMarinejs from 'avm-runner-marinejs'
import * as AvmRunnerBindgen from 'avm-runner-bindgen'

import { CallResultsArray, LogLevel } from '@fluencelabs/avm-runner-interface';

import {readFileSync} from "fs";
import * as buffer from "buffer";
import * as assert from "assert";

var Benchmark = require('benchmark');
const path = require('path');

const defaultAvmFileName = 'avm.wasm';
const defaultMarineFileName = 'marine-js.wasm';
const avmMarinejsPackageName = 'avm-marinejs';
const avmBindgenPackageName = 'avm-bindgen';
const marinePackageName = '@fluencelabs/marine-js';

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const airScript = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

const vmParams = { initPeerId: vmPeerId, currentPeerId: vmPeerId };


const avmMarinejsPackagePath = require.resolve(avmMarinejsPackageName);
const avmMarinejsFilePath = path.join(path.dirname(avmMarinejsPackagePath), defaultAvmFileName);

const avmBindgenPackagePath = require.resolve(avmBindgenPackageName);
const avmBindgenFilePath = path.join(path.dirname(avmBindgenPackagePath), defaultAvmFileName);

const marinePackagePath = require.resolve(marinePackageName);
const marineFilePath = path.join(path.dirname(marinePackagePath), defaultMarineFileName);


const marinejsLoadingMethod: AvmRunnerMarinejs.wasmLoadingMethod = {
    method: "read-from-fs",
    filePaths: {
        avm: avmMarinejsFilePath,
        marine: marineFilePath,
    }
};

const bindgenLoadingMethod: AvmRunnerBindgen.wasmLoadingMethod = {
    method: "read-from-fs",
    filePath: avmBindgenFilePath,
};

var enc = new TextEncoder(); // always utf-8

function generateData(size) {
    let data = {
        trace: new Array<{par: [number, number]}>(),
        streams: {},
        version: "0.2.2",
        lcid: 0,
        r_streams: {}
    }
    for (let i = 0; i < size; i++) {
        data.trace.push({"par": [0,0]});
    }

    return enc.encode(JSON.stringify(data))
}

const main = async () => {
    var marinejsCalls = 0;
    var bindgenCalls = 0;
    var suite = new Benchmark.Suite;
    var globalMarinejsRunner = new AvmRunnerMarinejs.AvmRunnerBackground(marinejsLoadingMethod);
    await globalMarinejsRunner.init('info');

    let globalBindgenRunner = new AvmRunnerBindgen.AvmRunnerBackground(bindgenLoadingMethod);
    await globalBindgenRunner.init('info');

    let seqNullAir = new Array<string>();
    let seqNullResuts = new Array<{ bindgen: number, marinejs: number }>();
    let SEQ_NULL_MAX_DEPTH = 20;
    for (let i = 1; i <= SEQ_NULL_MAX_DEPTH; i++) {
        let air = readFileSync("seq_null_" + i + ".air").toString();
        air = "(null)";
        //let prev_data = new Uint8Array(1 << i);
        let prev_data = generateData((1<<i));
        //prev_data = new Uint8Array(1 << i);
        console.log("at depth " + i + " data size is " + prev_data.length);
        suite.add(
            "bindgen#seq_null_" + i, {
                'defer': true,
                'engine': "bindgen",
                'treeDepth': i,
                'fn': function (deferred: any) {
                    globalBindgenRunner.run(air, prev_data, prev_data, vmParams, []).then((result) => {
                        deferred.resolve()
                    })
                },
            }
        );
        suite.add(
            "marinejs#seq_null_" + i, {
                'engine': "marinejs",
                'treeDepth': i,
                'defer': true,
                'fn': function (deferred: any) {
                    globalMarinejsRunner.run(air, prev_data, prev_data, vmParams, []).then((result) => {
                       /* if(result.retCode != 0) {
                            console.error(result)
                            process.exit(1)
                        }*/
                        deferred.resolve();
                    })
                },
            }
        );

        seqNullResuts.push({bindgen: 0.0, marinejs: 0.0})
    }


    suite/*.add('marinejs#instantiate', {
            'defer': true,
            'initCount': 1,
            'minSamples': 30,
            'fn': function (deferred: any) {
                let runner = new AvmRunnerMarinejs.AvmRunnerBackground(marinejsLoadingMethod);
                runner.init('off').then(() => deferred.resolve())
            },
        }
    )
    .add('bindgen#instantiate', {
        'defer': true,
        'initCount': 1,
        'minSamples': 30,
        'fn': function (deferred: any) {
            let runner = new AvmRunnerBindgen.AvmRunnerBackground(bindgenLoadingMethod);
            runner.init('off').then(() => deferred.resolve())
        },
    })
        .add('marinejs#call-little-data', {
            'defer': true,
            'initCount': 1,
            'minSamples': 60,
            'fn': function (deferred: any) {
                globalMarinejsRunner.run(airScript, new Uint8Array, new Uint8Array, vmParams, []).then(() => deferred.resolve())
            },
        })
        .add('bindgen#call-little-data', {
            'defer': true,
            'initCount': 1,
            'minSamples': 60,
            'fn': function (deferred: any) {
                globalBindgenRunner.run(airScript, new Uint8Array, new Uint8Array, vmParams, []).then(() => deferred.resolve())
            },
        })
        */
        .on('start', function() {
            console.log('bench started');
        })
        .on('cycle', function(event: any) {
            if(event.target.engine) {
                let depth = event.target.treeDepth
                let engine = event.target.engine;
                seqNullResuts[depth-1][engine] = event.target.hz;
            }

            console.log(String(event.target));
        })
        .on('complete', function() {
            console.log(seqNullResuts)
            for (let i = 1; i <= SEQ_NULL_MAX_DEPTH; i++) {
                let relation = seqNullResuts[i-1].marinejs / seqNullResuts[i-1].bindgen;
                console.log("marinejs/bindegen speed at tree depth " + i + ": " + relation);
            }

            console.log("benchmark finished")
            process.exit(0);
        })
        .on('abort', function(event: any) {
            console.log(String(event.target));
        })
        .on('error', function(event: any) {
            console.log(String(event.target));
        })
        .run({ 'async': true });
}

main().catch((err) => {
    console.log(err);
    process.exit(1);
});

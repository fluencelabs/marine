/*
export function call_export(module_name, export_name, args) {
    console.log("JS: call_export called with: ", module_name, export_name, args)
    let parsed_args = JSON.parse(args);
    console.log("parsed args: ", args);
    let prepared_args = [];
    for (let i = 0; i < parsed_args.length; i++) {
        let arg = parsed_args[i];
        console.log(arg)
        prepared_args.push(arg["I32"])
    }

    console.log("prepared args: ", prepared_args);
    let result = window.MARINE_WASM_MODULES[module_name].exports[export_name](...prepared_args);
    console.log("got result: ", result)
    let json_string = "[]";
    if (result !== undefined) {
        json_string = "[" + JSON.stringify(result) + "]"
    }

    console.log("got result_string: ", json_string)
    return json_string
}

export function get_memory_size(module_name) {
    //console.log("called get_memory_size with name=", module_name);
    let buf = new Uint8Array(window.MARINE_WASM_MODULES[module_name].exports.memory.buffer);
    console.log("result=", buf.byteLength);
    return buf.byteLength
}

export function write_memory(module_name, offset, bytes) {

}

export function read_memory(module_name, offset, size) {

}

export function write_byte(module_name, offset, value) {
    //console.log("write_byte called with args: module_name={}, offset={}, value={}", module_name, offset, value)
    let buf = new Uint8Array(window.MARINE_WASM_MODULES[module_name].exports.memory.buffer);
    //console.log(buf)
    buf[offset] = value
}

export function read_byte(module_name, offset) {
    //console.log("read_byte called with args: module_name={}, offset={}", module_name, offset)
    let buf = new Uint8Array(window.MARINE_WASM_MODULES[module_name].exports.memory.buffer);
    //console.log(buf)
    //console.log("read_byte returns {}", buf[offset])
    return buf[offset];
}
*/
export function call_export(instance, export_name, args) {
    console.log("JS: call_export called with: ", instance, export_name, args)
    let parsed_args = JSON.parse(args);
    console.log("parsed args: ", args);
    let prepared_args = [];
    for (let i = 0; i < parsed_args.length; i++) {
        let arg = parsed_args[i];
        console.log(arg)
        prepared_args.push(arg["I32"])
    }

    console.log("prepared args: ", prepared_args);
    let result = instance.exports[export_name](...prepared_args);
    console.log("got result: ", result)
    let json_string = "[]";
    if (result !== undefined) {
        json_string = "[" + JSON.stringify(result) + "]"
    }

    console.log("got result_string: ", json_string)
    return json_string
}

export function get_memory_size(instance) {
    //console.log("called get_memory_size with name=", module_name);
    let buf = new Uint8Array(instance.exports.memory.buffer);
    console.log("result=", buf.byteLength);
    return buf.byteLength
}

export function write_memory(instance, offset, bytes) {

}

export function read_memory(instance, offset, size) {

}

export function write_byte(instance, offset, value) {
    //console.log("write_byte called with args: module_name={}, offset={}, value={}", module_name, offset, value)
    let buf = new Uint8Array(instance.exports.memory.buffer);
    //console.log(buf)
    buf[offset] = value
}

export function read_byte(instance, offset) {
    //console.log("read_byte called with args: module_name={}, offset={}", module_name, offset)
    let buf = new Uint8Array(instance.exports.memory.buffer);
    //console.log(buf)
    //console.log("read_byte returns {}", buf[offset])
    return buf[offset];
}
/*
export function call_import(module_name, function_name, args) {
    call_wasm(...)
}





import { Fluence } from "@fluencelabs/fluence";
import { krasnodar } from "@fluencelabs/fluence-network-environment"; // (1)
import {
    registerHelloWorld,
    sayHello,
    getRelayTime,
    tellFortune,
} from "./_aqua/hello-world"; // (2)

async function main() {
    await Fluence.start({ connectTo: krasnodar[0] }); // (3)
    await Fluence.registerWasmAsHelloWorld(wasmUrl)

    // (4)
    registerHelloWorld({
        hello: (str) => {
            console.log(str);
        },
        getFortune: async () => {
            await new Promise((resolve) => {
                setTimeout(resolve, 1000);
            });
            return "Wealth awaits you very soon.";
        },
    });

    await sayHello(); // (4)

    console.log(await tellFortune()); // (6)

    const relayTime = await getRelayTime();

    console.log("The relay time is: ", new Date(relayTime).toLocaleString());

    await Fluence.stop(); // (7)
}

main();


pavel:
  run avm with web-marine in dashboard(or something other big)
  think about problems arised
  design interface for loading services in fluence-js

valery:
  add interaces needed by pavel:
    - provide names of exported functions
    - add interface for calling wasm IT functions : func call(module_name, func_name, argsInJson)
    -
*/
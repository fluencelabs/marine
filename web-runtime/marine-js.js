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
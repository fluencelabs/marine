export function call_export(instance, export_name, args) {
    let parsed_args = JSON.parse(args);
    let prepared_args = [];
    for (let arg_index = 0; arg_index < parsed_args.length; arg_index++) {
        let arg = parsed_args[arg_index];
        prepared_args.push(arg["I32"])
    }

    let result = instance.exports[export_name](...prepared_args);

    let json_result = "[]";
    if (result !== undefined) {
        json_result = "[" + JSON.stringify(result) + "]"
    }

    return json_result
}

export function get_memory_size(instance) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    return buf.byteLength
}

export function read_byte(instance, offset) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    return buf[offset];
}

export function write_byte_range(instance, offset, slice) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    for (let byte_index = 0; byte_index < slice.length; byte_index++) {
        buf[offset + byte_index] = slice[byte_index]
    }
}

export function read_byte_range(instance, offset, slice) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    for (let byte_index = 0; byte_index < slice.length; byte_index++) {
        slice[byte_index] = buf[offset + byte_index];
    }
}

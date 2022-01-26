
module.exports.call_export = function (instance, export_name, args) {
    //console.log("JS: call_export called with: ", instance, export_name, args)
    let parsed_args = JSON.parse(args);
    //console.log("parsed args: ", args);
    let prepared_args = [];
    for (let i = 0; i < parsed_args.length; i++) {
        let arg = parsed_args[i];
       // console.log(arg)
        prepared_args.push(arg["I32"])
    }

    //console.log("prepared args: ", prepared_args);
    let result = instance.exports[export_name](...prepared_args);
    //console.log("got result: ", result)
    let json_string = "[]";
    if (result !== undefined) {
        json_string = "[" + JSON.stringify(result) + "]"
    }

    //console.log("got result_string: ", json_string)
    return json_string
}

module.exports.get_memory_size = function (instance) {
    //console.log("called get_memory_size with name=", module_name);
    let buf = new Uint8Array(instance.exports.memory.buffer);
    //console.log("result=", buf.byteLength);
    return buf.byteLength
}

module.exports.write_byte =  function (instance, offset, value) {
    //console.log("write_byte called with args: module_name={}, offset={}, value={}", module_name, offset, value)
    let buf = new Uint8Array(instance.exports.memory.buffer);
    //console.log(buf)
    buf[offset] = value
}

module.exports.read_byte =  function (instance, offset) {
    //console.log("read_byte called with args: module_name={}, offset={}", module_name, offset)
    let buf = new Uint8Array(instance.exports.memory.buffer);
    //console.log(buf)
    //console.log("read_byte returns {}", buf[offset])
    return buf[offset];
}

module.exports.write_byte_range =  function (instance, offset, slice) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    for (i = 0; i < slice.length; i++) {
        buf[offset + i] = slice[i]
    }
}

module.exports.read_byte_range =  function (instance, offset, slice) {
    let buf = new Uint8Array(instance.exports.memory.buffer);
    for (i = 0; i < slice.length; i++) {
         slice[i] = buf[offset + i];
    }
}

/*
export function call_export(instance, export_name, args) {
    //console.log("JS: call_export called with: ", instance, export_name, args)
    let parsed_args = JSON.parse(args);
    //console.log("parsed args: ", args);
    let prepared_args = [];
    for (let i = 0; i < parsed_args.length; i++) {
        let arg = parsed_args[i];
        // console.log(arg)
        prepared_args.push(arg["I32"])
    }

    //console.log("prepared args: ", prepared_args);
    let result = instance.exports[export_name](...prepared_args);
    //console.log("got result: ", result)
    let json_string = "[]";
    if (result !== undefined) {
        json_string = "[" + JSON.stringify(result) + "]"
    }

    //console.log("got result_string: ", json_string)
    return json_string
}

export function get_memory_size(instance) {
    //console.log("called get_memory_size with name=", module_name);
    let buf = new Uint8Array(instance.exports.memory.buffer);
    //console.log("result=", buf.byteLength);
    return buf.byteLength
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
*/
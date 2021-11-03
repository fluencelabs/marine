export function call_export(module_name, export_name) {
    window.MARINE_WASM_MODULES[module_name].exports[export_name]()
}

export function write_memory(module_name, offset, bytes) {

}

export function read_memory(module_name, offset, size) {

}
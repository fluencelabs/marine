import { WASI } from '@wasmer/wasi'
import browserBindings from '@wasmer/wasi/lib/bindings/browser'
import { WasmFs } from '@wasmer/wasmfs'
import init, { register_module, call_module, test_call_avm, test_call_greeting_array} from "./marine_web_runtime.js";


const greetingPath = '/greeting.wasm'  // Path to our WASI module
const avmPath = '/air_interpreter_server.wasm'  // Path to our WASI module
const echoStr      = 'Hello World!'    // Text string to echo

window.MARINE_WASM_MODULES = {}
// Instantiate new WASI and WasmFs Instances
// IMPORTANT:
// Instantiating WasmFs is only needed when running in a browser.
// When running on the server, NodeJS's native FS module is assigned by default
const wasmFs = new WasmFs()

let wasi = new WASI({
    // Arguments passed to the Wasm Module
    // The first argument is usually the filepath to the executable WASI module
    // we want to run.
    args: [],

    // Environment variables that are accesible to the WASI module
    env: {},

    // Bindings that are used by the WASI Instance (fs, path, etc...)
    bindings: {
        ...browserBindings,
        fs: wasmFs.fs
    }
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async function to run our WASI module/instance

async function loadMarineModule(modulePath, moduleName) {
    console.log("loading", moduleName);
    let response  = await fetch(modulePath)
    let wasmBytes = new Uint8Array(await response.arrayBuffer())

    let wasmModule = await WebAssembly.compile(wasmBytes);
    console.log(wasmModule)

    let instance = await WebAssembly.instantiate(wasmModule, {
        ...wasi.getImports(wasmModule),
    });
    console.log(instance)

    window.MARINE_WASM_MODULES[moduleName] = instance;

    wasi.start(instance)                       // Start the WASI instance
    let custom_sections = WebAssembly.Module.customSections(wasmModule,"interface-types");
    console.log(custom_sections)
    let custom_section2 = new Uint8Array(custom_sections[0])
    return {
        name: moduleName,
        custom_section: custom_section2,
    }
    //return [instance, custom_section2, moduleName]
}
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here

async function main() {
    let greeting = await loadMarineModule(greetingPath, "greeting");
    let avm = await loadMarineModule(avmPath, "avm");
    await init()

    //register_module(avm.name, avm.custom_section)
    register_module(greeting.name, greeting.custom_section)
    //test_call_avm()
    let result = call_module(greeting.name, "greeting", JSON.stringify(["js"]));
    console.log("JS: got result from call_module:", result)
}

main()

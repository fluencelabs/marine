use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/wasi_bindings.js")]
extern "C" {
    pub fn create_wasi() -> JsValue;
    pub fn generate_wasi_imports(module: &JsValue, wasi: &JsValue) -> JsValue;
    pub fn bind_to_instance(wasi: &JsValue, memory: &JsValue);
}

mod global_state;
mod api;

use global_state::MARINE;

use marine_js_backend::JsWasmBackend;
use marine::generic::Marine;

use wasm_bindgen::prelude::*;

use std::cell::RefCell;

#[wasm_bindgen(start)]
fn main() {
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
}

#[cfg(test)]
mod tests {
    use marine_js_backend::JsWasmBackend;

    #[test]
    fn test_test() {
        let core = marine_core::generic::MarineCore::<JsWasmBackend>::new();
    }
}

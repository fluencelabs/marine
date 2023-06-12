mod api;
mod global_state;
mod logger;

use wasm_bindgen::prelude::*;

use crate::logger::MarineLogger;

#[wasm_bindgen(start)]
fn main() {
    log::set_boxed_logger(Box::new(MarineLogger::new(log::LevelFilter::Info))).unwrap();
    // Trace is required to accept all logs from a service.
    // Max level for this crate is set in MarineLogger constructor.
    log::set_max_level(log::LevelFilter::Trace);
}

#[cfg(test)]
mod tests {
    use marine_js_backend::JsWasmBackend;

    #[test]
    fn test_test() {
        let core = marine_core::generic::MarineCore::<JsWasmBackend>::new();
    }
}

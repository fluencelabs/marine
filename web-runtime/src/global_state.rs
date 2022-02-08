use crate::faas::FluenceFaaS;
use wasm_bindgen::prelude::JsValue;


use std::cell::RefCell;

// two variables required because public api functions borrow_mut MODULES,
// and deep internal functions borrow_mut INSTANCE
// this is a bad design, and it will be refactored while moving wasm compilation inside marine-web
thread_local!(pub(crate) static MODULES: RefCell<Option<FluenceFaaS>> = RefCell::new(None));
thread_local!(pub(crate) static INSTANCE: RefCell<Option<JsValue>> = RefCell::new(None));

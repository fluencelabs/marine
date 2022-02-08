use crate::faas::FluenceFaaS;
use wasm_bindgen::prelude::JsValue;


use std::cell::RefCell;

thread_local!(pub(crate) static MODULES: RefCell<Option<FluenceFaaS>> = RefCell::new(None));
thread_local!(pub(crate) static INSTANCE: RefCell<Option<JsValue>> = RefCell::new(None));

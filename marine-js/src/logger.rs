use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::fs::metadata;
use std::hash::Hash;
use log::{LevelFilter, Log, Metadata, Record};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

struct MarineLoggerInner {
    service_log_fn: Option<js_sys::Function>,
    module_names: Option<HashSet<String>>,
    self_max_level: LevelFilter,
}

pub(crate) struct MarineLogger {
    inner: RefCell<MarineLoggerInner>,
}

#[derive(Serialize, Deserialize)]
struct ModuleLogMessage {
    level: String,
    message: String,
    service: String,
}
// Safety: marine-js is supposed to be in a single-threaded wasm environment
unsafe impl Send for MarineLogger {}
unsafe impl Sync for MarineLogger {}
unsafe impl Send for MarineLoggerInner {}
unsafe impl Sync for MarineLoggerInner {}

impl MarineLogger {
    pub(crate) fn new(self_max_level: LevelFilter) -> Self {
        Self {
            inner: RefCell::new(MarineLoggerInner::new(self_max_level)),
        }
    }

    pub(crate) fn enable_service_logging(&self, log_fn: JsValue, module_names: HashSet<String>) {
        self.inner
            .borrow_mut()
            .enable_service_logging(log_fn, module_names);
    }
}

impl MarineLoggerInner {
    fn new(self_max_level: LevelFilter) -> Self {
        Self {
            service_log_fn: <_>::default(),
            module_names: <_>::default(),
            self_max_level,
        }
    }

    fn enable_service_logging(&mut self, log_fn: JsValue, module_names: HashSet<String>) {
        self.service_log_fn = Some(log_fn.into());
        self.module_names = Some(module_names);
    }

    fn is_service_log(&self, record: &Metadata) -> bool {
        match self.module_names.as_ref() {
            None => false,
            Some(modules) => modules.contains(record.target()), // TODO: is it the only needed check?
        }
    }

    fn log_module_message(&self, record: &Record) {
        let message = ModuleLogMessage {
            level: record.level().to_string().to_ascii_lowercase(),
            message: record.args().to_string(),
            service: record.target().to_string(),
        };

        // TODO: safety
        let message = serde_wasm_bindgen::to_value(&message).unwrap();
        let params = js_sys::Array::from_iter([message].iter());

        // TODO get rid of unwrap
        js_sys::Reflect::apply(
            self.service_log_fn.as_ref().unwrap(),
            &JsValue::NULL,
            &params,
        )
        .unwrap();
    }
}

impl log::Log for MarineLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.borrow().enabled(metadata)
    }

    fn log(&self, record: &Record) {
        self.inner.borrow().log(record)
    }

    fn flush(&self) {
        self.inner.borrow().flush()
    }
}

impl log::Log for MarineLoggerInner {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.is_service_log(metadata) || metadata.level() <= self.self_max_level
    }

    fn log(&self, record: &Record) {
        if self.is_service_log(record.metadata()) {
            self.log_module_message(record)
        } else if record.level() <= self.self_max_level {
            wasm_bindgen_console_logger::DEFAULT_LOGGER.log(record)
        }
    }

    fn flush(&self) {
        wasm_bindgen_console_logger::DEFAULT_LOGGER.flush()
    }
}

pub(crate) fn marine_logger() -> &'static MarineLogger {
    // Safety: MarineLogger is set as logger in the main function, so this is correct
    unsafe { &*(log::logger() as *const dyn Log as *const MarineLogger) }
}

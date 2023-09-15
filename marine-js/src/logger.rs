/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use log::LevelFilter;
use log::Log;
use log::Metadata;
use log::Record;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::JsValue;

use std::cell::RefCell;
use std::collections::HashSet;

struct ServiceLogger {
    log_fn: js_sys::Function,
    module_names: HashSet<String>,
}

struct MarineLoggerInner {
    service_logger: Option<ServiceLogger>,
    /// Log level for the marine-js itself. Log level for wasm modules is set via env vars in config.
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
// Safety: marine-js is supposed to be in a single-threaded wasm environment.
unsafe impl Send for MarineLogger {}
unsafe impl Sync for MarineLogger {}
unsafe impl Send for MarineLoggerInner {}
unsafe impl Sync for MarineLoggerInner {}
unsafe impl Send for ServiceLogger {}
unsafe impl Sync for ServiceLogger {}

impl MarineLogger {
    pub(crate) fn new(self_max_level: LevelFilter) -> Self {
        Self {
            inner: RefCell::new(MarineLoggerInner::new(self_max_level)),
        }
    }

    pub(crate) fn enable_service_logging(
        &self,
        log_fn: js_sys::Function,
        module_names: HashSet<String>,
    ) {
        self.inner
            .borrow_mut()
            .enable_service_logging(log_fn, module_names);
    }
}

impl MarineLoggerInner {
    fn new(self_max_level: LevelFilter) -> Self {
        Self {
            service_logger: None,
            self_max_level,
        }
    }

    fn enable_service_logging(&mut self, log_fn: js_sys::Function, module_names: HashSet<String>) {
        self.service_logger = Some(ServiceLogger::new(log_fn, module_names));
    }

    fn is_service_log(&self, metadata: &Metadata) -> bool {
        match &self.service_logger {
            None => false,
            Some(service_logger) => service_logger.should_handle(metadata),
        }
    }

    fn log_service_message(&self, record: &Record) {
        let result = self
            .service_logger
            .as_ref()
            .map(|logger| logger.log(record));

        if let Some(Err(e)) = result {
            web_sys::console::error_2(&"failed to log service message:".into(), &e);
        }
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
            self.log_service_message(record)
        } else if record.level() <= self.self_max_level {
            wasm_bindgen_console_logger::DEFAULT_LOGGER.log(record)
        }
    }

    fn flush(&self) {
        wasm_bindgen_console_logger::DEFAULT_LOGGER.flush()
    }
}

impl ServiceLogger {
    fn new(log_fn: js_sys::Function, module_names: HashSet<String>) -> Self {
        Self {
            log_fn,
            module_names,
        }
    }

    fn should_handle(&self, metadata: &Metadata) -> bool {
        self.module_names.contains(metadata.target())
    }

    fn log(&self, record: &Record) -> Result<(), JsValue> {
        let message = ModuleLogMessage {
            level: record.level().to_string().to_ascii_lowercase(),
            message: record.args().to_string(),
            service: record.target().to_string(),
        };

        let message = serde_wasm_bindgen::to_value(&message)?;
        let params = js_sys::Array::from_iter([message].iter());

        js_sys::Reflect::apply(&self.log_fn, &JsValue::NULL, &params)?;

        Ok(())
    }
}

pub(crate) fn marine_logger() -> &'static MarineLogger {
    // Safety: MarineLogger is set as logger in the main function, so this is correct.
    unsafe { &*(log::logger() as *const dyn Log as *const MarineLogger) }
}

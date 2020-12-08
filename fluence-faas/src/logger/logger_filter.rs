/*
 * Copyright 2020 Fluence Labs Limited
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
use std::collections::HashMap;

/// A logger filter.
///
/// This struct can be used to determine whether or not
/// a log record should be written to the output.
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub(crate) struct LoggerFilter<'env_string> {
    default_log_level: Option<LevelFilter>,
    module_levels: HashMap<&'env_string str, LevelFilter>,
}

impl<'env_string> LoggerFilter<'env_string> {
    /// Parses a content of supplied variable in form of "module_name_1=log_level,module_name_2".
    pub(crate) fn from_env_string(env: &'env_string str) -> Self {
        let mut module_levels = HashMap::new();
        let mut default_log_level: Option<LevelFilter> = None;

        for module_log in env.split(',') {
            if module_log.is_empty() {
                continue;
            }

            let mut module_log_parts = module_log.split('=');
            let part_0 = module_log_parts.next();
            let part_1 = module_log_parts.next().map(|s| s.trim());
            if let Some(part_3) = module_log_parts.next() {
                eprintln!(
                    "logger warning: invalid directive '{}', ignoring it",
                    part_3
                );
                continue;
            }
            let (module_name, module_log_level) = match (part_0, part_1) {
                // "info"
                // "1"
                (Some(part), None) => match part.parse() {
                    Ok(num) => (None, num),
                    Err(_) => (Some(part), LevelFilter::max()),
                },
                // "module_name="
                (Some(module_name), Some("")) => (Some(module_name), LevelFilter::max()),
                // "module_name=info"
                (Some(module_name), Some(log_level)) => match log_level.parse() {
                    Ok(log_level) => (Some(module_name), log_level),
                    Err(e) => {
                        eprintln!(
                            "logger warning: invalid directive '{}', error '{}', ignoring it",
                            log_level, e
                        );
                        continue;
                    }
                },
                d => {
                    eprintln!("logger warning: invalid directive '{:?}', ignoring it", d);
                    continue;
                }
            };

            match (module_name, &mut default_log_level) {
                (Some(module_name), _) => {
                    module_levels.insert(module_name, module_log_level);
                }
                (None, Some(_)) => {
                    eprintln!(
                        "logger warning: can't set default level twice, '{}' ignored",
                        module_log_level
                    );
                }
                (None, w) => *w = Some(module_log_level),
            }
        }

        Self {
            default_log_level,
            module_levels,
        }
    }

    pub(crate) fn module_level(&self, module_name: &str) -> Option<LevelFilter> {
        self.module_levels
            .get(module_name)
            .map_or_else(|| self.default_log_level, |l| Some(*l))
    }
}

/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

#[cfg(test)]
mod tests {
    use super::LoggerFilter;
    use log::LevelFilter;

    fn test_one_level_filter(unparsed_level: &str, expected_level: LevelFilter) {
        let logger_filter = LoggerFilter::from_env_string(unparsed_level);

        let actual_level = logger_filter
            .module_level("some_module_name")
            .expect("global option should work");
        assert_eq!(actual_level, expected_level);
    }

    #[test]
    fn one_default_filter() {
        use LevelFilter::*;

        test_one_level_filter("off", Off);
        test_one_level_filter("error", Error);
        test_one_level_filter("warn", Warn);
        test_one_level_filter("info", Info);
        test_one_level_filter("debug", Debug);
        test_one_level_filter("trace", Trace);
    }

    #[test]
    fn module_levels() {
        use LevelFilter::*;
        let logger_filter = LoggerFilter::from_env_string(
            "module_1=off,module_2=error,module_3=warn,module_4=info,module_5=debug,module_6=trace",
        );

        let actual_level = logger_filter
            .module_level("module_1")
            .expect("module option should work");
        assert_eq!(actual_level, Off);

        let actual_level = logger_filter
            .module_level("module_2")
            .expect("module option should work");
        assert_eq!(actual_level, Error);

        let actual_level = logger_filter
            .module_level("module_3")
            .expect("module option should work");
        assert_eq!(actual_level, Warn);

        let actual_level = logger_filter
            .module_level("module_4")
            .expect("module option should work");
        assert_eq!(actual_level, Info);

        let actual_level = logger_filter
            .module_level("module_5")
            .expect("module option should work");
        assert_eq!(actual_level, Debug);

        let actual_level = logger_filter
            .module_level("module_6")
            .expect("module option should work");
        assert_eq!(actual_level, Trace);
    }

    #[test]
    fn mixed_default_and_module_levels() {
        use LevelFilter::*;
        let logger_filter = LoggerFilter::from_env_string("module_1=off,module_2=error,module_3=warn,module_4=info,module_5=debug,module_6=trace,off");

        let actual_level = logger_filter
            .module_level("module_1")
            .expect("module option should work");
        assert_eq!(actual_level, Off);

        let actual_level = logger_filter
            .module_level("module_2")
            .expect("module option should work");
        assert_eq!(actual_level, Error);

        let actual_level = logger_filter
            .module_level("module_3")
            .expect("module option should work");
        assert_eq!(actual_level, Warn);

        let actual_level = logger_filter
            .module_level("module_4")
            .expect("module option should work");
        assert_eq!(actual_level, Info);

        let actual_level = logger_filter
            .module_level("module_5")
            .expect("module option should work");
        assert_eq!(actual_level, Debug);

        let actual_level = logger_filter
            .module_level("module_6")
            .expect("module option should work");
        assert_eq!(actual_level, Trace);

        let actual_level = logger_filter
            .module_level("some_module_name")
            .expect("global option should work");
        assert_eq!(actual_level, Off);
    }
}

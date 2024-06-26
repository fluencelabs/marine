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

import { WASIArgs, WASIEnv } from '@wasmer/wasi';

export interface MarineServiceConfig {
    /**
     * Settings for a module with particular name (not HashMap because the order is matter).
     */
    modules_config: Array<ModuleDescriptor>;

    /**
     * Settings for a module that name's not been found in modules_config.
     */
    default_modules_config?: MarineModuleConfig;
}

export interface ModuleDescriptor {
    import_name: string;
    config: MarineModuleConfig;
}

export interface MarineModuleConfig {
    /**
     * Defines whether FaaS should provide a special host log_utf8_string function for this module.
     */
    logger_enabled: boolean;

    /**
     * Export from host functions that will be accessible on the Wasm side by provided name.
     */
    // host_imports: Map<string, HostImportDescriptor>;

    /**
     * A WASI config.
     */
    wasi: MarineWASIConfig;

    /**
     * Mask used to filter logs, for details see `log_utf8_string`
     */
    logging_mask: number;
}

export type Env = WASIEnv;

export type Args = WASIArgs;

export interface MarineWASIConfig {
    /**
     * A list of environment variables available for this module.
     */
    envs: Env;

    /**
     * A list of files available for this module.
     * A loaded module could have access only to files from this list.
     */
    preopened_files: Set<string>;

    /**
     * Mapping from a usually short to full file name.
     */
    mapped_dirs: Map<String, string>;
}

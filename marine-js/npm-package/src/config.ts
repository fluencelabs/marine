/*
 * Copyright 2022 Fluence Labs Limited
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

import { WASIArgs, WASIEnv } from '@wasmer/wasi';

export interface MarineServiceConfig {
    /**
     * Path to a dir where compiled Wasm modules are located.
     */
    modules_dir: string;

    /**
     * Settings for a module with particular name (not HashMap because the order is matter).
     */
    modules_config: Array<ModuleDescriptor>;

    /**
     * Settings for a module that name's not been found in modules_config.
     */
    default_modules_config: MarineModuleConfig;
}

export interface ModuleDescriptor {
    file_name: String;
    import_name: String;
    config: MarineModuleConfig;
}

export interface MarineModuleConfig {
    /**
     * Maximum memory size accessible by a module in Wasm pages (64 Kb).
     */
    mem_pages_count: number;

    /**
     * Maximum memory size for heap of Wasm module in bytes, if it set, mem_pages_count ignored.
     */
    max_heap_size: number;

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

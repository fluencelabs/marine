export interface FaaSConfig {
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
    default_modules_config: FaaSModuleConfig;
}

export interface ModuleDescriptor {
    file_name: String;
    import_name: String;
    config: FaaSModuleConfig;
}

export interface FaaSModuleConfig {
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
    wasi: FaaSWASIConfig;

    /**
     * Mask used to filter logs, for details see `log_utf8_string`
     */
    logging_mask: number;
}

export interface MarineConfig {
    service_base_dir: string;
    faas_config: FaaSConfig;
}

export interface FaaSWASIConfig {
    /**
     * A list of environment variables available for this module.
     */
    envs: Map<Uint8Array, Uint8Array>;

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

export interface InitConfig {
    serviceId: string;
    marine: SharedArrayBuffer;
    service: SharedArrayBuffer;
    config?: MarineConfig;
    envs?: Map<Uint8Array, Uint8Array>;
}

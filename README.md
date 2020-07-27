# Fluence Compute Engine

FCE is intended to run various Wasm binaries. At now, it is in the heavily developing phase.

## Installation
- Clone this project
- `cargo install --path <path-to-project/tools/cli>`

## Usage
- `fce build` in Rust project

## HOW TO: Create App with FCE Modules

### Recommendations:

- Modules architecture should be upwards from `effectors` (modules that persist data and WASI modules) that will work with local binaries, local storage and syscalls to `pure modules` that perform business logic.
- Splitting app to small FCE modules are easier to support, reuse and distribute
- Each module for its own task (npm like)

### Module project structure

- Init simple rust project `cargo init --bin`

- `Config.toml`:
```toml
[[bin]]
name = "wasm_application"
path = "src/main.rs"

[dependencies]
# logger - if you'll use logging
fluence = { git = "https://github.com/fluencelabs/rust-sdk", features = ["logger"] }
```

- Methods that will be exported from this module marked with `#[fce]`
```rust
use fluence::fce;

#[fce]
pub fn get(url: String) -> String {
...
}
```
- Multiple arguments with primitive Rust types (`bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String, Vec<u8>`) and only one return argument could be used

- Build project with `fce build` (supports --release and all other cargo flags now)

- Copy wasm file from `target/wasm32-wasi/debug` to directory with other modules

- To import other wasm modules to your project use similar code:
```rust
#[fce]
#[link(wasm_import_module = "wasm_curl.wasm")]
extern "C" {
    #[link_name = "get"]
    pub fn curl_get(url: String) -> String;
}

#[fce]
#[link(wasm_import_module = "wasm_local_storage.wasm")]
extern "C" {
    #[link_name = "get"]
    pub fn curl_get(url: String) -> String;
}
``` 

### Combine modules to Application

- Create simple Rust project
- Create `Config.toml` to describe existed wasm modules and give accesses to host binaries and local storage if needed:
```toml
core_modules_dir = "wasm/artifacts/modules"

[[core_module]]
    name = "wasm_local_storage_with_curl.wasm"
    mem_pages_count = 100    

   [core_module.imports]
    curl = "/usr/bin/curl"

    [core_module.wasi]
    envs = []
    preopened_files = ["./wasm/artifacts"]
    mapped_dirs = { "tmp" = "./wasm/artifacts" }
```

`core_modules_dir` - path to directory with all modules. All subsequent paths will be relative to this path

`[[core_module]]` - modules list

`name` - wasm file name in `core_modules_dir`

`mem_pages_count` - a maximum number of Wasm memory pages that loaded module can use. Each Wasm pages is 65536 bytes long

`[core_module.imports]` - list of available imports

`curl = "/usr/bin/curl"` - gives possibility to call binary file `/usr/bin/curl` as method `curl` in Rust code

Import example:
```rust
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(args: String) -> String;
}
```

Call binary with arguments: `curl("-vvv ya.ru")`

`[core_module.wasi]` - this block manages communication with the "outside" world
`env` - environment variables. Usage: `std::env::var("IPFS_ADDR")`
`preopened_files` - list of available directories for loaded modules
`mapped_dirs` - mapping between paths

Working with files as usual:
```rust
fs::write(PathBuf::from("/tmp/somefile"), vec!(1,2,3));
fs::read(...);
```
# Marine

![marine version on crates.io](https://img.shields.io/crates/v/marine?color=green&style=flat-square)

Marine is a modern general purpose Wasm runtime based on the [Component model](https://github.com/WebAssembly/component-model), it runs multi-module Wasm applications with interface-types and shared-nothing linking scheme. This execution model suits well for different scenarios, especially for applications based on the [entity component system](https://en.wikipedia.org/wiki/Entity_component_system) (ECS) pattern or architecture based on plugins.

Fluence [nodes](https://github.com/fluencelabs/fluence) use Marine to execute Wasm services composed by compiled [Aqua](https://github.com/fluencelabs/aqua):\

## Motivational example

The most archetypal example to meet the power of Marine is a multi-module Wasm application. Let's consider [this](./examples/motivational-example) one consists of two modules: 
```rust
use marine_rs_sdk::marine;

fn main() {}

#[marine]
pub fn greeting(name: &str) -> Vec<String> {
    let donkey_greeting = donkey::greeting(name);
    let shrek_greeting = format!("Shrek: hi, {}", name);
    
    vec![shrek_greeting, donkey_greeting]
}

mod donkey {
    use super::*;

    #[marine]
    #[link(wasm_import_module = "donkey")]
    extern "C" {
        pub fn greeting(name: &str) -> String;
    }
}
```

```rust
use marine_rs_sdk::marine;

fn main() {}

#[marine]
pub fn greeting(name: &str) -> String {
    format!("Donkey: hi, {}", name)
}
```


Compile these modules (you'll need Marine tools to be installed, here is the installation [guide](https://doc.fluence.dev/docs/tutorials_tutorials/recipes_setting_up#marine-tools)) and run them with Marine REPL:
```bash
$ > ./build.sh
  ...
$ > mrepl Config.toml
  ...
1> interfaces
Loaded modules interface:

shrek:
  fn greeting(name: string) -> []string
donkey:
  fn greeting(name: string) -> string

2> call shrek greeting "Feona"
result: Array([String("Shrek: hi, Feona"), String("Donkey: hi, Feona")])
 elapsed time: 903.949µs

3> q
```

## Repository structure

- **[REPL](./tools/repl)** - REPL intended to test Marine Wasm modules
- **[CLI](./tools/cli)** - CLI intended to build and extract some info from Marine Wasm modules
- **[fluence-app-service](./fluence-app-service)** - Fluence Application Service layer provides basic API for service running
- **[fluence-faas](./fluence-faas)** - Fluence FaaS layer provides host closures, IT<->JSON conversion, logger, config handling and other
- **[runtime](./runtime)** - runtime layer provides basic functionality for loading, unloading and calling modules
- **[web-runtime](./web-runtime)** - web runtime layer aim to run Marine in browser
- **[examples](./examples)** - several Marine examples used mostly for tests
- **[it-generator](./crates/it-generator)** - generator of IT
- **[it-interfaces](./crates/it-interfaces)** - handy structure for interface types handling
- **[it-json-serde](./crates/it-json-serde)** - crate for conversion between IT and JSON
- **[min-it-version](./crates/min-it-version)** - keeps minimal supported versions of IT and SDK by runtime
- **[module-info-parser](./crates/module-info-parser)** - parser of module manifest and sdk version
- **[module-interface](./crates/module-interface)** - parser of module IT
- **[utils](./crates/utils)** - some utility functions and consts

## References

- [Documentation](https://doc.fluence.dev/docs/knowledge_aquamarine/marine)
- [Examples](https://github.com/fluencelabs/examples/tree/main/marine-examples)
- [Quickstart](https://doc.fluence.dev/docs/quick-start/2.-hosted-services)

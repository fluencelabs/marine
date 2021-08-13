# Marine

Marine is a general purpose Wasm runtime that could be used in different scenarios, especially in programs based on the [ECS](https://en.wikipedia.org/wiki/Entity_component_system) pattern or plugin architecture. It runs multi-module WebAssembly applications with interface-types and shared-nothing linking scheme.

Fluence [nodes](https://github.com/fluencelabs/fluence) uses Marine to execute [AIR](https://github.com/fluencelabs/aquamarine) scripts and services compiled to Wasm:

<p align="center" width="100%">
    <img alt="fluence stack" align="center" src="images/fluence_stack_overview.png" width="663"/>
</p>

## Documentation

To learn more about tooling around marine, see [Marine](https://doc.fluence.dev/docs/knowledge_aquamarine/marine) doc. For a more high-level usage, take a look at how to [create Fluence services with WASM & Marine](https://doc.fluence.dev/docs/quick-start/2.-hosted-services).


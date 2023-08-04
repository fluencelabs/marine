# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.20.0 to 0.20.1

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.20.1 to 0.20.2
    * marine-wasmtime-backend bumped from 0.2.0 to 0.2.1

## [0.28.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.27.0...marine-runtime-v0.28.0) (2023-08-04)


### ⚠ BREAKING CHANGES

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357))

### Bug Fixes

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357)) ([71e138d](https://github.com/fluencelabs/marine/commit/71e138dce31c2896bcd7b0657c3122c4b7f6402b))

## [0.27.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.26.3...marine-runtime-v0.27.0) (2023-07-25)


### ⚠ BREAKING CHANGES

* **marine-js:** replace old marine-js with common marine-runtime + backend traits impl for JS ([#332](https://github.com/fluencelabs/marine/issues/332))
* **wasm-backend:** split Function trait, improve naming ([#347](https://github.com/fluencelabs/marine/issues/347))
* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333))

### Features

* **marine-js:** replace old marine-js with common marine-runtime + backend traits impl for JS ([#332](https://github.com/fluencelabs/marine/issues/332)) ([a61ddfc](https://github.com/fluencelabs/marine/commit/a61ddfc4044b53a9d5f7864c933a48f7404c473c))
* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333)) ([aeae703](https://github.com/fluencelabs/marine/commit/aeae703229f5f9410429390fe2e72d6084527f14))
* **wasm-backend:** split Function trait, improve naming ([#347](https://github.com/fluencelabs/marine/issues/347)) ([0f9979a](https://github.com/fluencelabs/marine/commit/0f9979ae11267af119eccc3063c4001ffece4aee))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.20.3 to 0.21.0
    * marine-wasm-backend-traits bumped from 0.2.1 to 0.3.0
    * marine-wasmtime-backend bumped from 0.2.2 to 0.3.0

## [0.26.3](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.26.2...marine-runtime-v0.26.3) (2023-04-04)


### Bug Fixes

* **release-please:** Get rid of workspace.dependencies ([#316](https://github.com/fluencelabs/marine/issues/316)) ([71835e6](https://github.com/fluencelabs/marine/commit/71835e6762515a83cde1cc944d60352a4c1221f5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.20.2 to 0.20.3
    * marine-module-interface bumped from 0.7.0 to 0.7.1
    * it-json-serde bumped from 0.4.0 to 0.4.1
    * marine-wasm-backend-traits bumped from 0.2.0 to 0.2.1
    * marine-wasmtime-backend bumped from 0.2.1 to 0.2.2

## [0.26.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.25.0...marine-runtime-v0.26.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.19.0 to 0.20.0
    * marine-module-interface bumped from 0.6.1 to 0.7.0
    * marine-utils bumped from 0.4.0 to 0.5.0
    * it-json-serde bumped from 0.3.5 to 0.4.0
    * marine-wasm-backend-traits bumped from 0.1.0 to 0.2.0
    * marine-wasmtime-backend bumped from 0.1.0 to 0.2.0

## [0.25.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.24.1...marine-runtime-v0.25.0) (2023-03-03)


### ⚠ BREAKING CHANGES

* **fluence-app-service:** add separate bases for temp dirs and mapped dirs ([#288](https://github.com/fluencelabs/marine/issues/288))

### Features

* **fluence-app-service:** add separate bases for temp dirs and mapped dirs ([#288](https://github.com/fluencelabs/marine/issues/288)) ([1d86899](https://github.com/fluencelabs/marine/commit/1d868992bd944eb83926c18a17a24d135c692b4c))

## [0.24.1](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.24.0...marine-runtime-v0.24.1) (2023-02-20)


### Bug Fixes

* **mrepl:** replace fn with func in mrepl output to match aqua syntax ([#284](https://github.com/fluencelabs/marine/issues/284)) ([e4c77a8](https://github.com/fluencelabs/marine/commit/e4c77a8cc4c9963ae74e63504dedbcd227bd7cbf))

## [Unreleased]

## [0.24.0] - 2022-12-06

### Added
- [**breaking**] prohibit going out of service_dir in app-service (#244)
- *(fluence-app-service)* make base path field optional in ConfigContext interface (#202)

### Fixed
- *(runtime)* detect mapped/preopened dirs conflicts before wasmer-wasi crashes (#223)
- [**breaking**] bump minor versions where it was required in #189 (#212)
- fix tests after renaming (#174)

### Other
- *(deps)* update all non-major rust dependencies (#211)
- *(build)* fix clippy warnings (#213)
- Update Rust crate semver to v1 (#198)
- Update all non-major Rust dependencies (#204)
- Update Rust crate serde_with to v2 (#203)
- Update Rust crate cmd_lib to v1 (#194)
- Update Rust crate pretty_assertions to v1 (#196)
- Update all non-major Rust dependencies (#189)
- Rework module searching on filesystem (#184)
- bump crate versions that used marine-rs-sdk-main 0.6.15 (#185)
- Support marine-rs-sdk 0.7.0  (#180)
- Add tests for wasm memory leaks when passing/returning records (#182)
- Add record destruction test (#181)
- Migrate  marine tests to github-actions (#178)
- Fix value after table problem in TomlMarineNamedModuleConfig(#175)
- improve "interface" command output readability (#169)
- Rename `FaaS` to `Marine`, `Runtime` to `Core` (#172)

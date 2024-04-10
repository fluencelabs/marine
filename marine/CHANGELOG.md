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

## [0.37.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.36.2...marine-runtime-v0.37.0) (2024-04-10)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#430](https://github.com/fluencelabs/marine/issues/430))
* move to async execution ([#396](https://github.com/fluencelabs/marine/issues/396))

### Features

* move to async execution ([#396](https://github.com/fluencelabs/marine/issues/396)) ([13cf85b](https://github.com/fluencelabs/marine/commit/13cf85ba369f000c01d040897b366e1087560053))


### Bug Fixes

* **versions:** enforce minor version bumps ([#430](https://github.com/fluencelabs/marine/issues/430)) ([be8293b](https://github.com/fluencelabs/marine/commit/be8293bc06b0e1d28ed19403f6f3af5266aa4de5))

## [0.36.2](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.36.1...marine-runtime-v0.36.2) (2024-03-27)


### Features

* **deps:** reexport types from toml crate, so users don't have to depend on it ([#427](https://github.com/fluencelabs/marine/issues/427)) ([ee39ce0](https://github.com/fluencelabs/marine/commit/ee39ce07bcab85f92d1978e00e244e19577a6b01))

## [0.36.1](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.36.0...marine-runtime-v0.36.1) (2024-02-29)


### Bug Fixes

* **host imports:** add mounted binaries for __marine_host_api_v3 ([#424](https://github.com/fluencelabs/marine/issues/424)) ([2ab2011](https://github.com/fluencelabs/marine/commit/2ab2011610775bda047663c624e434d966744ff0))


## [0.36.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.35.0...marine-runtime-v0.36.0) (2024-02-22)


### ⚠ BREAKING CHANGES

* rework wasi mapped dirs handing, relax restrictions ([#421](https://github.com/fluencelabs/marine/issues/421))

### Features

* rework wasi mapped dirs handing, relax restrictions ([#421](https://github.com/fluencelabs/marine/issues/421)) ([f54ca71](https://github.com/fluencelabs/marine/commit/f54ca715a362b51ca269c3882a0337b8d4390c3d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.29.0 to 0.30.0
    * marine-wasm-backend-traits bumped from 0.5.1 to 0.6.0
    * marine-wasmtime-backend bumped from 0.5.1 to 0.6.0

## [0.35.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.34.0...marine-runtime-v0.35.0) (2024-02-21)


### ⚠ BREAKING CHANGES

* support marine-rs-sdk 0.14

### Features

* support marine-rs-sdk 0.14 ([b20a27f](https://github.com/fluencelabs/marine/commit/b20a27f8b64733f3300afc8e4b5409337dc860aa))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.28.0 to 0.29.0

## [0.34.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.33.0...marine-runtime-v0.34.0) (2024-02-20)


### ⚠ BREAKING CHANGES

* support particle parameters in CallParameters (__marine_host_api_v2) ([#417](https://github.com/fluencelabs/marine/issues/417))

### Features

* support particle parameters in CallParameters (__marine_host_api_v2) ([#417](https://github.com/fluencelabs/marine/issues/417)) ([220c028](https://github.com/fluencelabs/marine/commit/220c02804567ef1c00ac8e02e08d4bbadd97bfd3))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.27.0 to 0.28.0

## [0.33.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.32.2...marine-runtime-v0.33.0) (2024-02-08)


### ⚠ BREAKING CHANGES

* **ABI:** support marine-rs-sdk host api versions, support worker_id in CallParamaters ([#409](https://github.com/fluencelabs/marine/issues/409))

### Features

* **ABI:** support marine-rs-sdk host api versions, support worker_id in CallParamaters ([#409](https://github.com/fluencelabs/marine/issues/409)) ([c948b8b](https://github.com/fluencelabs/marine/commit/c948b8b86674164020c79e900c58c5aff46c5eb7))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.26.2 to 0.27.0

## [0.32.2](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.32.1...marine-runtime-v0.32.2) (2024-02-05)


### Bug Fixes

* add/update total_memory_limit field in configs for tests ([#411](https://github.com/fluencelabs/marine/issues/411)) ([bdd109d](https://github.com/fluencelabs/marine/commit/bdd109d8424cb1159039a0c3082fad8450e2f328))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.26.1 to 0.26.2

## [0.32.1](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.32.0...marine-runtime-v0.32.1) (2024-01-07)


### Bug Fixes

* **docs:** add correct repository link for every published crate ([#403](https://github.com/fluencelabs/marine/issues/403)) ([ebb0bcb](https://github.com/fluencelabs/marine/commit/ebb0bcb1d15d37e8b5c10096ce42171a87abe0fa))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-core bumped from 0.26.0 to 0.26.1
    * marine-module-interface bumped from 0.8.0 to 0.8.1
    * marine-utils bumped from 0.5.0 to 0.5.1
    * it-json-serde bumped from 0.5.0 to 0.5.1
    * marine-wasm-backend-traits bumped from 0.5.0 to 0.5.1
    * marine-wasmtime-backend bumped from 0.5.0 to 0.5.1

## [0.32.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.31.0...marine-runtime-v0.32.0) (2023-12-14)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#400](https://github.com/fluencelabs/marine/issues/400))
* add memory limiting ([#391](https://github.com/fluencelabs/marine/issues/391))

### Features

* add memory limiting ([#391](https://github.com/fluencelabs/marine/issues/391)) ([662a492](https://github.com/fluencelabs/marine/commit/662a49204f98f481007aa4eb030bb8478cc066db))


### Bug Fixes

* **versions:** enforce minor version bumps ([#400](https://github.com/fluencelabs/marine/issues/400)) ([597ef4f](https://github.com/fluencelabs/marine/commit/597ef4f80d4be0170e8d575da1181647c284fe6c))

## [0.31.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.30.0...marine-runtime-v0.31.0) (2023-11-07)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397))
* propagate errors from linked modules ([#388](https://github.com/fluencelabs/marine/issues/388))

### Features

* **deps:** update rust crate log to 0.4.20 ([8a6035f](https://github.com/fluencelabs/marine/commit/8a6035f2f1f9d81895926dd8e612542570c5617f))
* **deps:** update rust crate serde_json to 1.0.107 ([0c1d378](https://github.com/fluencelabs/marine/commit/0c1d3780b04da3a63d7a59469f91bc056f3a56e7))
* **deps:** update rust crate thiserror to 1.0.50 ([0b88b23](https://github.com/fluencelabs/marine/commit/0b88b236015320972315b1bd7ae07f5277d6acbd))
* propagate errors from linked modules ([#388](https://github.com/fluencelabs/marine/issues/388)) ([a94494b](https://github.com/fluencelabs/marine/commit/a94494b042e32e284790d4ddc650e3086f6ab600))


### Bug Fixes

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397)) ([8c217c7](https://github.com/fluencelabs/marine/commit/8c217c7c3d367f6dcb6abeea0b54de88dbd17be5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-wasmtime-backend bumped from 0.3.0 to 0.4.0

## [0.30.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.29.0...marine-runtime-v0.30.0) (2023-09-13)


### ⚠ BREAKING CHANGES

* **deps:** update marine-rs-sdk-to 0.10.0 ([#364](https://github.com/fluencelabs/marine/issues/364))

### Features

* **deps:** update marine-rs-sdk-to 0.10.0 ([#364](https://github.com/fluencelabs/marine/issues/364)) ([036c334](https://github.com/fluencelabs/marine/commit/036c3348e3361e3a39eb79fb16641ef4bbff1f6c))

## [0.29.0](https://github.com/fluencelabs/marine/compare/marine-runtime-v0.28.0...marine-runtime-v0.29.0) (2023-08-09)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362))
* **deps:** update marine-rs-sdk

### Features

* **deps:** update marine-rs-sdk ([e7861f5](https://github.com/fluencelabs/marine/commit/e7861f5613b387ea59a05b9f91170b2b364e821c))


### Bug Fixes

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362)) ([bf8e2e9](https://github.com/fluencelabs/marine/commit/bf8e2e91141c216b1a8a1db572a01f921c77f543))

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

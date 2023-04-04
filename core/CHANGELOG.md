# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * marine-wasmtime-backend bumped from 0.2.0 to 0.2.1

## [0.20.3](https://github.com/fluencelabs/marine/compare/marine-core-v0.20.2...marine-core-v0.20.3) (2023-04-04)


### Bug Fixes

* **release-please:** Get rid of workspace.dependencies ([#316](https://github.com/fluencelabs/marine/issues/316)) ([71835e6](https://github.com/fluencelabs/marine/commit/71835e6762515a83cde1cc944d60352a4c1221f5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-module-info-parser bumped from 0.6.0 to 0.6.1
    * marine-it-interfaces bumped from 0.8.0 to 0.8.1
    * marine-it-parser bumped from 0.12.0 to 0.12.1
    * marine-it-generator bumped from 0.10.0 to 0.10.1
    * marine-module-interface bumped from 0.7.0 to 0.7.1
    * marine-wasm-backend-traits bumped from 0.2.0 to 0.2.1
    * marine-wasmtime-backend bumped from 0.2.1 to 0.2.2

## [0.20.1](https://github.com/fluencelabs/marine/compare/marine-core-v0.20.0...marine-core-v0.20.1) (2023-03-22)


### Bug Fixes

* **runtime:** support new wasm opcodes by removing unused memory limit setting ([#299](https://github.com/fluencelabs/marine/issues/299)) ([b9dbf67](https://github.com/fluencelabs/marine/commit/b9dbf6737655218619fb1275e564f03756c59a13))

## [0.20.0](https://github.com/fluencelabs/marine/compare/marine-core-v0.19.0...marine-core-v0.20.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-module-info-parser bumped from 0.5.1 to 0.6.0
    * marine-it-interfaces bumped from 0.7.3 to 0.8.0
    * marine-it-parser bumped from 0.11.1 to 0.12.0
    * marine-it-generator bumped from 0.9.5 to 0.10.0
    * marine-module-interface bumped from 0.6.1 to 0.7.0
    * marine-utils bumped from 0.4.0 to 0.5.0
    * marine-min-it-version bumped from 0.2.1 to 0.3.0
    * marine-wasm-backend-traits bumped from 0.1.0 to 0.2.0
    * marine-wasmtime-backend bumped from 0.1.0 to 0.2.0

## [Unreleased]

## [0.19.0] - 2022-12-06

### Fixed
- [**breaking**] bump minor versions where it was required in #189 (#212)
- fix tests after renaming (#174)

### Other
- *(deps)* update all non-major rust dependencies (#211)
- Add marine e2e (#230)
- *(build)* fix clippy warnings (#213)
- Update Rust crate semver to v1 (#198)
- Update all non-major Rust dependencies (#204)
- Update all non-major Rust dependencies (#189)
- bump crate versions that used marine-rs-sdk-main 0.6.15 (#185)
- Support marine-rs-sdk 0.7.0  (#180)
- Fix value after table problem in TomlMarineNamedModuleConfig(#175)
- Rename `FaaS` to `Marine`, `Runtime` to `Core` (#172)

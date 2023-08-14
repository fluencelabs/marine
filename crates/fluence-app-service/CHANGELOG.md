# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.24.0 to 0.24.1

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.26.0 to 0.26.1

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.26.1 to 0.26.2
    * marine-wasmtime-backend bumped from 0.2.0 to 0.2.1

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.26.2 to 0.26.3
    * marine-wasm-backend-traits bumped from 0.2.0 to 0.2.1
    * marine-wasmtime-backend bumped from 0.2.1 to 0.2.2

## [0.28.0](https://github.com/fluencelabs/marine/compare/fluence-app-service-v0.27.0...fluence-app-service-v0.28.0) (2023-08-09)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362))

### Bug Fixes

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362)) ([bf8e2e9](https://github.com/fluencelabs/marine/commit/bf8e2e91141c216b1a8a1db572a01f921c77f543))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.28.0 to 0.29.0

## [0.27.0](https://github.com/fluencelabs/marine/compare/fluence-app-service-v0.26.0...fluence-app-service-v0.27.0) (2023-08-04)


### ⚠ BREAKING CHANGES

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357))

### Bug Fixes

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357)) ([71e138d](https://github.com/fluencelabs/marine/commit/71e138dce31c2896bcd7b0657c3122c4b7f6402b))

## [0.26.0](https://github.com/fluencelabs/marine/compare/fluence-app-service-v0.25.3...fluence-app-service-v0.26.0) (2023-07-25)


### ⚠ BREAKING CHANGES

* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333))

### Features

* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333)) ([aeae703](https://github.com/fluencelabs/marine/commit/aeae703229f5f9410429390fe2e72d6084527f14))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.26.3 to 0.27.0
    * marine-wasm-backend-traits bumped from 0.2.1 to 0.3.0
    * marine-wasmtime-backend bumped from 0.2.2 to 0.3.0

## [0.25.0](https://github.com/fluencelabs/marine/compare/fluence-app-service-v0.24.0...fluence-app-service-v0.25.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.25.0 to 0.26.0
    * marine-min-it-version bumped from 0.2.1 to 0.3.0
    * marine-wasm-backend-traits bumped from 0.1.0 to 0.2.0
    * marine-wasmtime-backend bumped from 0.1.0 to 0.2.0

## [0.24.0](https://github.com/fluencelabs/marine/compare/fluence-app-service-v0.23.1...fluence-app-service-v0.24.0) (2023-03-03)


### ⚠ BREAKING CHANGES

* **fluence-app-service:** add separate bases for temp dirs and mapped dirs ([#288](https://github.com/fluencelabs/marine/issues/288))

### Features

* **fluence-app-service:** add separate bases for temp dirs and mapped dirs ([#288](https://github.com/fluencelabs/marine/issues/288)) ([1d86899](https://github.com/fluencelabs/marine/commit/1d868992bd944eb83926c18a17a24d135c692b4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-runtime bumped from 0.24.1 to 0.25.0

## [Unreleased]

## [0.23.0] - 2022-12-06

### Added
- [**breaking**] prohibit going out of service_dir in app-service (#244)

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.23.0 to 0.23.1

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.25.0 to 0.25.1

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.25.1 to 0.25.2

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.25.2 to 0.25.3
    * marine-wasm-backend-traits bumped from 0.2.0 to 0.2.1

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.25.3 to 0.26.0
    * marine-wasm-backend-traits bumped from 0.2.1 to 0.3.0

## [0.25.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.24.0...mrepl-v0.25.0) (2023-11-07)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397))

### Features

* **deps:** update rust crate anyhow to 1.0.75 ([#372](https://github.com/fluencelabs/marine/issues/372)) ([44b8e96](https://github.com/fluencelabs/marine/commit/44b8e96362cacc3d48a8a765fdd2c7aeb4fe695d))
* **deps:** update rust crate check-latest to 1.0.2 ([8480a49](https://github.com/fluencelabs/marine/commit/8480a49e084398d0b884f4f7fd2c73821f352145))
* **deps:** update rust crate log to 0.4.20 ([8a6035f](https://github.com/fluencelabs/marine/commit/8a6035f2f1f9d81895926dd8e612542570c5617f))
* **deps:** update rust crate serde_json to 1.0.107 ([0c1d378](https://github.com/fluencelabs/marine/commit/0c1d3780b04da3a63d7a59469f91bc056f3a56e7))


### Bug Fixes

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397)) ([8c217c7](https://github.com/fluencelabs/marine/commit/8c217c7c3d367f6dcb6abeea0b54de88dbd17be5))

## [0.24.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.23.0...mrepl-v0.24.0) (2023-09-13)


### ⚠ BREAKING CHANGES

* **deps:** update marine-rs-sdk-to 0.10.0 ([#364](https://github.com/fluencelabs/marine/issues/364))

### Features

* **deps:** update marine-rs-sdk-to 0.10.0 ([#364](https://github.com/fluencelabs/marine/issues/364)) ([036c334](https://github.com/fluencelabs/marine/commit/036c3348e3361e3a39eb79fb16641ef4bbff1f6c))

## [0.23.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.22.0...mrepl-v0.23.0) (2023-08-09)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362))
* **deps:** update marine-rs-sdk

### Features

* **deps:** update marine-rs-sdk ([e7861f5](https://github.com/fluencelabs/marine/commit/e7861f5613b387ea59a05b9f91170b2b364e821c))


### Bug Fixes

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362)) ([bf8e2e9](https://github.com/fluencelabs/marine/commit/bf8e2e91141c216b1a8a1db572a01f921c77f543))

## [0.22.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.21.4...mrepl-v0.22.0) (2023-08-04)


### ⚠ BREAKING CHANGES

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357))

### Bug Fixes

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357)) ([71e138d](https://github.com/fluencelabs/marine/commit/71e138dce31c2896bcd7b0657c3122c4b7f6402b))

## [0.21.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.20.0...mrepl-v0.21.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.24.0 to 0.25.0
    * marine-wasm-backend-traits bumped from 0.1.0 to 0.2.0

## [0.20.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.19.2...mrepl-v0.20.0) (2023-03-03)


### ⚠ BREAKING CHANGES

* **fluence-app-service:** add separate bases for temp dirs and mapped dirs ([#288](https://github.com/fluencelabs/marine/issues/288))

### Features

* **fluence-app-service:** add separate bases for temp dirs and mapped dirs ([#288](https://github.com/fluencelabs/marine/issues/288)) ([1d86899](https://github.com/fluencelabs/marine/commit/1d868992bd944eb83926c18a17a24d135c692b4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * fluence-app-service bumped from 0.23.1 to 0.24.0

## [0.19.1](https://github.com/fluencelabs/marine/compare/mrepl-v0.19.0...mrepl-v0.19.1) (2023-02-10)


### Bug Fixes

* **cli:** Make checking latest version an optional feature [#278](https://github.com/fluencelabs/marine/issues/278) ([da31cbb](https://github.com/fluencelabs/marine/commit/da31cbbe38e884ec7989c86af6ebf0fc19093341))

## [0.19.0](https://github.com/fluencelabs/marine/compare/mrepl-v0.18.8...mrepl-v0.19.0) (2023-02-08)


### ⚠ BREAKING CHANGES

* add pretty-print for repl output + small error messaging improvement ([#274](https://github.com/fluencelabs/marine/issues/274))

### Features

* add pretty-print for repl output + small error messaging improvement ([#274](https://github.com/fluencelabs/marine/issues/274)) ([9c1f20b](https://github.com/fluencelabs/marine/commit/9c1f20b8a74e467f2e403d18aabb7428baef9bc1))

## [Unreleased]

## [0.18.8] - 2022-12-06

### Other
- updated the following local packages: fluence-app-service

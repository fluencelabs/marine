# Changelog

## [0.3.0](https://github.com/fluencelabs/marine/compare/marine-wasmtime-backend-v0.2.2...marine-wasmtime-backend-v0.3.0) (2023-07-25)


### ⚠ BREAKING CHANGES

* **wasm-backend:** split Function trait, improve naming ([#347](https://github.com/fluencelabs/marine/issues/347))
* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333))

### Features

* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333)) ([aeae703](https://github.com/fluencelabs/marine/commit/aeae703229f5f9410429390fe2e72d6084527f14))
* **wasm-backend:** split Function trait, improve naming ([#347](https://github.com/fluencelabs/marine/issues/347)) ([0f9979a](https://github.com/fluencelabs/marine/commit/0f9979ae11267af119eccc3063c4001ffece4aee))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-wasm-backend-traits bumped from 0.2.1 to 0.3.0

## [0.2.2](https://github.com/fluencelabs/marine/compare/marine-wasmtime-backend-v0.2.1...marine-wasmtime-backend-v0.2.2) (2023-04-04)


### Bug Fixes

* **release-please:** Get rid of workspace.dependencies ([#316](https://github.com/fluencelabs/marine/issues/316)) ([71835e6](https://github.com/fluencelabs/marine/commit/71835e6762515a83cde1cc944d60352a4c1221f5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-wasm-backend-traits bumped from 0.2.0 to 0.2.1

## [0.2.1](https://github.com/fluencelabs/marine/compare/marine-wasmtime-backend-v0.2.0...marine-wasmtime-backend-v0.2.1) (2023-03-29)


### Bug Fixes

* **wasmtime-backend:** give access to stdout and stderr for instances ([#312](https://github.com/fluencelabs/marine/issues/312)) ([a76ace9](https://github.com/fluencelabs/marine/commit/a76ace9337df5b07d9da3f3a449cf12f14e4cf2f))

## [0.2.0](https://github.com/fluencelabs/marine/compare/marine-wasmtime-backend-v0.1.0...marine-wasmtime-backend-v0.2.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-wasm-backend-traits bumped from 0.1.0 to 0.2.0

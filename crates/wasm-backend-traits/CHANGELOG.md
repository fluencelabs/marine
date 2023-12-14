# Changelog

## [0.5.0](https://github.com/fluencelabs/marine/compare/marine-wasm-backend-traits-v0.4.0...marine-wasm-backend-traits-v0.5.0) (2023-12-14)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#400](https://github.com/fluencelabs/marine/issues/400))
* add memory limiting ([#391](https://github.com/fluencelabs/marine/issues/391))

### Features

* add memory limiting ([#391](https://github.com/fluencelabs/marine/issues/391)) ([662a492](https://github.com/fluencelabs/marine/commit/662a49204f98f481007aa4eb030bb8478cc066db))


### Bug Fixes

* **versions:** enforce minor version bumps ([#400](https://github.com/fluencelabs/marine/issues/400)) ([597ef4f](https://github.com/fluencelabs/marine/commit/597ef4f80d4be0170e8d575da1181647c284fe6c))

## [0.4.0](https://github.com/fluencelabs/marine/compare/marine-wasm-backend-traits-v0.3.0...marine-wasm-backend-traits-v0.4.0) (2023-11-07)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397))
* propagate errors from linked modules ([#388](https://github.com/fluencelabs/marine/issues/388))

### Features

* **deps:** update rust crate anyhow to 1.0.75 ([#372](https://github.com/fluencelabs/marine/issues/372)) ([44b8e96](https://github.com/fluencelabs/marine/commit/44b8e96362cacc3d48a8a765fdd2c7aeb4fe695d))
* **deps:** update rust crate paste to 1.0.14 ([e75dafe](https://github.com/fluencelabs/marine/commit/e75dafe7c20f2c3245aba50a40c9e3e5ab8f9410))
* **deps:** update rust crate thiserror to 1.0.50 ([0b88b23](https://github.com/fluencelabs/marine/commit/0b88b236015320972315b1bd7ae07f5277d6acbd))
* propagate errors from linked modules ([#388](https://github.com/fluencelabs/marine/issues/388)) ([a94494b](https://github.com/fluencelabs/marine/commit/a94494b042e32e284790d4ddc650e3086f6ab600))


### Bug Fixes

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397)) ([8c217c7](https://github.com/fluencelabs/marine/commit/8c217c7c3d367f6dcb6abeea0b54de88dbd17be5))

## [0.3.0](https://github.com/fluencelabs/marine/compare/marine-wasm-backend-traits-v0.2.1...marine-wasm-backend-traits-v0.3.0) (2023-07-25)


### ⚠ BREAKING CHANGES

* **marine-js:** replace old marine-js with common marine-runtime + backend traits impl for JS ([#332](https://github.com/fluencelabs/marine/issues/332))
* **wasm-backend:** split Function trait, improve naming ([#347](https://github.com/fluencelabs/marine/issues/347))
* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333))

### Features

* **marine-js:** replace old marine-js with common marine-runtime + backend traits impl for JS ([#332](https://github.com/fluencelabs/marine/issues/332)) ([a61ddfc](https://github.com/fluencelabs/marine/commit/a61ddfc4044b53a9d5f7864c933a48f7404c473c))
* **wasm-backend, app-service:** use String for wasi env vars + require Clone for Function trait   ([#333](https://github.com/fluencelabs/marine/issues/333)) ([aeae703](https://github.com/fluencelabs/marine/commit/aeae703229f5f9410429390fe2e72d6084527f14))
* **wasm-backend:** split Function trait, improve naming ([#347](https://github.com/fluencelabs/marine/issues/347)) ([0f9979a](https://github.com/fluencelabs/marine/commit/0f9979ae11267af119eccc3063c4001ffece4aee))

## [0.2.1](https://github.com/fluencelabs/marine/compare/marine-wasm-backend-traits-v0.2.0...marine-wasm-backend-traits-v0.2.1) (2023-04-04)


### Bug Fixes

* **release-please:** Get rid of workspace.dependencies ([#316](https://github.com/fluencelabs/marine/issues/316)) ([71835e6](https://github.com/fluencelabs/marine/commit/71835e6762515a83cde1cc944d60352a4c1221f5))

## [0.2.0](https://github.com/fluencelabs/marine/compare/marine-wasm-backend-traits-v0.1.0...marine-wasm-backend-traits-v0.2.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))

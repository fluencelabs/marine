# Changelog

* The following workspace dependencies were updated
  * dependencies
    * marine-it-interfaces bumped from 0.7.3 to 0.8.0
    * marine-module-interface bumped from 0.6.1 to 0.7.0
    * marine-utils bumped from 0.4.0 to 0.5.0
    * marine-min-it-version bumped from 0.2.1 to 0.3.0
    * it-json-serde bumped from 0.3.5 to 0.4.0







## [0.8.0](https://github.com/fluencelabs/marine/compare/marine-js-v0.7.4...marine-js-v0.8.0) (2023-11-07)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397))
* propagate errors from linked modules ([#388](https://github.com/fluencelabs/marine/issues/388))

### Features

* **deps:** update rust crate log to 0.4.20 ([8a6035f](https://github.com/fluencelabs/marine/commit/8a6035f2f1f9d81895926dd8e612542570c5617f))
* **deps:** update rust crate serde_json to 1.0.107 ([0c1d378](https://github.com/fluencelabs/marine/commit/0c1d3780b04da3a63d7a59469f91bc056f3a56e7))
* propagate errors from linked modules ([#388](https://github.com/fluencelabs/marine/issues/388)) ([a94494b](https://github.com/fluencelabs/marine/commit/a94494b042e32e284790d4ddc650e3086f6ab600))


### Bug Fixes

* **marine-js:** Add JSONValue return type ([#393](https://github.com/fluencelabs/marine/issues/393)) ([8ea6c3c](https://github.com/fluencelabs/marine/commit/8ea6c3cd1b150ec6093a333558f6956edcea8a37))
* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397)) ([8c217c7](https://github.com/fluencelabs/marine/commit/8c217c7c3d367f6dcb6abeea0b54de88dbd17be5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-js-backend bumped from 0.2.0 to 0.2.1

## [0.7.2](https://github.com/fluencelabs/marine/compare/marine-js-v0.7.1...marine-js-v0.7.2) (2023-08-04)


### Features

* **marine-js:** use wasmparser instead of walrus to parse module exports ([#359](https://github.com/fluencelabs/marine/issues/359)) ([550f7d3](https://github.com/fluencelabs/marine/commit/550f7d38513625cab65fe7aed3261863cc3769d9))

## [0.7.0](https://github.com/fluencelabs/marine/compare/marine-js-v0.6.0...marine-js-v0.7.0) (2023-08-03)


### ⚠ BREAKING CHANGES

* **marine-js:** update register_module interface to vastly improve performance ([#354](https://github.com/fluencelabs/marine/issues/354))

### Features

* **marine-js:** update register_module interface to vastly improve performance ([#354](https://github.com/fluencelabs/marine/issues/354)) ([1e1f71d](https://github.com/fluencelabs/marine/commit/1e1f71d630f8b5a53daab198489b5d805fad0989))

## [0.6.0](https://github.com/fluencelabs/marine/compare/marine-js-v0.5.0...marine-js-v0.6.0) (2023-08-02)


### ⚠ BREAKING CHANGES

* **marine-js:** Export wasm file from package.json ([#353](https://github.com/fluencelabs/marine/issues/353))
* **marine-js:** support call parameters  ([#351](https://github.com/fluencelabs/marine/issues/351))

### Features

* **marine-js:** Export wasm file from package.json ([#353](https://github.com/fluencelabs/marine/issues/353)) ([49a095a](https://github.com/fluencelabs/marine/commit/49a095a99bf04bb45c8ff36b7886528310b8a12d))
* **marine-js:** support call parameters  ([#351](https://github.com/fluencelabs/marine/issues/351)) ([456521b](https://github.com/fluencelabs/marine/commit/456521bf8bacc54d26f0537c7105971173431c1b))

## [0.5.0](https://github.com/fluencelabs/marine/compare/marine-js-v0.4.1...marine-js-v0.5.0) (2023-07-25)


### ⚠ BREAKING CHANGES

* **marine-js:** replace old marine-js with common marine-runtime + backend traits impl for JS ([#332](https://github.com/fluencelabs/marine/issues/332))

### Features

* **marine-js:** replace old marine-js with common marine-runtime + backend traits impl for JS ([#332](https://github.com/fluencelabs/marine/issues/332)) ([a61ddfc](https://github.com/fluencelabs/marine/commit/a61ddfc4044b53a9d5f7864c933a48f7404c473c))


### Bug Fixes

* **build:** add imports update into marine-js bindgen glue code patcher ([#348](https://github.com/fluencelabs/marine/issues/348)) ([08508ac](https://github.com/fluencelabs/marine/commit/08508ac9a3468c17135405e918fe188b5d75d761))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-js-backend bumped from 0.1.0 to 0.1.1

## [0.4.1](https://github.com/fluencelabs/marine/compare/marine-js-v0.4.0...marine-js-v0.4.1) (2023-04-04)


### Bug Fixes

* **release-please:** Get rid of workspace.dependencies ([#316](https://github.com/fluencelabs/marine/issues/316)) ([71835e6](https://github.com/fluencelabs/marine/commit/71835e6762515a83cde1cc944d60352a4c1221f5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-it-interfaces bumped from 0.8.0 to 0.8.1
    * marine-module-interface bumped from 0.7.0 to 0.7.1
    * it-json-serde bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/fluencelabs/marine/compare/marine-js-v0.3.45...marine-js-v0.4.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))

## [0.3.45](https://github.com/fluencelabs/marine/compare/marine-js-v0.3.44...marine-js-v0.3.45) (2023-02-15)


### Bug Fixes

* **marine-js:** add empty index.js as main entry point ([#282](https://github.com/fluencelabs/marine/issues/282)) ([cc430a0](https://github.com/fluencelabs/marine/commit/cc430a073517047921128e6f6bd6b221aabf71d1))

## [0.3.44](https://github.com/fluencelabs/marine/compare/marine-js-v0.3.43...marine-js-v0.3.44) (2023-02-06)


### Bug Fixes

* **ci:** Fix marine-js release build ([#272](https://github.com/fluencelabs/marine/issues/272)) ([dce6333](https://github.com/fluencelabs/marine/commit/dce6333f43e6258f41268fa62a1530694e21d5fe))

## [0.3.43](https://github.com/fluencelabs/marine/compare/marine-js-v0.3.42...marine-js-v0.3.43) (2023-02-03)


### Features

* **marine-js:** Switch MarineJS package From CJS to ESM format ([#265](https://github.com/fluencelabs/marine/issues/265)) ([9e2dd39](https://github.com/fluencelabs/marine/commit/9e2dd3912ed1db3820278f37ee095fa6acf409b6))

# Changelog

* The following workspace dependencies were updated
  * dependencies
    * marine-it-generator bumped from 0.10.0 to 0.10.1
    * marine-it-parser bumped from 0.12.0 to 0.12.1
    * marine-module-info-parser bumped from 0.6.0 to 0.6.1

* The following workspace dependencies were updated
  * dependencies
    * marine-it-generator bumped from 0.10.1 to 0.10.2
    * marine-it-parser bumped from 0.12.1 to 0.12.2
    * marine-module-info-parser bumped from 0.6.1 to 0.6.2

## [0.19.2](https://github.com/fluencelabs/marine/compare/marine-v0.19.1...marine-v0.19.2) (2024-01-10)


### Features

* **cli, mrepl:** support windows in marine cli and mrepl ([#406](https://github.com/fluencelabs/marine/issues/406)) ([71d1fb1](https://github.com/fluencelabs/marine/commit/71d1fb16ca322f5e227989fe521b3cbc5acbdff3))

## [0.19.1](https://github.com/fluencelabs/marine/compare/marine-v0.19.0...marine-v0.19.1) (2024-01-07)


### Features

* **aqua:** Generate `aqua` header instead of `module` ([#404](https://github.com/fluencelabs/marine/issues/404)) ([dd22da0](https://github.com/fluencelabs/marine/commit/dd22da07f80f4a74639244a838b1ca7f2a5bde73))


### Bug Fixes

* **docs:** add correct repository link for every published crate ([#403](https://github.com/fluencelabs/marine/issues/403)) ([ebb0bcb](https://github.com/fluencelabs/marine/commit/ebb0bcb1d15d37e8b5c10096ce42171a87abe0fa))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-it-generator bumped from 0.13.0 to 0.13.1
    * marine-it-parser bumped from 0.15.0 to 0.15.1
    * marine-module-info-parser bumped from 0.11.0 to 0.11.1

## [0.19.0](https://github.com/fluencelabs/marine/compare/marine-v0.18.0...marine-v0.19.0) (2023-12-14)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#400](https://github.com/fluencelabs/marine/issues/400))

### Bug Fixes

* **versions:** enforce minor version bumps ([#400](https://github.com/fluencelabs/marine/issues/400)) ([597ef4f](https://github.com/fluencelabs/marine/commit/597ef4f80d4be0170e8d575da1181647c284fe6c))

## [0.18.0](https://github.com/fluencelabs/marine/compare/marine-v0.17.0...marine-v0.18.0) (2023-11-07)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397))

### Features

* **deps:** update rust crate anyhow to 1.0.75 ([#372](https://github.com/fluencelabs/marine/issues/372)) ([44b8e96](https://github.com/fluencelabs/marine/commit/44b8e96362cacc3d48a8a765fdd2c7aeb4fe695d))
* **deps:** update rust crate check-latest to 1.0.2 ([8480a49](https://github.com/fluencelabs/marine/commit/8480a49e084398d0b884f4f7fd2c73821f352145))
* **deps:** update rust crate log to 0.4.20 ([8a6035f](https://github.com/fluencelabs/marine/commit/8a6035f2f1f9d81895926dd8e612542570c5617f))
* **deps:** update rust crate semver to 1.0.20 ([7b666ae](https://github.com/fluencelabs/marine/commit/7b666aeb40590cccda2d9a542024cf0928d9b2fa))
* **deps:** update rust crate serde_json to 1.0.107 ([0c1d378](https://github.com/fluencelabs/marine/commit/0c1d3780b04da3a63d7a59469f91bc056f3a56e7))
* **deps:** update rust crate thiserror to 1.0.50 ([0b88b23](https://github.com/fluencelabs/marine/commit/0b88b236015320972315b1bd7ae07f5277d6acbd))


### Bug Fixes

* **versions:** enforce minor version bumps ([#397](https://github.com/fluencelabs/marine/issues/397)) ([8c217c7](https://github.com/fluencelabs/marine/commit/8c217c7c3d367f6dcb6abeea0b54de88dbd17be5))

## [0.17.0](https://github.com/fluencelabs/marine/compare/marine-v0.16.0...marine-v0.17.0) (2023-09-13)


### ⚠ BREAKING CHANGES

* **deps:** update marine-rs-sdk-to 0.10.0 ([#364](https://github.com/fluencelabs/marine/issues/364))

### Features

* **deps:** update marine-rs-sdk-to 0.10.0 ([#364](https://github.com/fluencelabs/marine/issues/364)) ([036c334](https://github.com/fluencelabs/marine/commit/036c3348e3361e3a39eb79fb16641ef4bbff1f6c))

## [0.16.0](https://github.com/fluencelabs/marine/compare/marine-v0.15.0...marine-v0.16.0) (2023-08-09)


### ⚠ BREAKING CHANGES

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362))

### Bug Fixes

* **versions:** enforce minor version bump on minor dependency update ([#362](https://github.com/fluencelabs/marine/issues/362)) ([bf8e2e9](https://github.com/fluencelabs/marine/commit/bf8e2e91141c216b1a8a1db572a01f921c77f543))

## [0.15.0](https://github.com/fluencelabs/marine/compare/marine-v0.14.2...marine-v0.15.0) (2023-08-04)


### ⚠ BREAKING CHANGES

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357))

### Bug Fixes

* update versions to comply with semver ([#357](https://github.com/fluencelabs/marine/issues/357)) ([71e138d](https://github.com/fluencelabs/marine/commit/71e138dce31c2896bcd7b0657c3122c4b7f6402b))

## [0.14.0](https://github.com/fluencelabs/marine/compare/marine-v0.13.0...marine-v0.14.0) (2023-03-14)


### ⚠ BREAKING CHANGES

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219))

### Features

* decouple wasmer from Marine, replace it with generic backend interface ([#219](https://github.com/fluencelabs/marine/issues/219)) ([d3a773d](https://github.com/fluencelabs/marine/commit/d3a773df4f7ec80ab8146f68922802a4b9a450d0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * marine-it-generator bumped from 0.9.5 to 0.10.0
    * marine-it-parser bumped from 0.11.1 to 0.12.0
    * marine-module-info-parser bumped from 0.5.1 to 0.6.0

## [0.13.0](https://github.com/fluencelabs/marine/compare/marine-v0.12.7...marine-v0.13.0) (2023-03-03)


### ⚠ BREAKING CHANGES

* **cli:** use sdk dependency version from Cargo.lock instead of Cargo.toml ([#286](https://github.com/fluencelabs/marine/issues/286))

### Features

* **cli:** use sdk dependency version from Cargo.lock instead of Cargo.toml ([#286](https://github.com/fluencelabs/marine/issues/286)) ([fc384a4](https://github.com/fluencelabs/marine/commit/fc384a477c2274c9ebff4968871995935b5d6900))

## [0.12.7](https://github.com/fluencelabs/marine/compare/marine-v0.12.6...marine-v0.12.7) (2023-02-10)


### Bug Fixes

* **cli:** Make checking latest version an optional feature [#278](https://github.com/fluencelabs/marine/issues/278) ([da31cbb](https://github.com/fluencelabs/marine/commit/da31cbbe38e884ec7989c86af6ebf0fc19093341))

## [0.12.6](https://github.com/fluencelabs/marine/compare/marine-v0.12.5...marine-v0.12.6) (2023-01-27)


### Bug Fixes

* support workspace dependencies by using cargo_toml 0.14.0 ([#256](https://github.com/fluencelabs/marine/issues/256)) ([6ee6697](https://github.com/fluencelabs/marine/commit/6ee6697ed88297bbc26565514c6c54352a6ebab5))

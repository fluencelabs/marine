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

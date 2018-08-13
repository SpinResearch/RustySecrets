
# Changelog

## [Unreleased](https://github.com/SpinResearch/RustySecrets/compare/v0.2.2...master)

* Add EditorConfig configuration file ([17a9c14](https://github.com/SpinResearch/RustySecrets/commit/17a9c14))
* Add ErrorKind::ShareParsingInvalidShareThreshold ([55b7c78](https://github.com/SpinResearch/RustySecrets/commit/55b7c78))
* Add rust-toolchain file ([2ed5bfb](https://github.com/SpinResearch/RustySecrets/commit/2ed5bfb))
* Add support for custom RNGs in SSS and WrappedSecrets (#64) ([f83ef1b](https://github.com/SpinResearch/RustySecrets/commit/f83ef1b)), closes [#64](https://github.com/SpinResearch/RustySecrets/issues/64)
* Change signatures of share validation fns ([840f5cc](https://github.com/SpinResearch/RustySecrets/commit/840f5cc))
* Ensure there is at least one point in QuickCheck tests ([b477d3d](https://github.com/SpinResearch/RustySecrets/commit/b477d3d))
* Fix arg order missing shares validation ([fd74534](https://github.com/SpinResearch/RustySecrets/commit/fd74534))
* Fix wrong validation of threshold ([06033f1](https://github.com/SpinResearch/RustySecrets/commit/06033f1))
* Initial barycentric Langrange interpolation ([e767f28](https://github.com/SpinResearch/RustySecrets/commit/e767f28))
* Minor improvement to validation ([71064a6](https://github.com/SpinResearch/RustySecrets/commit/71064a6))
* MissingShares should take `u8` for `required` arg ([cb13a9b](https://github.com/SpinResearch/RustySecrets/commit/cb13a9b))
* More specific validation error when share thresholds mismatch ([df091b0](https://github.com/SpinResearch/RustySecrets/commit/df091b0))
* Remove `DuplicateShareData` error and validation ([cdcf012](https://github.com/SpinResearch/RustySecrets/commit/cdcf012))
* Remove `ShareIdentifierTooBig` error and validation ([ed867ba](https://github.com/SpinResearch/RustySecrets/commit/ed867ba))
* Rustfmt updates + refactor Travis configuration (#60) ([c25f661](https://github.com/SpinResearch/RustySecrets/commit/c25f661)), closes [#60](https://github.com/SpinResearch/RustySecrets/issues/60)
* Simplify share threshold and secret length consistency validation ([88743ca](https://github.com/SpinResearch/RustySecrets/commit/88743ca))
* Simplify threshold consistency validation ([5b35c69](https://github.com/SpinResearch/RustySecrets/commit/5b35c69))
* Standardize validation var identifier on ([c437775](https://github.com/SpinResearch/RustySecrets/commit/c437775))
* Update rand to ^0.4.2 ([e34f45d](https://github.com/SpinResearch/RustySecrets/commit/e34f45d))
* Use barycentric Lagrange interpolation in all cases. ([36dc14e](https://github.com/SpinResearch/RustySecrets/commit/36dc14e))
* Use Horner's method for evaluating polynomials ([73e45bf](https://github.com/SpinResearch/RustySecrets/commit/73e45bf))
* Validate shares have the same data length ([a6046dd](https://github.com/SpinResearch/RustySecrets/commit/a6046dd))
* Validation consistency between format & validation modules ([3f215cd](https://github.com/SpinResearch/RustySecrets/commit/3f215cd))

## [v0.2.2](https://github.com/SpinResearch/RustySecrets/compare/v0.2.1...v0.2.2) (2018-05-17)

## Changed

- Pin protobuf to >=1.4 && <1.6. Fixes #67.

## [v0.2.1](https://github.com/SpinResearch/RustySecrets/compare/v0.1.0...v0.2.1) (2018-03-08)

## Fixed

- Fix bug where threshold did not set deg of secret polynomial (@nvesely)

## Added

- Implement {Add, Div, Mul, Sub}Assign for Gf256 (@nvesely)

## [v0.1.0](https://github.com/SpinResearch/RustySecrets/compare/0.0.2...v0.1.0) (2018-02-13)

### Added

- Preliminary implementation of deterministic secret sharing (under feature `dss`).
  **WARNING:** This feature has not yet been audited, and should be considered pre-alpha.

### Changed

- `sss::generate_shares` has been renamed to `sss::split_secret`.
- `wrapped_secrets::generate_shares` has been renamed to `wrapped_secrets::split_secret`.
- New share format which supports versioning.
- Use `error-chain` instead of custom error struct.
- Errors related to a particular share now contain the share number.
- MIME type for wrapped share is now optional.
- Updated dependencies.

## [v0.0.2](https://github.com/SpinResearch/RustySecrets/releases/tag/0.0.2) (2016-04-06)

> Initial (alpha) release


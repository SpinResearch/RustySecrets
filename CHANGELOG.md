
# Changelog

## [Unreleased](https://github.com/SpinResearch/RustySecrets/compare/v0.2.1...master)

> Nothing yet.

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


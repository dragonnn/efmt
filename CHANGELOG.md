# Change Log

## Changed (Macros)
- Separation of lib.rs into lib, parser, write_gen and debug_gen
- Traits DisplayFormatted, DisplayPadded added
- Internal traits DisplayFormat, Display revised

## Changed (Lib)
- Add Float Formatting
- Add Padding
- Code reworked and restructured
- Source code for size and cycle measurement added

## [v0.3.0] - 2022-08-10

## Changed
- [breaking-change] Minimum Supported Rust Version (MSRV) guarantee has been removed

## Fixed

- fixed `uwrite!` and `uwriteln!` in presence of a third-party `Ok` constructor

## [v0.2.0] - 2022-08-10

### Added

- added support for `{:x}`-style formatting arguments. must be used with `ufmt` 0.1.2+

## [v0.1.2] - 2022-08-09

### Fixed

- `derive(uDebug)` on enums that have no variants

## [v0.1.1] - 2020-02-11

### Fixed

- fully qualify internal uses of `core::result::Result` to avoid problems when derive in presence of an imported `Result` type that's not libcore's

## v0.1.0 - 2019-11-17

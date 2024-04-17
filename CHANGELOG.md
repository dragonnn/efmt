# Change Log

# [v0.4.0] - 2024-04-17

## Changed
- macros and tfmt now have the same version and changelog as they are linked to each other
- tests can now be executed with and without feature std
- macros: separation of lib.rs into lib, parser, write_gen and debug_gen
- internal traits DisplayFormat, Display revised
- get rid of unitialised wherever possible - replace unitialised with Maybeuninit
- Documentation improved, 

## Added
- traits DisplayFormatted and DisplayPadded
- binary and octal formatting
- float formatting
- padding
- source code and performance chart for size and cycle measurement

## [v0.3.0] - 2022-08-10
- Below is the copy of ufmt changelog (macros)

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

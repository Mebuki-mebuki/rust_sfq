# Changelog

## [0.1.0] - 2025-05-30

First release of RustSFQ

## [0.1.3] - 2026-01-15

### Added

- Added changelog(this).

### Fixed

- Corrected gate names for AND and OR in both `RsfqlibVerilog` and `RsfqlibSpice` backends.
- Fixed incorrect Verilog output formatting in the `RsfqlibVerilog` backend.

## [1.0.0-beta] - 2026-03-06

### Notes

- This is a **beta release** and may not be stable.
- This version introduces **breaking changes**, and existing code may not be compatible.

### Changed

- The circuit description format has been updated to require **timing annotations** for logic simulation.

### Added

- Added the `LogicalVerilog` backend for performing logic simulation in Verilog, along with the corresponding library `lib/logical`.

## [1.0.1-beta] - 2026-03-06

### Fixed

- Fixed incorrect Verilog output formatting in the `RsfqlibVerilog` backend.

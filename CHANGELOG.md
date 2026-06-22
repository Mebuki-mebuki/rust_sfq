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

## [1.0.2-beta] - 2026-06-02

### Changed

- Changed the meaning of timing annotations.
  - For clocked gates, they represent the arrival order of pulses at that gate.
  - For non-clocked gates and subcircuits, there is no annotation.
  - Multi-cycle paths are expressed using `Circuit::add_delay()`.

- Added APIs like `and_p()` to represent typical pipeline timing.
  - example: the following two are equivalent.
    - `circuit.and_p(a, b, clk)`
    - `circuit.and(a % 1, b % 1, clk % 0)`

- Removed labeled APIs such as `and_labeled()`. Added `Wire::label()` instead.
  - example:
    - old: `let c = circuit.and_labeled(a1 % 1, b1 % 1, clk1 % 0, "c");`
    - new: `let c = circuit.and(a1 % 1, b1 % 1, clk1 % 0).label("c", &mut circuit);`

## [1.0.3] - 2026-06-22

### Changed

- Netlist generation now goes through `Design`. Replace direct backend calls such as `RsfqlibSpice::generate(&circuit)` with `design![&circuit].generate(RsfqlibSpice)` or `design![&circuit].print(RsfqlibSpice)`.
- `unify()` now updates gate connections directly instead of relying on wire aliases during backend generation.

### Added

- Added timing validation before backend generation, including validation across subcircuit boundaries.
- Added `Design` and the `design![...]` macro to check timing constraints and generate output for one or more circuits.
- Added cycle-level timing support to `LogicalVerilog`, including explicit multi-cycle delays through `Circuit::add_delay()`.
- Added timing-model documentation and runnable examples for pipelined, combinational, single-cycle, multi-cycle, mixed arrival-order, and unsatisfiable-timing cases.
- Added timing and logical-Verilog regression tests.

### Fixed

- Fixed LogicalVerilog generation for multi-stage register delays.

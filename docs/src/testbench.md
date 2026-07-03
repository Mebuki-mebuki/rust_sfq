# Testbench Generation

This page records the planned API and behavior for generating simulation
testbenches from a single Rust-side test pattern. The implementation is still
planned.

## Goal

The goal is to describe one test pattern in Rust and generate simulator input
for each backend:

- logical Verilog for Icarus Verilog
- rsfqlib Verilog for Icarus Verilog
- rsfqlib SPICE for JoSIM

Circuit module generation remains separate from testbench generation. Existing
backends such as `LogicalVerilog`, `RsfqlibVerilog`, and `RsfqlibSpice` generate
the circuit modules. The testbench generator creates the top-level simulation
wrapper, input patterns, dump settings, and observed outputs.

## Example

```rust
let test = Testbench::new(&fa)
    .cycles(11)
    .signals(["cin", "b", "a"], 0..8, 0.5)
    .constant("clk", 1, 0.0)
    .observe(["cout", "s"])
    .period_ps(100.0);
```

This describes an 11-cycle simulation. The first 8 cycles enumerate the
full-adder input truth table, and the remaining 3 cycles are automatically
filled with zeros for `cin`, `b`, and `a`.

The `clk` signal is not a special testbench concept. It is defined as a normal
input signal, here using the `constant` helper because many SFQ examples need a
clock pulse in every cycle.

## Stimulus API

Each input port must be specified by exactly one of these methods:

```rust
.signal(name, values, phase)
.signals(names, values, phase)
.constant(name, value, phase)
.pulse(name, cycles, phase)
.toggle(name, cycles, phase)
```

The same signal must not be specified more than once.

### `signal`

`signal` defines one 1-bit input signal by an integer sequence.

```rust
.signal("a", [0, 1, 0, 1], 0.5)
```

The values must be `0` or `1`.

### `signals`

`signals` defines multiple 1-bit input signals from an integer sequence.

```rust
.signals(["cin", "b", "a"], 0..8, 0.5)
```

Signal names are MSB first. For example, with `["cin", "b", "a"]`, the value
`4`, written as `0b100`, expands to:

```text
cin = 1
b   = 0
a   = 0
```

Each integer value must fit in the width given by the number of signal names.

### `constant`

`constant` defines a signal that has the same value for every simulation cycle.

```rust
.constant("clk", 1, 0.0)
```

The value must be `0` or `1`.

### `pulse`

`pulse` defines a signal that is `1` only at the listed cycle indices and `0`
otherwise.

```rust
.pulse("trigger", [3, 7, 10], 0.5)
```

The cycle indices must be within the simulation length specified by `cycles`.

### `toggle`

`toggle` defines a signal with initial value `0`. At each listed cycle index,
the signal value is inverted and keeps that new value until the next toggle.

```rust
.toggle("mode", [2, 6], 0.4)
```

In this example, `mode` is `0` before cycle 2, `1` from cycle 2 through cycle 5,
and `0` again from cycle 6 onward.

## Cycles And Padding

`cycles(n)` sets the total simulation length. This includes both active input
patterns and any quiet cycles at the end.

```rust
.cycles(11)
```

Length rules:

- if `signal` has more than `n` values, it is an error
- if `signal` has fewer than `n` values, zeros are appended
- if `signals` has more than `n` values, it is an error
- if `signals` has fewer than `n` values, zeros are appended for every signal
- `constant` expands to `n` cycles
- `pulse` and `toggle` cycle indices must be less than `n`

There is no separate `flush_cycles` setting. Quiet time at the end of a
simulation is represented by choosing a larger `cycles` value.

## Phase And Period

Each stimulus definition carries its own phase.

```rust
.signal("a", [0, 1, 0, 1], 0.5)
.constant("clk", 1, 0.0)
```

The phase is a real value in the range `0.0 <= phase < 1.0`. It represents the
position of the pulse within a simulation cycle.

`period_ps` is required and sets the cycle period in picoseconds:

```rust
.period_ps(100.0)
```

For physical backends, the event time is:

```text
cycle_index * period_ps + phase * period_ps
```

For `phase = 0.0`, the pulse is placed on the next cycle boundary instead of
time zero. This keeps the first physical pulse away from simulator startup time
and matches the existing hand-written samples.

The default SPICE pulse shape is the same as the existing hand-written samples:
the pulse rises from zero, reaches `827.13u`, and returns to zero over a short
picosecond-scale interval.

## Observed Signals

Input signals are automatically observed. `observe` lists only additional
signals, such as outputs or internal nodes.

```rust
.observe(["cout", "s"])
```

For the full-adder example above, the final observed signal list is:

```text
cin, b, a, clk, cout, s
```

SPICE generation uses the observed list for `.print v(...)` statements. Verilog
generation may initially use `$dumpvars(0, top)`, but the API-level meaning is
still "dump inputs plus the signals listed in `observe`."

SPICE internal hierarchical names, such as `a1.XTOP`, should be accepted by
`observe` without strict validation against the circuit port list.

## Generated Outputs

The test name determines backend output names.

For a circuit named `FullAdder`:

```text
logical Verilog: FullAdder.vcd
rsfqlib Verilog: FullAdder.vcd
rsfqlib SPICE: FullAdder.csv
```

The generated top-level testbench or SPICE file should be written separately
from the generated circuit module file.

## Validation Rules

The generator should validate the testbench before emitting backend code:

- `cycles(n)` must be specified
- `period_ps(...)` must be specified
- phase must satisfy `0.0 <= phase < 1.0`
- 1-bit values must be `0` or `1`
- `signals` values must fit in the given bit width
- the same signal must not be specified more than once
- every circuit input port must be specified
- stimulus names must refer to circuit input ports
- `observe` names are allowed to include backend-specific hierarchical names

## Planned Implementation Files

The planned implementation split is:

```text
src/testbench.rs
src/testbench_backends/mod.rs
src/testbench_backends/logical_verilog.rs
src/testbench_backends/rsfqlib_verilog.rs
src/testbench_backends/rsfqlib_spice.rs
tests/testbench.rs
```

`src/testbench.rs` should contain the shared testbench builder, validation, and
normalized stimulus representation.

`src/testbench_backends/*` should contain only backend-specific rendering for
the logical Verilog, rsfqlib Verilog, and rsfqlib SPICE simulation wrappers.

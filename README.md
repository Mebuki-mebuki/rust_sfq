# RustSFQ

**A domain-specific language for constructing Single Flux Quantum (SFQ) circuits in Rust.**

[![Crates.io](https://img.shields.io/crates/v/rust_sfq.svg)](https://crates.io/crates/rust_sfq)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://github.com/Mebuki-mebuki/rust_sfq/blob/main/LICENSE)

> ⚠️ **This project is in active development and considered beta.**
> APIs and behavior may change without notice.

**Documentation** is available at: <https://mebuki-mebuki.github.io/rust_sfq/>

## Features

- **Gate-level SFQ circuit design in Rust**  
  Define Single Flux Quantum (SFQ) circuits at the gate level using a Rust-native DSL.

- **Automatic wire name generation with optional labels**  
  Wire names are automatically generated to ensure uniqueness, but you can also assign explicit labels when needed.

- **Input-output consistency validation**  
  The system guarantees that all wires satisfies input-output consistency. For example, no wire can be used multiple times unless explicitly split using a `Split` gate.

- **Multiple simulation netlist formats supported**  
  Export circuits to various netlist formats for use in different SFQ simulation or analysis tools.

## Example

### DSL description for half adder

```rust
use rust_sfq::*;

fn main() {
    let (mut circuit, [a, b, clk], [], [o_c, o_s], []) =
        Circuit::create(["a", "b", "clk"], [], ["c", "s"], [], "HalfAdder");

    let (a1, a2) = circuit.split(a);
    let (b1, b2) = circuit.split(b);
    let (clk1, clk2) = circuit.split(clk);

    let c = circuit.and_p(a1, b1, clk1);
    let s = circuit.xor_p(a2, b2, clk2);

    circuit.unify(c, o_c);
    circuit.unify(s, o_s);

    design![&circuit].print(RsfqlibSpice);
}
```

### Output in SPICE format

```text
.subckt HalfAdder a b clk c s
XSPLIT1 a _SPLIT1_q1 _SPLIT1_q2 THmitll_SPLIT
XSPLIT2 b _SPLIT2_q1 _SPLIT2_q2 THmitll_SPLIT
XSPLIT3 clk _SPLIT3_q1 _SPLIT3_q2 THmitll_SPLIT
XAND4 _SPLIT1_q1 _SPLIT2_q1 _SPLIT3_q1 c THmitll_AND2
XXOR5 _SPLIT1_q2 _SPLIT2_q2 _SPLIT3_q2 s THmitll_XOR
.ends
```

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
rust_sfq = "0.1.3"
```

## Changelog

See the [CHANGELOG](./CHANGELOG.md) for a list of updates and version history.

## Third-party libraries

RustSFQ includes a subset of the RSFQ cell library (<https://github.com/sunmagnetics/RSFQlib>) in `lib/rsfqlib`:

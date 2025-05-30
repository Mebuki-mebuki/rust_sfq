# RustSFQ

**A domain-specific language for constructing Single Flux Quantum (SFQ) circuits in Rust.**

[![Crates.io](https://img.shields.io/crates/v/rust_sfq.svg)](https://crates.io/crates/rust_sfq)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://github.com/Mebuki-mebuki/rust_sfq/blob/main/LICENSE)

> ⚠️ **This project is in active development and considered beta.**
> APIs and behavior may change without notice.

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

### DSL description for half adder:

```rust
use rust_sfq::*;

fn main() {
    let (mut circuit, [a, b, clk], [], [o_c, o_s], []) =
        Circuit::create(["a", "b", "clk"], [], ["c", "s"], [], "HalfAdder");

    let [a1, a2] = circuit.split(a);
    let [b1, b2] = circuit.split(b);
    let [clk1, clk2] = circuit.split(clk);
    let clk1 = clk1 % circuit.label("clk_1");

    let c = circuit.and(a1, b1, clk1);
    let s = circuit.xor(a2, b2, clk2);

    circuit.unify(c, o_c);
    circuit.unify(s, o_s);

    println!("{}", RsfqlibSpice::generate(&circuit));
}
```

### Output in SPICE format:
```
.subckt HalfAdder a b clk  c s 
XSPLIT1 a _XSPLIT1_q1 _XSPLIT1_q2 THmitll_SPLIT
XSPLIT2 b _XSPLIT2_q1 _XSPLIT2_q2 THmitll_SPLIT
XSPLIT3 clk clk_1 _XSPLIT3_q2 THmitll_SPLIT
XAND4 _XSPLIT1_q1 _XSPLIT2_q1 clk_1 c THmitll_AND
XXOR5 _XSPLIT1_q2 _XSPLIT2_q2 _XSPLIT3_q2 s THmitll_XOR
.ends
```

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
sfq_dsl = "0.1"
```
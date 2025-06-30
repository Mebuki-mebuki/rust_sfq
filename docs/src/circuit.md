# Circuit

## Overview

The `Circuit` struct corresponds to a subcircuit in SPICE and serves as the main object for constructing SFQ circuits in RustSFQ.

The `Circuit` instance holds all the necessary information to generate a netlist.

By passing a reference to a backend's `generate()` function, you can convert the circuit into a string in formats such as SPICE or Verilog, depending on the backend.

---

## Type Parameter

A `Circuit` is parameterized by four compile-time constants representing the number of I/O channels:

```rust
Circuit<NUM_INPUT, NUM_COUNTER_INPUT, NUM_OUTPUT, NUM_COUNTER_OUTPUT>
```

Input and output are I/O channels for `Wire`. These are normal I/O channels.

Counter input and counter output are I/O chennels for `CounterWire`.
Counter input is logical input for `CounterWire`, but it is physical output in netlist.

---

## Creating a Circuit Instance

A new `Circuit` can be instantiated using the `Circuit::create()` function.
This function defines the structure of the circuit by specifying its I/O ports and name.

```rust
pub fn create(
    inputs: [&str; N_I],
    counter_inputs: [&str; N_CI],
    outputs: [&str; N_O],
    counter_outputs: [&str; N_CO],
    name: &str,
) -> (Circuit<N_I, N_CI, N_O, N_CO>, [Wire; N_I], [CounterWire; N_CI], [CounterWire; N_O], [Wire; N_CO])
```

**Parameters**:

- `inputs`: Array of names for input wires
- `counter_inputs`: Array of names for counter input wires
- `outputs`: Array of names for output wires
- `counter_outputs`: Array of names for counter output wires
- `name`: A string representing the name of the subcircuit (used in the netlist)

The lengths of these arrays determine the type parameters of the `Circuit`.

**Return Value**:

This function returns a tuple of:

- The `Circuit` instance itself
- An array of `Wire` instances for the input ports
- An array of `CounterWire` instances for the counter input ports
- An array of `CounterWire` instances for the output ports
- An array of `Wire` instances for the counter output ports

The wires for input ports are used to start constructing the circuit while the wires for output ports are used to finish constructing.

---

## Functions

To construct a circuit, a `Circuit` **instance** provides the following functions:

### Logic Gates

The `and()` function adds an AND gate to the circuit.
The `and_labeled()` is a convenience function that combines `and()` with `label()`, allowing you to define the gate and assign a label to its output wire in a single step.

```rust
pub fn and(&self, a: Wire, b: Wire, clk: Wire) -> Wire
pub fn and_labeled(&self, a: Wire, b: Wire, clk: Wire, label: &str) -> Wire
```

This function takes ownership of the input wires: `a`, `b`, and `clk`.
After being passed into the function, these wire values cannot be used again elsewhere in the program.

The function returns a new `Wire` that represents the output of the AND gate.

---

Other logic and routing gates (OR, XOR, XNOR, NOT, DFF, NDRO, JTL, BUFF, MERGE, and ZERO_ASYNC) are provided as similar functions. While the number of their input wires may differ, all of these functions return a single ``Wire`` corresponding to the gate's output.

---

### SPLIT

The `split()` function adds an SPLIT gate to the circuit.

```rust
pub fn split(&mut self, a: Wire) -> (Wire, Wire)
pub fn split_labeled(&mut self, a: Wire, label1: &str, label2: &str) -> (Wire, Wire)
```

The function takes one input `Wire` and returns a tuple of two new `Wire` instances.

This is necessary whenever the same signal needs to be used as input to multiple gates, ensuring that each use has its own distinct `Wire` object.

---

### Gates for CounterWire

To support circuits employing counter-flow clocking, BUFF and SPLIT are available for `CounterWire`.

```rust
pub fn cbuff(&mut self, q: CounterWire) -> CounterWire
pub fn cbuff_labeled(&mut self, q: CounterWire, label: &str) -> CounterWire
pub fn csplit(&mut self, q1: CounterWire) -> (Wire, CounterWire)
pub fn csplit_labeled(&mut self, q1: CounterWire, label_q2: &str, label_a: &str) -> (Wire, CounterWire) 
pub fn csplit2(&mut self, q1: CounterWire, q2: CounterWire) -> CounterWire 
pub fn csplit2_labeled(&mut self, q1: CounterWire, q2: CounterWire, label: &str) -> CounterWire
```

The `cbuff()` function takes a `CounterWire` representing the **output** of a BUFF gate then returns a new `CounterWire` representing the **input** of the gate.

The `csplit()` function takes **one** `CounterWire` representing the **one output** of a SPLIT gate then returns a tuple of a new `Wire` representing **the other output** of the gate and a new `CounterWire` representing the **input** of the gate.

The `csplit2()` function takes two `CounterWire`s representing the **outputs** of a SPLIT gate then returns a new `CounterWire` representing the **input** of the gate.

---

### Subcircuits

The `subcircuit()` function allows you to instantiate a reusable subcircuit within a larger circuit.

```rust
pub fn subcircuit<const M_I: usize, const M_CI: usize, const M_O: usize, const M_CO: usize>(
    &mut self,
    circuit: &Circuit<M_I, M_CI, M_O, M_CO>,
    inputs: [Wire; M_I],
    counter_inputs: [CounterWire; M_CI],
) -> ([Wire; M_O], [CounterWire; M_CO])
```

It takes:

- A **reference** to an existing `Circuit`
- An array of `Wire`s for the subcircuit's inputs
- An array of `CounterWire`s for the subcircuit's counter inputs

It returns a tuple of:

- An array of `Wire`s for the subcircuit’s outputs
- An array of `CounterWire`s for the subcircuit’s counter outputs

The input arrays are checked at compile time for correct lengths, ensuring type safety

The subcircuit is passed by reference, so ownership is preserved and the same subcircuit can be reused multiple times in different contexts

---

### Loops

To construct feedback loops, use the `gen_loop()` function:

```rust
pub fn gen_loop(&mut self, label: &str) -> (Wire, CounterWire)
```

This returns a pair of `Wire` and `CounterWire` representing the same wire.

By unifying the `CounterWire` with another `Wire` later, you can ensure the `Wire` has a driver.

---

### Unification

The `unify()` function connects a `Wire` and a `CounterWire`.

Since a `Wire` has a driver and a `CounterWire` has a receiver, unifying them creates no new wire. Therefore, the function has no return value.

```rust
pub fn unify(&mut self, wire: Wire, cwire: CounterWire)
```

This is primarily used in two scenarios:

- To connect a wire to an output port of the circuit
- To close a feedback loop created with `gen_loop()`

The label of the unified wire is determined as follows:

- If only one side has an explicit label, that label is used
- If both sides have labels and they match, the label is retained
- If both sides have labels and they differ, the function raises an error

---

### Labeling

You can assign an explicit label to a wire using the `label()` function.

This function is defined generically for both `Wire` and `CounterWire`, meaning that `T` can be either type:

```rust
pub fn label<T>(&mut self, wire: &T, label: &str)
```

The function takes a **reference** to the wire, **so ownership is not moved**.

Explicit labels must not begin with an underscore to avoid conflicts with automatically generated labels.

A wire that already has an explicit label cannot be labeled again.

RustSFQ does not check for collisions among explicitly assigned labels; it is the user's responsibility to ensure uniqueness.

---

## Exporting

By passing a reference of `Circuit` instance into a backend’s `generate()` function, you can convert the circuit into a backend-specific string representation.

The resulting string can be printed to standard output to obtain the final netlist.

```rust
use rust_sfq::*;

fn half_adder() -> Circuit<3, 0, 2, 0> { ... }
fn full_adder(ha: &Circuit<3, 0, 2, 0>) -> Circuit<4, 0, 2, 0> { ... }

fn main() {
    let half_adder = half_adder();
    let full_adder = full_adder(&half_adder);
    
    type Backend = RsfqlibSpice;
    println!("{}", Backend::generate(&half_adder));
    println!("{}", Backend::generate(&full_adder));
}
```

# Wire and CounterWire

## Overview

`Wire` and `CounterWire` represent electrical nets in a netlist and are used to describe the connections between gates within a circuit.

A `Wire` appears when constructing a circuit in a forward direction. It represents a net that has **already been driven**, but has **not yet been received**.  
A `CounterWire` is used when constructing circuits in a counter-flow manner, such as in counter-flow clocking. It represents a net that has **already been received**, but has **not yet been driven**.

These objects are governed by Rust's ownership system, which ensures that each wire has exactly **one driver and one receiver**.

## Generating Wires

Although `Wire` and `CounterWire` are structs, their constructors are private. Users **cannot instantiate them freely**.

They can only be obtained in the following ways:

- As circuit's inputs or outputs when calling `Circuit::create()`
- As outputs from gate functions such as `and()`, `xor()`, etc.
- From loop construction using `gen_loop()`

Also, `Wire` instances cannot be cloned.

## Using Wires

Wires are used in the following scenarios:

- **Gate inputs**: You pass `Wire` instances as arguments to gate functions like `circuit.and(a, b, clk)`. In doing so, ownership of the `Wire` is moved and cannot be reused.
- **Labeling**: If you want to assign a label to a wire, pass a reference to `label()` like `circuit.label(&a)`.
- **Unification**: When connecting a `Wire` and a `CounterWire`, you pass them to `unify()` like `circuit.unify(a, b)`. The `Wire` and `CounterWire` is consumed (ownership is moved).

The reason `Circuit::create()` returns **`CounterWire`s for outputs** is because those nets are "to be driven." You use `unify()` to connect a `Wire` (already driven) to these `CounterWire`s to complete the connection.

## Prevention of Multiple Use

In SFQ circuits, multiple receivers (fanout) from a single wire is prohibited. This restriction is **enforced statically** via Rust’s ownership system.

When you use a `Wire` as an input to a gate function, ownership is moved.

You cannot pass the same `Wire` to another gate function afterward, preventing accidental multi-use.

```rust
let (mut circuit, [a, clk], [], [], []) =
    Circuit::create(["a", "clk"], [], [], [], "invalid");

let c = circuit.dff(a, clk);
let d = circuit.not(a, clk);    // use of moved value. failed to compile
```

## Prevention of Unused Wires

Rust’s ownership system guarantees that a `Wire` is used **at most once**, but **not necessarily at least once**.

In most cases, an unused `Wire` will be caught by the Rust compiler as an unused variable.
However, it is possible to bypass this warning like this:

```rust
let (mut circuit, [a, b, c], [], [], []) =
    Circuit::create(["a", "b", "c"], [], [], [], "invalid");

// underscored variable name
let _a = circuit.jtl(a);

// discard return value
circuit.jtl(b);

// irrelevant use
println!("{:?}", c.type_id());
```

To catch such silent mistakes, RustSFQ performs **runtime validation**:

- Each `Wire` internally tracks a private counter for receivers.
- When a `Wire` is dropped (i.e., its destructor is called), RustSFQ checks whether it has exactly one receiver.
- If not, a runtime error is reported.

This mechanism ensures correctness even in subtle or intentionally suppressed cases.

# Available Gates and Backends

## Compatibility table (as of version 1.0.3)

| Gate | LogicalVerilog | RsfqlibSpice | RsfqlibVerilog |
| ---- | :------------: | :----------: | :------------: |
| JTL | ✅ | ✅ | ✅ |
| SPLIT | ✅ | ✅ | ✅ |
| MERGE | ✅ | ✅ | ✅ |
| AND | ✅ | ✅ | ✅ |
| OR | ✅ | ✅ | ✅ |
| XOR | ✅ | ✅ | ✅ |
| NOT | ✅ | ✅ | ✅ |
| XNOR | ✅ | ✅ | ✅ |
| DFF | ✅ | ✅ | ✅ |
| NDRO | ✅ | ✅ | ✅ |
| BUFF | ✅ | ✅ | ✅ |
| ZERO_ASYNC | ✅ | ✅ | ✅ |
| TERMINATE | ✅ | ✅ | ✅ |

## Backends

Backends are selected when generating a `Design`:

```rust
let netlist = design![&circuit].generate(RsfqlibSpice);
```

- `LogicalVerilog`: simple logical Verilog output for simulation
- `RsfqlibSpice`: SPICE output using RSFQlib (<https://github.com/sunmagnetics/RSFQlib>)
- `RsfqlibVerilog`: Verilog output using RSFQlib cell names

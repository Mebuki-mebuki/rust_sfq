# Simulation Examples

## Requirements

- Icarus Verilog
- gtkwave
- JoSIM
- uv (recommended)

## Logical

```shell
cd logical
cargo run logical > modules.v
cargo run logical-testbench > fulladder.sv
iverilog -g2012 -s top -I ../../../lib/logical/ fulladder.sv 
./a.out
gtkwave FullAdder.vcd
```

## Spice

```shell
cd spice

# python environment (if needed)
uv venv
uv pip install -r requirements.txt

cargo run spice > modules.cir
cargo run spice-testbench > fulladder.cir
josim-cli -o FullAdder.csv fulladder.cir
uv run josim-plot2.py FullAdder.csv -t stacked
```

## Verilog

```shell
cd verilog
cargo run verilog > modules.v
cargo run verilog-testbench > fulladder.sv
iverilog -g2012 -s top -I ../../../lib/rsfqlib/verilog/ fulladder.sv
./a.out
gtkwave FullAdder.vcd
```

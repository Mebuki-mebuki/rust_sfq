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
iverilog -g2012 -s top -I ../../../lib/logical/ fulladder.sv 
./a.out
gtkwave fulladder.vcd
```

## Spice

```shell
cd spice

# python environment (if needed)
uv venv
uv pip install -r requirements.txt

cargo run spice > modules.cir
josim-cli -o fulladder.csv fulladder.cir
uv run josim-plot2.py fulladder.csv -t stacked
```

## Verilog

```shell
cd verilog
cargo run verilog > modules.v
iverilog -g2012 -s top -I ../../../lib/rsfqlib/ fulladder.sv
./a.out
gtkwave fulladder.vcd
```

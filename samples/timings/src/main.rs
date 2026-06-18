use rust_sfq::*;
use std::env;

fn nor_pipelined() -> Circuit<3, 0, 1, 0> {
    let inputs = ["a", "b", "clk"];
    let outputs = ["y"];
    let (mut ckt, [a, b, clk], [], [y_out], []) =
        Circuit::create(inputs, [], outputs, [], "NorPipelined");

    let (clk1, clk2) = ckt.split(clk);
    ckt.label(&clk1, "clk1");
    let x = ckt.or(a % 1, b % 1, clk1 % 0).label("x", &mut ckt);

    ckt.label(&clk2, "clk2");
    let y = ckt.not(x % 1, clk2 % 0);

    ckt.unify(y, y_out);

    return ckt;
}

fn nor_combinational() -> Circuit<3, 0, 1, 0> {
    let inputs = ["a", "b", "clk"];
    let outputs = ["y"];
    let (mut ckt, [a, b, clk], [], [y_out], []) =
        Circuit::create(inputs, [], outputs, [], "NorCombinational");

    let (clk1, clk2) = ckt.split(clk);
    ckt.label(&clk1, "clk1");
    let x = ckt.or(a % 0, b % 0, clk1 % 1).label("x", &mut ckt);

    let clk2 = ckt.buff(clk2);
    let clk2 = ckt.buff(clk2);
    ckt.label(&clk2, "clk2");
    let y = ckt.not(x % 0, clk2 % 1);

    ckt.unify(y, y_out);

    return ckt;
}

fn main() {
    let nor_pipelined = nor_pipelined();
    let nor_combinational = nor_combinational();
    let complex_timing = complex_timing();
    let design = design!(&nor_pipelined, &nor_combinational, &complex_timing);

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("logical") => design.print(LogicalVerilog),
        Some("spice") => design.print(RsfqlibSpice),
        Some("verilog") => design.print(RsfqlibVerilog),
        _ => {
            println!("Usage: cargo run [logical|spice|verilog]");
        }
    }
}

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

fn single_cycle_path() -> Circuit<2, 0, 1, 0> {
    let inputs = ["a", "clk"];
    let outputs = ["y"];
    let (mut ckt, [a, clk], [], [y_out], []) =
        Circuit::create(inputs, [], outputs, [], "SingleCyclePath");

    let (a1, a2) = ckt.split(a);
    ckt.label(&a1, "a1");
    ckt.label(&a2, "a2");

    let (clk1, clk2) = ckt.split(clk);
    ckt.label(&clk1, "clk1");
    let x = ckt.dff_p(a1, clk1);
    ckt.label(&x, "x");

    ckt.label(&clk2, "clk2");
    let y = ckt.and_p(x, a2, clk2);
    let y = ckt.jtl(y);

    ckt.unify(y, y_out);

    return ckt;
}

fn multi_cycle_path() -> Circuit<2, 0, 1, 0> {
    let inputs = ["a", "clk"];
    let outputs = ["y"];
    let (mut ckt, [a, clk], [], [y_out], []) =
        Circuit::create(inputs, [], outputs, [], "MultiCyclePath");

    let (a1, a2) = ckt.split(a);
    ckt.label(&a1, "a1");

    // physical delay for analog simulation
    let mut a2 = a2;
    ckt.label(&a2, "a2_start");
    for _ in 0..32 {
        a2 = ckt.buff(a2);
    }
    ckt.label(&a2, "a2_end");

    // cycle delay for logical simulation
    ckt.add_delay(&a2, 2);

    let (clk1, clk2) = ckt.split(clk);
    ckt.label(&clk1, "clk1");
    let x = ckt.dff_p(a1, clk1);
    let x = ckt.buff(x);
    let x = ckt.buff(x);
    ckt.label(&x, "x");

    ckt.label(&clk2, "clk2");
    let y = ckt.and_p(x, a2, clk2);
    let y = ckt.jtl(y);

    ckt.unify(y, y_out);

    return ckt;
}

fn clock_between_data() -> Circuit<3, 0, 1, 0> {
    let inputs = ["a", "b", "clk"];
    let outputs = ["y"];
    let (mut ckt, [a, b, clk], [], [y_out], []) =
        Circuit::create(inputs, [], outputs, [], "ClockBetweenData");

    let (clk1, clk2) = ckt.split(clk);
    ckt.label(&clk1, "clk1");

    let x = ckt.and(a % 1, b % 1, clk1 % 0).label("x", &mut ckt);

    // delay clk2
    let mut clk2 = clk2;
    for _ in 0..5 {
        clk2 = ckt.buff(clk2);
    }
    ckt.label(&clk2, "clk2");

    let (y1, y1_) = ckt.gen_loop("y1");

    // x arrives before the clock, while y1 arrives after it.
    // This is neither the usual pipelined order nor the combinational order.
    let y0 = ckt.or(x % 0, y1 % 2, clk2 % 1);
    ckt.label(&y0, "y0");

    let (y, y1) = ckt.split(y0);
    ckt.unify(y, y_out);
    ckt.unify(y1, y1_);

    return ckt;
}

fn unsatisfiable_timing() -> Circuit<3, 0, 1, 0> {
    let inputs = ["a", "b", "clk"];
    let outputs = ["y"];
    let (mut ckt, [a, b, clk], [], [y_out], []) =
        Circuit::create(inputs, [], outputs, [], "UnsatisfiableTiming");

    let (clk1, clk2) = ckt.split(clk);
    ckt.label(&clk1, "clk1");

    let x = ckt.and(a % 1, b % 1, clk1 % 0).label("x", &mut ckt);

    let (y1, y1_) = ckt.gen_loop("y1");

    // It requires y1 precedes clk2, but y1 is output after clk2 arrives.
    // This is an unsatisfiable timing constraint.
    let y0 = ckt.or(x % 0, y1 % 0, clk2 % 1);

    let (y, y1) = ckt.split(y0);
    ckt.unify(y, y_out);
    ckt.unify(y1, y1_);

    return ckt;
}

fn main() {
    let nor_pipelined = nor_pipelined();
    let nor_combinational = nor_combinational();
    let single_cycle_path = single_cycle_path();
    let multi_cycle_path = multi_cycle_path();
    let clock_between_data = clock_between_data();
    let design = design!(
        &nor_pipelined,
        &nor_combinational,
        &single_cycle_path,
        &multi_cycle_path,
        &clock_between_data
    );

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("logical") => design.print(LogicalVerilog),
        Some("spice") => design.print(RsfqlibSpice),
        Some("verilog") => design.print(RsfqlibVerilog),
        Some("invalid") => {
            let invalid = unsatisfiable_timing();
            design!(&invalid).print(LogicalVerilog);
        }
        _ => {
            println!("Usage: cargo run [logical|spice|verilog|invalid]");
        }
    }
}

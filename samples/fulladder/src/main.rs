use rust_sfq::*;
use std::env;

fn half_adder_circuit() -> Circuit<3, 0, 2, 0> {
    let inputs = ["a", "b", "clk"];
    let outputs = ["c", "s"];
    let (mut circuit, [a, b, clk], [], [c_o, s_o], []) =
        Circuit::create(inputs, [], outputs, [], "HalfAdder");

    let (clk1, clk2) = circuit.split(clk); // automatically assign name
    let (a1, a2) = circuit.split(a);
    let (b1, b2) = circuit.split(b);

    let c = circuit
        .and(a1 % 1, b1 % 1, clk1 % 0)
        .label("c", &mut circuit); // explicitly assign name
    let s = circuit.xor_p(a2, b2, clk2).label("s", &mut circuit);

    circuit.unify(c, c_o);
    circuit.unify(s, s_o);

    return circuit;
}

fn full_adder_circuit(ha: &Circuit<3, 0, 2, 0>) -> Circuit<4, 0, 2, 0> {
    let inputs = ["a", "b", "cin", "clk"];
    let outputs = ["cout", "s"];
    let (mut circuit, [a, b, cin, clk], [], [cout_o, s_o], []) =
        Circuit::create(inputs, [], outputs, [], "FullAdder");

    let (clk, clk1) = circuit.split(clk);
    let (clk, clk2) = circuit.split(clk);
    let ([c1, s1], []) = circuit.subcircuit(ha, [a, b, clk1], []);
    let cin = circuit.dff_p(cin, clk2);

    let (clk3, clk4) = circuit.split(clk);

    let c1 = circuit.buff(c1); // delay adjustment
    let c1 = circuit.buff(c1); // Not needed for logical simulation.
    let c1 = circuit.dff_p(c1, clk3);

    let s1 = circuit.buff(s1); // delay adjustment
    let s1 = circuit.buff(s1);
    let cin = circuit.buff(cin);
    let cin = circuit.buff(cin);
    let ([c2, s2], []) = circuit.subcircuit(ha, [s1, cin, clk4], []);

    let cout = circuit.merge(c1, c2).label("cout", &mut circuit);

    circuit.unify(cout, cout_o);
    circuit.unify(s2, s_o);

    return circuit;
}

fn main() {
    let ha = half_adder_circuit();
    let fa = full_adder_circuit(&ha);
    let design = design![&ha, &fa];
    let testbench = Testbench::new(&fa)
        .cycles(11)
        .signals(["cin", "b", "a"], 0..8, 0.5)
        .constant("clk", 1, 0.0)
        .observe(["cout", "s"])
        .period_ps(100.0);

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("logical") => design.print(LogicalVerilog),
        Some("spice") => design.print(RsfqlibSpice),
        Some("verilog") => design.print(RsfqlibVerilog),
        Some("logical-testbench") => testbench.print(LogicalVerilogTestbench),
        Some("spice-testbench") => testbench.print(RsfqlibSpiceTestbench),
        Some("verilog-testbench") => testbench.print(RsfqlibVerilogTestbench),
        _ => {
            println!(
                "Usage: cargo run [logical|spice|verilog|logical-testbench|spice-testbench|verilog-testbench]"
            );
        }
    }
}

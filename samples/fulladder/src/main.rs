use rust_sfq::*;
use std::env;

fn half_adder_circuit() -> Circuit<3, 0, 2, 0> {
    let inputs = ["a", "b", "clk"];
    let outputs = ["c", "s"];
    let (mut circuit, [a, b, clk], [], [c_o, s_o], []) =
        Circuit::create(inputs, [], outputs, [], "HalfAdder");

    let (clk1, clk2) = circuit.split(clk % 0); // automatically assign name
    let (a1, a2) = circuit.split(a % 0);
    let (b1, b2) = circuit.split(b % 0);

    let c = circuit.and_labeled(a1 % 1, b1 % 1, clk1 % 0, "c"); // explicitly assign name
    let s = circuit.xor_labeled(a2 % 1, b2 % 1, clk2 % 0, "s");

    circuit.unify(c, c_o);
    circuit.unify(s, s_o);

    return circuit;
}

fn full_adder_circuit(ha: &Circuit<3, 0, 2, 0>) -> Circuit<4, 0, 2, 0> {
    let inputs = ["a", "b", "cin", "clk"];
    let outputs = ["cout", "s"];
    let (mut circuit, [a, b, cin, clk], [], [cout_o, s_o], []) =
        Circuit::create(inputs, [], outputs, [], "FullAdder");

    let (clk, clk1) = circuit.split(clk % 0);
    let (clk, clk2) = circuit.split(clk % 0);
    let ([c1, s1], []) = circuit.subcircuit(ha, [a % 0, b % 0, clk1 % 0], []);
    let cin = circuit.dff(cin % 1, clk2 % 0);

    let (clk3, clk4) = circuit.split(clk % 0);

    let c1 = circuit.buff(c1 % 0); // delay adjustment
    let c1 = circuit.buff(c1 % 0); // Not needed for logical simulation.
    let c1 = circuit.dff(c1 % 1, clk3 % 0);

    let s1 = circuit.buff(s1 % 0); // delay adjustment
    let s1 = circuit.buff(s1 % 0);
    let cin = circuit.buff(cin % 0);
    let cin = circuit.buff(cin % 0);
    let ([c2, s2], []) = circuit.subcircuit(ha, [s1 % 0, cin % 0, clk4 % 0], []);

    let cout = circuit.merge_labeled(c1 % 0, c2 % 0, "cout");

    circuit.unify(cout, cout_o);
    circuit.unify(s2, s_o);

    return circuit;
}

fn main() {
    let ha = half_adder_circuit();
    let fa = full_adder_circuit(&ha);

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("logical") => {
            println!("{}", LogicalVerilog::generate(&ha));
            println!("{}", LogicalVerilog::generate(&fa));
        }
        Some("spice") => {
            println!("{}", RsfqlibSpice::generate(&ha));
            println!("{}", RsfqlibSpice::generate(&fa));
        }
        Some("verilog") => {
            println!("{}", RsfqlibVerilog::generate(&ha));
            println!("{}", RsfqlibVerilog::generate(&fa));
        }
        _ => {
            println!("Usage: cargo run [logical|spice|verilog]");
        }
    }
}

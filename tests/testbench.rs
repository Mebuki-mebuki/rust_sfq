use rust_sfq::*;

fn and_circuit() -> Circuit<3, 0, 1, 0> {
    let (mut circuit, [a, b, clk], [], [q_out], []) =
        Circuit::create(["a", "b", "clk"], [], ["q"], [], "AndForTestbench");

    let q = circuit.and_p(a, b, clk);
    circuit.unify(q, q_out);
    circuit
}

#[test]
fn logical_testbench_expands_msb_first_signals_and_pads_with_zero() {
    let circuit = and_circuit();
    let testbench = Testbench::new(&circuit)
        .cycles(5)
        .signals(["a", "b"], [0, 1, 2, 3], 0.5)
        .constant("clk", 1, 0.0)
        .observe(["q"])
        .period_ps(100.0);

    let verilog = testbench.generate(LogicalVerilogTestbench);

    assert!(verilog.contains("$dumpfile(\"AndForTestbench.vcd\");"));
    assert!(verilog.contains("    a = 0;\n    b = 0;\n    clk = 1;"));
    assert!(verilog.contains("    a <= 0;\n    b <= 1;\n    clk <= 1;"));
    assert!(verilog.contains("    a <= 1;\n    b <= 0;\n    clk <= 1;"));
    assert!(verilog.contains("    a <= 1;\n    b <= 1;\n    clk <= 1;"));
    assert!(verilog.contains("    a <= 0;\n    b <= 0;\n    clk <= 1;"));
}

#[test]
fn spice_testbench_observes_inputs_plus_requested_signals() {
    let circuit = and_circuit();
    let testbench = Testbench::new(&circuit)
        .cycles(3)
        .pulse("a", [1], 0.25)
        .toggle("b", [1], 0.5)
        .constant("clk", 1, 0.0)
        .observe(["q", "internal.XTOP"])
        .period_ps(100.0);

    let spice = testbench.generate(RsfqlibSpiceTestbench);

    assert!(spice.contains("XTOP a b clk q AndForTestbench"));
    assert!(spice.contains(".print v(a)"));
    assert!(spice.contains(".print v(b)"));
    assert!(spice.contains(".print v(clk)"));
    assert!(spice.contains(".print v(q)"));
    assert!(spice.contains(".print v(internal.XTOP)"));
    assert!(spice.contains("V0      a       0       pwl(0 0 125p 0 127.5p 827.13u 130p 0 )"));
    assert!(spice.contains("V1      b       0       pwl(0 0 150p 0 152.5p 827.13u 155p 0 250p 0 252.5p 827.13u 255p 0 )"));
}

#[test]
#[should_panic(expected = "signals value 4 does not fit in 2 bits")]
fn signals_rejects_values_that_do_not_fit_width() {
    let circuit = and_circuit();
    let _ = Testbench::new(&circuit)
        .cycles(1)
        .signals(["a", "b"], [4], 0.5)
        .constant("clk", 1, 0.0)
        .generate(LogicalVerilogTestbench);
}

#[test]
#[should_panic(expected = "input port `clk` has no testbench stimulus")]
fn generation_requires_all_input_ports() {
    let circuit = and_circuit();
    let _ = Testbench::new(&circuit)
        .cycles(1)
        .signals(["a", "b"], [0], 0.5)
        .generate(LogicalVerilogTestbench);
}

#[test]
#[should_panic(expected = "testbench period_ps must be specified before generation")]
fn generation_requires_period_ps() {
    let circuit = and_circuit();
    let _ = Testbench::new(&circuit)
        .cycles(1)
        .signals(["a", "b"], [0], 0.5)
        .constant("clk", 1, 0.0)
        .generate(LogicalVerilogTestbench);
}

#[test]
#[should_panic(expected = "testbench stimulus for `a` is specified more than once")]
fn duplicate_stimulus_is_rejected() {
    let circuit = and_circuit();
    let _ = Testbench::new(&circuit)
        .cycles(1)
        .signal("a", [0], 0.5)
        .pulse("a", [0], 0.5)
        .signal("b", [0], 0.5)
        .constant("clk", 1, 0.0)
        .generate(LogicalVerilogTestbench);
}

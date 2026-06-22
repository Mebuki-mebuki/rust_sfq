use rust_sfq::*;

// 後段のシミュレータが依存する LogicalVerilog の出力形を固定するテスト群。
// delay 付き wire は __cycle で駆動される reg として表現し、パイプライン用の
// order 指定は入力側 reg として実現する。また、不正または重複した Verilog 宣言を
// 出力しないことも確認する。

#[test]
fn add_delay_generates_register_chain() {
    // 複数サイクルの delay は reg チェーンとして生成される。
    // 各段の代入元は直前の段であり、reg 宣言文字列が混ざってはいけない。
    let (mut circuit, [a], [], [q_out], []) = Circuit::create(["a"], [], ["q"], [], "DelayChain");

    circuit.add_delay(&a, 2);
    let q = circuit.buff(a);
    circuit.unify(q, q_out);

    let verilog = design![&circuit].generate(LogicalVerilog);

    assert!(verilog.contains("reg a_d1 = 1'b0, a_d2 = 1'b0;"));
    assert!(verilog.contains("rustsfq_buff BUFF1 (a_d2, q, __cycle);"));
    assert!(verilog.contains("always @(posedge __cycle) begin"));
    assert!(verilog.contains("a_d1 <= a;"));
    assert!(verilog.contains("a_d2 <= a_d1;"));
}

#[test]
fn pipelined_gate_order_generates_input_registers() {
    // *_p ヘルパは「データ入力が clk より 1 サイクル後」という指定を表す。
    // LogicalVerilog では、その局所 order を入力側 reg として実現する。
    let (mut circuit, [a, b, clk], [], [q_out], []) =
        Circuit::create(["a", "b", "clk"], [], ["q"], [], "PipelineAnd");

    let q = circuit.and_p(a, b, clk).label("q", &mut circuit);
    circuit.unify(q, q_out);

    let verilog = design![&circuit].generate(LogicalVerilog);

    assert!(verilog.contains("reg a_d1 = 1'b0, b_d1 = 1'b0;"));
    assert!(verilog.contains("a_d1 <= a;"));
    assert!(verilog.contains("b_d1 <= b;"));
    assert!(verilog.contains("rustsfq_and AND1 (a_d1, b_d1, clk, q, __cycle);"));
}

#[test]
fn ports_are_not_redeclared_as_internal_wires() {
    // port は input/output として宣言済みなので、内部 wire として再宣言しない。
    let (mut circuit, [a], [], [q_out], []) = Circuit::create(["a"], [], ["q"], [], "PortOnly");

    let q = circuit.buff(a);
    circuit.unify(q, q_out);

    let verilog = design![&circuit].generate(LogicalVerilog);

    assert!(verilog.contains("module PortOnly (a, q, __cycle);"));
    assert!(verilog.contains("input a, __cycle;"));
    assert!(verilog.contains("output q;"));
    assert!(!verilog.contains("\nwire "));
}

#[test]
fn terminate_does_not_emit_a_gate_instance() {
    // terminate は API 上 wire を消費するための操作であり、生成 Verilog に
    // インスタンス化すべき logical primitive は存在しない。
    let (mut circuit, [a], [], [], []) = Circuit::create(["a"], [], [], [], "TerminateOnly");

    circuit.terminate(a);

    let verilog = design![&circuit].generate(LogicalVerilog);

    assert!(verilog.contains("module TerminateOnly (a, __cycle);"));
    assert!(!verilog.contains("rustsfq_terminate"));
    assert!(verilog.ends_with("endmodule"));
}

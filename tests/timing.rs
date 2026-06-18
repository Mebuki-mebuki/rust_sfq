use rust_sfq::*;

// タイミングチェッカの基本契約を固定するテスト群。
// delay が 0 の wire は同一サイクル内の前後関係を作り、add_delay() は
// サイクル境界としてその関係を切る。サブ回路は親回路より先に検査され、
// 境界ポート間のタイミング制約が親回路のグラフへ展開される必要がある。

fn passthrough_circuit() -> Circuit<1, 0, 1, 0> {
    let (mut circuit, [a], [], [q_out], []) = Circuit::create(["a"], [], ["q"], [], "Pass");
    let q = circuit.jtl(a);
    circuit.unify(q, q_out);
    circuit
}

#[test]
fn clocked_gate_with_explicit_order_generates() {
    // クロック付きゲートでは入力ごとの局所的な order 指定を受け取る。
    // clk より後の order を持つ入力は LogicalVerilog でパイプライン reg になる。
    let (mut circuit, [a, b, clk], [], [q_out], []) =
        Circuit::create(["a", "b", "clk"], [], ["q"], [], "AndOrdered");

    let q = circuit.and(a % 1, b % 1, clk % 0).label("q", &mut circuit);
    circuit.unify(q, q_out);

    let verilog = design![&circuit].generate(LogicalVerilog);

    assert!(verilog.contains("module AndOrdered (a, b, clk, q, __cycle);"));
    assert!(verilog.contains("rustsfq_and AND1 (a_d1, b_d1, clk, q, __cycle);"));
}

#[test]
#[should_panic(expected = "Timing constraints are unsatisfiable")]
fn feedback_loop_without_delay_is_rejected() {
    // 同一サイクル内の組み合わせフィードバックはタイミング制約を満たせない。
    let (mut circuit, [], [], [], []) = Circuit::create([], [], [], [], "LoopNoDelay");
    let (loop_in, loop_out) = circuit.gen_loop("loop");
    let q = circuit.jtl(loop_in);

    circuit.unify(q, loop_out);

    let _ = design![&circuit].generate(LogicalVerilog);
}

#[test]
fn feedback_loop_with_delay_is_accepted() {
    // 同じフィードバックでも、add_delay() がタイミンググラフをサイクル境界で
    // 切れば有効な回路として扱える。
    let (mut circuit, [], [], [], []) = Circuit::create([], [], [], [], "LoopWithDelay");
    let (loop_in, loop_out) = circuit.gen_loop("loop");
    let q = circuit.jtl(loop_in);

    circuit.add_delay(&q, 1);
    circuit.unify(q, loop_out);

    let verilog = design![&circuit].generate(LogicalVerilog);

    assert!(verilog.contains("reg loop_d1 = 1'b0;"));
    assert!(verilog.contains("loop_d1 <= loop;"));
    assert!(verilog.contains("rustsfq_jtl JTL1 (loop_d1, loop, __cycle);"));
}

#[test]
#[should_panic(expected = "Timing information for subcircuit `Pass` is missing")]
fn subcircuit_requires_prior_timing_information() {
    // Design の生成順序には意味がある。親回路は、先に Design へ追加済みの
    // サブ回路についてだけタイミング要約を利用できる。
    let subcircuit = passthrough_circuit();
    let (mut circuit, [a], [], [q_out], []) =
        Circuit::create(["a"], [], ["q"], [], "ParentMissingChild");

    let ([q], []) = circuit.subcircuit(&subcircuit, [a], []);
    circuit.unify(q, q_out);

    let _ = design![&circuit].generate(LogicalVerilog);
}

#[test]
#[should_panic(expected = "Timing constraints are unsatisfiable")]
fn subcircuit_timing_edges_are_propagated_to_parent() {
    // 子回路の input -> output 制約は親回路からも見える必要がある。
    // これが伝播しないと、このループが誤って timing OK になってしまう。
    let subcircuit = passthrough_circuit();
    let (mut circuit, [], [], [], []) = Circuit::create([], [], [], [], "ParentLoop");
    let (loop_in, loop_out) = circuit.gen_loop("loop");

    let ([q], []) = circuit.subcircuit(&subcircuit, [loop_in], []);
    circuit.unify(q, loop_out);

    let _ = design![&subcircuit, &circuit].generate(LogicalVerilog);
}

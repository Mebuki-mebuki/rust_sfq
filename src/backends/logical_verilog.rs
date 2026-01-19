use std::collections::{BTreeSet, HashMap};

use super::Backend;
use crate::circuit::Circuit;
use crate::gate::Gate;
use crate::id::WireID;

pub struct LogicalVerilog;

// ワイヤ名から遅延付きワイヤ名を生成する
fn delayed_name(name: &str, delay: usize) -> String {
    format!("{}_d{}", name, delay)
}

fn gate_string<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
    c: &Circuit<N_I, N_CI, N_O, N_CO>,
    m: &HashMap<&WireID, String>, // delayed_wire_names
    name: &str,
    inputs: Vec<&WireID>,
    outputs: Vec<&WireID>,
    gate: &str,
) -> String {
    let mut ports = Vec::new();
    // 入力は遅延つき
    ports.extend(
        inputs
            .iter()
            .map(|id| m[&c.resolved_wire_id(**id)].as_str()),
    );
    // 出力は遅延なし
    ports.extend(outputs.iter().map(|id| c.get_resolved_wire_name(**id)));
    // イベント
    ports.push("__event");
    format!("rustsfq_{} {} ({});", gate, name, ports.join(", "))
}

impl Backend for LogicalVerilog {
    fn generate<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        c: &Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> String {
        let mut res = Vec::new();

        /* ------------------- header ------------------- */
        let mut in_ports: Vec<&str> = c.in_ports();
        in_ports.push("__event");
        let out_ports: Vec<&str> = c.out_ports();
        let mut ports: Vec<&str> = c.all_ports();
        ports.push("__event");
        res.push(format!("module {} ({});", c.name(), ports.join(", ")));
        if in_ports.len() > 0 {
            res.push(format!("input {};", in_ports.join(", ")));
        }
        if out_ports.len() > 0 {
            res.push(format!("output {};", out_ports.join(", ")));
        }

        let wires: Vec<&str> = c
            .all_wire_names()
            .into_iter()
            .filter(|s| !ports.contains(s)) // ポートのwireは除外
            .collect::<BTreeSet<&str>>() // 重複(ないはず)削除, ソート
            .into_iter()
            .collect();
        if wires.len() > 0 {
            res.push(format!("wire {};", wires.join(", ")));
        }

        // delay の分だけ reg を追加し, 最後の段を参照する HashMap を作成.
        // 後でブロッキング代入で接続する.
        let mut regs = BTreeSet::new();
        let mut delayed_wire_names = HashMap::new();
        let mut assignments = Vec::new();

        for (id, info) in c.wires().iter() {
            let delay = info.delay;

            if delay == 0 {
                delayed_wire_names.insert(id, info.name.clone());
                continue;
            }
            let mut prev_name = &info.name;
            for d in 1..=delay {
                let dname = delayed_name(&info.name, d);
                assignments.push(format!("{} <= {};", dname, prev_name));

                // 配線用 reg を初期値 0 で宣言
                regs.insert(format!("{} = 1'b0", dname));
                prev_name = regs.last().unwrap();
            }
            delayed_wire_names.insert(id, delayed_name(&info.name, delay));
        }
        if regs.len() > 0 {
            res.push(format!(
                "reg {};",
                regs.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }

        /* ------------------- body ------------------- */
        let m = &delayed_wire_names;
        for gate in c.gates().iter() {
            let s = match gate {
                Gate::Jtl { name, a, q } => gate_string(c, m, name, vec![a], vec![q], "JTL"),
                Gate::Split { name, a, q1, q2 } => {
                    gate_string(c, m, name, vec![a], vec![q1, q2], "split")
                }
                Gate::Merge { name, a, b, q } => {
                    gate_string(c, m, name, vec![a, b], vec![q], "merge")
                }
                Gate::And { name, a, b, clk, q } => {
                    gate_string(c, m, name, vec![a, b, clk], vec![q], "and")
                }
                Gate::Or { name, a, b, clk, q } => {
                    gate_string(c, m, name, vec![a, b, clk], vec![q], "or")
                }
                Gate::Xor { name, a, b, clk, q } => {
                    gate_string(c, m, name, vec![a, b, clk], vec![q], "xor")
                }
                Gate::Xnor { name, a, b, clk, q } => {
                    gate_string(c, m, name, vec![a, b, clk], vec![q], "xnor")
                }
                Gate::Not { name, a, clk, q } => {
                    gate_string(c, m, name, vec![a, clk], vec![q], "not")
                }
                Gate::Dff { name, a, clk, q } => {
                    gate_string(c, m, name, vec![a, clk], vec![q], "dff")
                }
                Gate::Ndro { name, a, b, clk, q } => {
                    gate_string(c, m, name, vec![a, b, clk], vec![q], "ndro")
                }
                Gate::Buff { name, a, q } => gate_string(c, m, name, vec![a], vec![q], "BUFF"),
                Gate::ZeroAsync { name, q } => {
                    gate_string(c, m, name, vec![], vec![q], "zero_async")
                }
                Gate::Terminate { name: _, a: _ } => String::new(),
                Gate::Subcircuit {
                    name,
                    inputs,
                    outputs,
                    circuit,
                } => {
                    let mut ports = Vec::new();
                    // 入力は遅延つき
                    ports.extend(inputs.iter().map(|id| m[&c.resolved_wire_id(*id)].as_str()));
                    // 出力は遅延なし
                    ports.extend(outputs.iter().map(|id| c.get_resolved_wire_name(*id)));
                    // イベント
                    ports.push("__event");
                    format!("{} {} ({});", circuit, name, ports.join(", "))
                }
                _ => panic!("Unsupported Gate"),
            };
            res.push(s);
        }

        if assignments.len() > 0 {
            res.push("always @(posedge __event) begin".to_string());
            res.push(assignments.join(" "));
            res.push("end".to_string());
        }

        /* ------------------- footer ------------------- */
        res.push("endmodule".to_string());

        return res.join("\n");
    }
}

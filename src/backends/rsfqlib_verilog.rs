use std::collections::BTreeSet;
use std::sync::Once;

use super::Backend;
use crate::circuit::Circuit;
use crate::gate::Gate;

pub struct RsfqlibVerilog;

// Timing 未対応警告
static WARN_ONCE: Once = Once::new();
fn warn_timing_ignored() {
    WARN_ONCE.call_once(|| {
        eprintln!("Warning: timing information is currently not supported and will be ignored.");
    });
}

//  (circuit, gate名, 引数WireIDリスト, ゲート名)
macro_rules! gate_string {
    ($c:ident, $name:ident, [$($arg:expr),*], $gate:expr) => {
        format!("THmitll_{}_v3p0_extracted {} ({});", $gate, $name,
            vec![ $($c.get_wire_name($arg), )*].join(", ")
        )
    };
}

impl Backend for RsfqlibVerilog {
    fn generate<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        c: &Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> String {
        warn_timing_ignored();
        let mut res = Vec::new();

        /* ------------------- header ------------------- */
        let in_ports: Vec<&str> = c.in_ports();
        let out_ports: Vec<&str> = c.out_ports();
        let ports: Vec<&str> = c.all_ports();
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

        /* ------------------- body ------------------- */
        for gate in c.gates().iter() {
            let s = match &gate.value {
                Gate::Jtl { name, a, q } => gate_string!(c, name, [*a, *q], "JTL"),
                Gate::Split { name, a, q1, q2 } => gate_string!(c, name, [*a, *q1, *q2], "SPLIT"),
                Gate::Merge { name, a, b, q } => gate_string!(c, name, [*a, *b, *q], "MERGE"),
                Gate::And {
                    name, a, b, clk, q, ..
                } => gate_string!(c, name, [a.id, b.id, clk.id, *q], "AND2"),
                Gate::Or {
                    name, a, b, clk, q, ..
                } => gate_string!(c, name, [a.id, b.id, clk.id, *q], "OR2"),
                Gate::Xor {
                    name, a, b, clk, q, ..
                } => gate_string!(c, name, [a.id, b.id, clk.id, *q], "XOR"),
                Gate::Xnor {
                    name, a, b, clk, q, ..
                } => gate_string!(c, name, [a.id, b.id, clk.id, *q], "XNOR"),
                Gate::Not {
                    name, a, clk, q, ..
                } => gate_string!(c, name, [a.id, clk.id, *q], "NOT"),
                Gate::Dff {
                    name, a, clk, q, ..
                } => gate_string!(c, name, [a.id, clk.id, *q], "DFF"),
                Gate::Ndro {
                    name, a, b, clk, q, ..
                } => gate_string!(c, name, [a.id, b.id, clk.id, *q], "NDRO"),
                Gate::Buff { name, a, q } => gate_string!(c, name, [*a, *q], "BUFF"),
                Gate::ZeroAsync { name, q } => format!(
                    "THmitll_ALWAYS0_ASYNC_NOA {} ({});",
                    name,
                    c.get_wire_name(*q)
                ),
                Gate::Terminate { name: _, a: _ } => String::new(),
                Gate::Subcircuit {
                    name,
                    inputs,
                    outputs,
                    circuit,
                } => {
                    let ports: Vec<&str> = inputs
                        .iter()
                        .chain(outputs.iter())
                        .map(|wid| c.get_wire_name(*wid))
                        .collect();
                    format!("{} {} ({});", circuit, name, ports.join(", "))
                }
                _ => panic!("Unsupported Gate"),
            };
            res.push(s);
        }

        /* ------------------- footer ------------------- */
        res.push("endmodule".to_string());

        return res.join("\n");
    }
}

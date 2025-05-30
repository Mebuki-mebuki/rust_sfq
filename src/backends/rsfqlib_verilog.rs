use std::collections::BTreeSet;

use super::Backend;
use crate::circuit::Circuit;
use crate::gate::Gate;

pub struct RsfqlibVerilog;

//  (circuit, gate名, 引数WireIDリスト, ゲート名)
macro_rules! gate_string {
    ($c:ident, $name:ident, [$($arg:ident),*], $gate:expr) => {
        format!("THmitll_{}_v3p0_extracted {} ({});", $gate, $name,
            vec![ $($c.wire_names.get($arg).unwrap().as_str(), )*].join(", ")
        )
    };
}

impl Backend for RsfqlibVerilog {
    fn generate<const N_I: usize, const N_CO: usize, const N_O: usize, const N_CI: usize>(
        c: &Circuit<N_I, N_CO, N_O, N_CI>,
    ) -> String {
        let mut res = Vec::new();

        /* ------------------- header ------------------- */
        let in_ports: Vec<&str> = c
            .inputs
            .iter()
            .chain(c.counter_inputs.iter())
            .map(|s| s.as_str())
            .collect();
        let out_ports: Vec<&str> = c
            .outputs
            .iter()
            .chain(c.counter_outputs.iter())
            .map(|s| s.as_str())
            .collect();
        let ports: Vec<&str> = [in_ports.clone(), out_ports.clone()].concat();
        res.push(format!("module {} ({})", c.name, ports.join(", ")));
        if in_ports.len() > 0 {
            res.push(format!("input {};", in_ports.join(", ")));
        }
        if c.outputs.len() > 0 {
            res.push(format!("output {};", out_ports.join(", ")));
        }

        let wires: Vec<&str> = c
            .wire_names
            .values()
            .map(|s| s.as_str())
            .filter(|s| !ports.contains(s)) // ポートのwireは除外
            .collect::<BTreeSet<&str>>() // 重複削除, ソート
            .into_iter()
            .collect();
        if wires.len() > 0 {
            res.push(format!("wire {};", wires.join(", ")));
        }

        /* ------------------- body ------------------- */
        for gate in c.gates.iter() {
            let s = match gate {
                Gate::Jtl { name, a, q } => gate_string!(c, name, [a, q], "JTL"),
                Gate::Split { name, a, q1, q2 } => gate_string!(c, name, [a, q1, q2], "SPLIT"),
                Gate::Merge { name, a, b, q } => gate_string!(c, name, [a, b, q], "MERGE"),
                Gate::And { name, a, b, clk, q } => gate_string!(c, name, [a, b, clk, q], "AND"),
                Gate::Or { name, a, b, clk, q } => gate_string!(c, name, [a, b, clk, q], "OR"),
                Gate::Xor { name, a, b, clk, q } => gate_string!(c, name, [a, b, clk, q], "XOR"),
                Gate::Xnor { name, a, b, clk, q } => gate_string!(c, name, [a, b, clk, q], "XNOR"),
                Gate::Not { name, a, clk, q } => gate_string!(c, name, [a, clk, q], "NOT"),
                Gate::Dff { name, a, clk, q } => gate_string!(c, name, [a, clk, q], "DFF"),
                Gate::Ndro { name, a, b, clk, q } => gate_string!(c, name, [a, b, clk, q], "NDRO"),
                Gate::Buff { name, a, q } => gate_string!(c, name, [a, q], "BUFF"),
                Gate::ZeroAsync { name, q } => format!(
                    "THmitll_ALWAYS0_ASYNC_NOA {} ({});",
                    name,
                    c.wire_names.get(q).unwrap()
                ),
                _ => panic!("Unsupported Gate"),
            };
            res.push(s);
        }

        /* ------------------- footer ------------------- */
        res.push("endmodule".to_string());

        return res.join("\n");
    }
}

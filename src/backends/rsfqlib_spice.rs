use super::Backend;
use crate::circuit::Circuit;
use crate::gate::Gate;

pub struct RsfqlibSpice;

//  (circuit, gate名, 引数WireIDリスト, ゲート名)
macro_rules! gate_string {
    ($c:ident, $name:ident, [$($arg:ident),*],$gate:expr) => {
        vec![$name,
            $($c.wire_names.get($arg).unwrap(), )*
            concat!("THmitll_", $gate)
        ].join(" ")
    };
}

impl Backend for RsfqlibSpice {
    fn generate<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        c: &Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> String {
        let mut res = Vec::new();

        /* ------------------- header ------------------- */
        res.push(format!(
            ".subckt {} {}",
            c.name,
            c.inputs
                .iter()
                .chain(c.counter_outputs.iter())
                .chain(c.outputs.iter())
                .chain(c.counter_inputs.iter())
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(" "),
        ));

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
                Gate::ZeroAsync { name, q } => gate_string!(c, name, [q], "ALWAYS0_ASYNC_NOA"),
                _ => panic!("Unsupported Gate"),
            };
            res.push(s);
        }

        /* ------------------- footer ------------------- */
        res.push(".ends".to_string());

        return res.join("\n");
    }
}

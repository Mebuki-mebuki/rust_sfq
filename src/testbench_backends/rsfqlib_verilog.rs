use super::{format_ps, physical_event_time_ps, port_declarations};
use crate::testbench::{Testbench, TestbenchBackend};

pub struct RsfqlibVerilogTestbench;

impl TestbenchBackend for RsfqlibVerilogTestbench {
    fn generate(&self, testbench: &Testbench<'_>) -> String {
        let tb = testbench.normalize();
        let (inputs, outputs) = port_declarations(&tb);
        let mut events = Vec::new();
        let mut res = Vec::new();

        for signal in &tb.signals {
            for (cycle, value) in signal.values.iter().enumerate() {
                if *value == 1 {
                    let start = physical_event_time_ps(cycle, signal, tb.period_ps);
                    events.push((start, signal.name.clone()));
                }
            }
        }
        events.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

        res.push("`timescale 1ps / 100fs".to_string());
        res.push("`include \"all.v\"".to_string());
        res.push("`include \"modules.v\"".to_string());
        res.push(String::new());
        res.push("module top;".to_string());
        res.push("  // Inputs".to_string());
        if !inputs.is_empty() {
            res.push(format!("  reg {};", inputs.join(", ")));
        }
        res.push("  // Outputs".to_string());
        if !outputs.is_empty() {
            res.push(format!("  wire {};", outputs.join(", ")));
        }
        res.push(String::new());
        res.push(format!("  {} dut (.*);", tb.circuit.name()));
        res.push(String::new());
        res.push("  // Input pulses".to_string());
        res.push("  initial begin".to_string());
        for signal in &tb.signals {
            res.push(format!("    {} = 0;", signal.name));
        }

        let mut current_time = 0.0;
        for (time, name) in events {
            let delay = time - current_time;
            if delay > 0.0 {
                res.push(format!("    #{};", format_ps(delay)));
            }
            res.push(format!("    {} ^= 1;", name));
            current_time = time;
        }

        let finish_time = (tb.cycles as f64 + 1.0) * tb.period_ps;
        let delay = finish_time - current_time;
        if delay > 0.0 {
            res.push(format!("    #{};", format_ps(delay)));
        }
        res.push("    $finish;".to_string());
        res.push("  end".to_string());
        res.push(String::new());
        res.push("  initial begin".to_string());
        res.push(format!("    $dumpfile(\"{}.vcd\");", tb.name));
        res.push("    $dumpvars(0, top);".to_string());
        res.push("  end".to_string());
        res.push("endmodule".to_string());

        res.join("\n")
    }
}

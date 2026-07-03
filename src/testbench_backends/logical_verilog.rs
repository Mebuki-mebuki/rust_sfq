use super::port_declarations;
use crate::testbench::{Testbench, TestbenchBackend};

pub struct LogicalVerilogTestbench;

impl TestbenchBackend for LogicalVerilogTestbench {
    fn generate(&self, testbench: &Testbench<'_>) -> String {
        let tb = testbench.normalize();
        let (inputs, outputs) = port_declarations(&tb);
        let mut res = Vec::new();

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
        res.push("  reg __cycle;".to_string());
        res.push("  integer step;".to_string());
        res.push(String::new());
        res.push(format!("  {} dut (.*);", tb.circuit.name()));
        res.push(String::new());
        res.push("  // Simulation Steps".to_string());
        res.push("  initial begin".to_string());
        res.push("    __cycle = 0;".to_string());
        res.push("    step = 0;".to_string());
        res.push("    forever begin".to_string());
        res.push("      #50;".to_string());
        res.push("      __cycle = 0;".to_string());
        res.push("      #50;".to_string());
        res.push("      __cycle = 1;".to_string());
        res.push("      step = step + 1;".to_string());
        res.push("    end".to_string());
        res.push("  end".to_string());
        res.push(String::new());
        res.push("  // Input patterns".to_string());
        res.push("  initial begin".to_string());
        for signal in &tb.signals {
            res.push(format!("    {} = {};", signal.name, signal.values[0]));
        }
        for cycle in 1..tb.cycles {
            res.push("    @(posedge __cycle);".to_string());
            for signal in &tb.signals {
                res.push(format!("    {} <= {};", signal.name, signal.values[cycle]));
            }
        }
        res.push("    @(posedge __cycle);".to_string());
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

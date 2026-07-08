mod logical_verilog;
mod rsfqlib_spice;
mod rsfqlib_verilog;

pub use logical_verilog::LogicalVerilogTestbench;
pub use rsfqlib_spice::RsfqlibSpiceTestbench;
pub use rsfqlib_verilog::RsfqlibVerilogTestbench;

use crate::testbench::{NormalizedTestbench, SignalPattern};

fn format_ps(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        let formatted = format!("{:.6}", value);
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn physical_event_time_ps(cycle: usize, signal: &SignalPattern, period_ps: f64) -> f64 {
    if signal.phase == 0.0 {
        // Keep a phase-0 pulse away from simulator startup time.
        (cycle as f64 + 1.0) * period_ps
    } else {
        (cycle as f64 + signal.phase) * period_ps
    }
}

fn port_declarations<'a>(tb: &'a NormalizedTestbench<'_>) -> (Vec<&'a str>, Vec<&'a str>) {
    (tb.circuit.in_ports(), tb.circuit.out_ports())
}

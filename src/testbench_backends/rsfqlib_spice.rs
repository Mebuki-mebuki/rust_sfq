use super::{format_ps, physical_event_time_ps};
use crate::testbench::{SignalPattern, Testbench, TestbenchBackend};

pub struct RsfqlibSpiceTestbench;

const PULSE_HEIGHT: &str = "827.13u";
const PULSE_RISE_PS: f64 = 2.5;
const PULSE_WIDTH_PS: f64 = 5.0;
const TRAN_STEP_PS: f64 = 0.25;
const LOAD_OHMS: &str = "2";

impl TestbenchBackend for RsfqlibSpiceTestbench {
    fn generate(&self, testbench: &Testbench<'_>) -> String {
        let tb = testbench.normalize();
        let mut res = Vec::new();

        res.push(".include ../../../lib/rsfqlib/spice/all.cir".to_string());
        res.push(".include modules.cir".to_string());
        res.push(String::new());

        let ports = tb.circuit.all_ports();
        res.push(format!("XTOP {} {}", ports.join(" "), tb.circuit.name()));
        res.push(String::new());

        for (index, output) in tb.circuit.out_ports().iter().enumerate() {
            res.push(format!(
                "R{}      {}    0       {}",
                index, output, LOAD_OHMS
            ));
        }

        for (index, signal) in tb.signals.iter().enumerate() {
            res.push(pwl_source(index, signal, tb.period_ps));
        }
        res.push(String::new());

        let stop_ps = (tb.cycles as f64 + 1.0) * tb.period_ps;
        res.push(format!(
            ".tran {}p {}p 0 {}p",
            format_ps(TRAN_STEP_PS),
            format_ps(stop_ps),
            format_ps(TRAN_STEP_PS)
        ));
        for signal in &tb.observe {
            res.push(format!(".print v({})", signal));
        }
        res.push(".end".to_string());

        res.join("\n")
    }
}

fn pwl_source(index: usize, signal: &SignalPattern, period_ps: f64) -> String {
    let mut points = vec!["0 0".to_string()];
    for (cycle, value) in signal.values.iter().enumerate() {
        if *value == 1 {
            let start = physical_event_time_ps(cycle, signal, period_ps);
            let peak = start + PULSE_RISE_PS;
            let end = start + PULSE_WIDTH_PS;
            points.push(format!("{}p 0", format_ps(start)));
            points.push(format!("{}p {}", format_ps(peak), PULSE_HEIGHT));
            points.push(format!("{}p 0", format_ps(end)));
        }
    }
    format!(
        "V{}      {}       0       pwl({} )",
        index,
        signal.name,
        points.join(" ")
    )
}

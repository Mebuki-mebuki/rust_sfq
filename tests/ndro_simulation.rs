//! Run with: cargo test --test ndro_simulation -- --ignored
//! These tests require iverilog and vvp on PATH.

use rust_sfq::*;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct SimulationDir(PathBuf);

impl SimulationDir {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("rust-sfq-ndro-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn run(&self, source: &str) {
        std::fs::write(self.0.join("test.sv"), source).unwrap();
        let library = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib/logical");
        let compile = Command::new("iverilog")
            .current_dir(&self.0)
            .args(["-g2012", "-s", "top", "-I"])
            .arg(library)
            .args(["-o", "test.vvp", "test.sv"])
            .output()
            .expect("install Icarus Verilog to run NDRO simulation tests");
        assert!(
            compile.status.success(),
            "{}",
            String::from_utf8_lossy(&compile.stderr)
        );
        let result = Command::new("vvp")
            .current_dir(&self.0)
            .arg("test.vvp")
            .output()
            .expect("vvp must be available to run NDRO simulation tests");
        assert!(
            result.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(String::from_utf8_lossy(&result.stdout).contains("NDRO PASS"));
    }
}

impl Drop for SimulationDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Case {
    order: [usize; 3],
    delay: [usize; 3],
    state: Option<bool>,
}

impl Case {
    // Independent oracle: deliver explicitly delayed pulses in physical order,
    // reading the state when clk arrives. No clock-aligned input registers or
    // SET_LAST priority calculation are used here.
    fn output(&mut self, inputs: &[[bool; 3]], cycle: usize) -> Option<bool> {
        let arriving: [bool; 3] = std::array::from_fn(|port| {
            cycle
                .checked_sub(self.delay[port])
                .is_some_and(|source| inputs[source][port])
        });
        let mut events = [0, 1, 2];
        // Preserve the existing convention: data tied with clk precedes it.
        events.sort_by_key(|&port| (self.order[port], port == 2));
        let mut output = Some(false);
        for port in events {
            if !arriving[port] {
                continue;
            }
            if port == 2 {
                output = self.state;
            } else if self.order[0] == self.order[1] && arriving[0] && arriving[1] {
                self.state = None;
            } else {
                self.state = Some(port == 0);
            }
        }
        output
    }
}

#[test]
#[ignore = "requires iverilog and vvp; see lib/logical/README.md"]
fn generated_ndro_matches_event_reference() {
    let orders = [
        [0, 1, 2],
        [1, 0, 2],
        [1, 2, 0],
        [2, 1, 0],
        [0, 2, 1],
        [2, 0, 1],
        [0, 0, 1],
        [1, 1, 0],
        [0, 0, 0],
        [0, 1, 0],
        [1, 0, 0],
        [1, 0, 1],
        [0, 1, 1],
    ];
    let mut cases = Vec::new();
    for order in orders {
        for delay in [[0, 0, 0], [1, 0, 0], [0, 2, 0], [1, 2, 1]] {
            cases.push(Case {
                order,
                delay,
                state: Some(false),
            });
        }
    }

    // Test startup, then all 8^4 four-cycle input sequences from each state.
    // Repeated single-input pulses establish the state through every delay
    // configuration using only public ports. Read-only tails flush delayed
    // events and verify that readout is non-destructive.
    let mut inputs = vec![[false, false, true]; 4];
    for initial in [false, true] {
        for sequence in 0..4096 {
            inputs.extend([[initial, !initial, false]; 4]);
            inputs.extend([[false, false, false]; 4]);
            for cycle in 0..4 {
                let bits = (sequence >> (3 * cycle)) & 7;
                inputs.push([bits & 4 != 0, bits & 2 != 0, bits & 1 != 0]);
            }
            inputs.extend([[false, false, true]; 4]);
        }
    }

    let count = cases.len();
    let cycles = inputs.len();
    let mut source = String::from("`include \"ndro.sv\"\n");
    for (index, case) in cases.iter().enumerate() {
        let name = format!("NdroCase{index}");
        let (mut circuit, [a, b, clk], [], [q_out], []) =
            Circuit::create(["a", "b", "clk"], [], ["q"], [], &name);
        for (wire, delay) in [&a, &b, &clk].into_iter().zip(case.delay) {
            circuit.add_delay(wire, delay);
        }
        let q = circuit.ndro(a % case.order[0], b % case.order[1], clk % case.order[2]);
        circuit.unify(q, q_out);
        writeln!(source, "{}", design![&circuit].generate(LogicalVerilog)).unwrap();
    }

    let mut vectors = String::with_capacity(cycles * (count + 4));
    for (cycle, input) in inputs.iter().enumerate() {
        for &bit in input {
            vectors.push(if bit { '1' } else { '0' });
        }
        // q[index] is packed least-significant bit first.
        for case in cases.iter_mut().rev() {
            vectors.push(match case.output(&inputs, cycle) {
                Some(false) => '0',
                Some(true) => '1',
                None => 'x',
            });
        }
        vectors.push('\n');
    }
    writeln!(
        source,
        "module top;
reg a=0, b=0, clk=0, __cycle=0;
wire [{}:0] q;
reg [{}:0] vectors [0:{}];
integer cycle, index;",
        count - 1,
        count + 2,
        cycles - 1
    )
    .unwrap();
    for index in 0..count {
        writeln!(
            source,
            "NdroCase{index} dut{index}(a, b, clk, q[{index}], __cycle);"
        )
        .unwrap();
    }
    writeln!(source, "initial begin
  $readmemb(\"vectors.txt\", vectors);
  for (cycle=0; cycle<{cycles}; cycle=cycle+1) begin
    {{a, b, clk}} = vectors[cycle][{}:{}];
    #1;
    for (index=0; index<{count}; index=index+1)
      if (q[index] !== vectors[cycle][index])
        $fatal(1, \"case=%0d cycle=%0d expected=%b actual=%b\", index, cycle, vectors[cycle][index], q[index]);
    __cycle=1;
    #1;
    __cycle=0;
  end
  $display(\"NDRO PASS: {count} configurations, {cycles} cycles\");
  $finish;
end
endmodule", count + 2, count).unwrap();

    let directory = SimulationDir::new();
    std::fs::write(directory.0.join("vectors.txt"), vectors).unwrap();
    directory.run(&source);
}

#[test]
#[ignore = "requires iverilog and vvp; see lib/logical/README.md"]
fn default_ndro_propagates_unknowns_and_recovers() {
    // Also exercise the default parameter for existing hand-written instances,
    // and X/Z input patterns omitted from the binary-input exhaustive test.
    SimulationDir::new().run(
        r#"
`include "ndro.sv"
module top;
reg a=0, b=0, clk=0, __cycle=0;
wire q;
rustsfq_ndro dut(a,b,clk,q,__cycle);
task step(input logic sa, sb, sc, expected);
begin
  {a,b,clk}={sa,sb,sc};
  #1;
  if (q !== expected)
    $fatal(1, "inputs=%b expected=%b actual=%b", {a,b,clk}, expected, q);
  __cycle=1; #1; __cycle=0;
end
endtask
initial begin
  step(0,0,1,0);
  step(1,1,0,0); // Ambiguous update even without readout.
  step(0,0,1,1'bx);
  step(1,0,0,0); // Recover by setting.
  step(0,0,1,1);
  step(1'bx,0,1,1'bx);
  step(0,0,1,1'bx); // Unknown input must not latch the previous next value.
  step(0,1,1,0); // Recover by resetting.
  step(0,1'bz,0,0);
  step(0,0,1,1'bx);
  step(1,0,1,1);
  step(0,0,1'bx,1'bx);
  step(0,0,1,1); // Unknown readout does not modify storage.
  $display("NDRO PASS");
  $finish;
end
endmodule
"#,
    );
}

use colored::Colorize;
use std::collections::HashMap;
use twox_hash::XxHash32;

use crate::gate::Gate;
use crate::id::{CircuitID, WireID};
use crate::wire::{CounterWire, TimedWire, Wire, WireInfo, WireKey};

pub struct Circuit<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize> {
    id: CircuitID,
    name: String,
    inputs: [String; N_I],
    counter_inputs: [String; N_CI],
    outputs: [String; N_O],
    counter_outputs: [String; N_CO],

    wires: HashMap<WireID, WireInfo>,
    gates: Vec<Gate>,
    aliases: HashMap<WireID, WireID>,

    next_wire_id: u32,
    next_gate_id: u32,
}

impl<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>
    Circuit<N_I, N_CI, N_O, N_CO>
{
    /* 検証用の関数群 */
    fn is_explicit_name(&self, name: &str) -> bool {
        !name.starts_with("_")
    }
    fn assert_wire_name_exists(&self, name: &str) {
        assert!(
            self.wires.values().all(|v| v.name != name),
            "{}",
            format!("Wire `{}` is already exist!", name).red()
        );
    }
    fn assert_circuit_id(&self, cid: CircuitID) {
        assert!(cid == self.id);
    }
    fn assert_conflict_name(&self, old_name: &str, name: &str, src: &str) {
        assert!(
            old_name == name || !self.is_explicit_name(old_name) || !self.is_explicit_name(name),
            "{}",
            format!("Conflict names in {}: `{}`, `{}`!", src, old_name, name).red()
        );
    }
    fn assert_explicit_name(&self, name: &str) {
        assert!(
            self.is_explicit_name(name),
            "{}",
            format!("Wire `{}` must not start with underscore!", name).red()
        );
    }
}

// 1出力ゲート関数定義用マクロ (関数名, Enumバリアント名, 引数Wireリスト)
macro_rules! define_gate_fn {
    ($fn_name:ident, $fn_name_labeled:ident, $variant:ident, [$($arg:ident),*]) => {
        pub fn $fn_name(&mut self, $($arg:TimedWire),*) -> Wire {
            // 入力 Wire のチェック, receive
            $(
                let $arg = self.process_input($arg);
            )*
            // ゲート名, 出力 Wire の生成, drive
            let gate_name = format!("{}{}", stringify!($fn_name).to_uppercase(), self.generate_gate_id());
            let q_name = format!("_{}_q", gate_name);
            let q_key = self.generate_wire(q_name);

            // ゲートの作成, 追加
            let gate = Gate::$variant {
                name: gate_name,
                $( $arg, )*
                q: q_key.id,
            };
            self.gates.push(gate);

            return Wire(q_key);
        }

        pub fn $fn_name_labeled (&mut self, $($arg: TimedWire,)* label: &str) -> Wire {
            let wire = self.$fn_name($($arg),*);
            self.label(&wire, label);
            return wire;
        }
    };
}

impl<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>
    Circuit<N_I, N_CI, N_O, N_CO>
{
    // 新しい回路を作成するのに使う関数
    pub fn create(
        inputs: [&str; N_I],
        counter_inputs: [&str; N_CI],
        outputs: [&str; N_O],
        counter_outputs: [&str; N_CO],
        name: &str,
    ) -> (
        Self,
        [Wire; N_I],
        [CounterWire; N_CI],
        [CounterWire; N_O],
        [Wire; N_CO],
    ) {
        // 固定のシードでハッシュ化
        let cid: u32 = XxHash32::oneshot(0, name.as_bytes());
        let mut circuit = Self {
            id: CircuitID(cid),
            name: name.to_string(),
            inputs: inputs.map(|s| s.to_string()),
            counter_inputs: counter_inputs.map(|s| s.to_string()),
            outputs: outputs.map(|s| s.to_string()),
            counter_outputs: counter_outputs.map(|s| s.to_string()),
            wires: HashMap::new(),
            gates: Vec::new(),
            aliases: HashMap::new(),
            next_wire_id: 1,
            next_gate_id: 1,
        };

        // 入出力に対応する WireInfo, WireKey 生成
        let input_wires = inputs.map(|s| Wire(circuit.generate_wire(s.to_string())));
        let counter_input_wires =
            counter_inputs.map(|s| CounterWire(circuit.generate_wire(s.to_string())));
        let output_wires = outputs.map(|s| CounterWire(circuit.generate_wire(s.to_string())));
        let counter_output_wires =
            counter_outputs.map(|s| Wire(circuit.generate_wire(s.to_string())));

        return (
            circuit,
            input_wires,
            counter_input_wires,
            output_wires,
            counter_output_wires,
        );
    }

    // 新しい WireInfo を wires に追加し、対応する WireKey を返す
    fn generate_wire(&mut self, name: String) -> WireKey {
        self.assert_wire_name_exists(&name);

        let wid = WireID(self.next_wire_id);
        self.next_wire_id += 1;

        let key = WireKey::new(wid, self.id);
        let info = WireInfo::new(name);
        self.wires.insert(key.id, info);

        return key;
    }

    fn generate_gate_id(&mut self) -> u32 {
        let res = self.next_gate_id;
        self.next_gate_id += 1;
        return res;
    }

    // unify で生じるエイリアスを解決する
    fn resolved_wire_id(&self, id: WireID) -> WireID {
        let mut current_id = id;
        while let Some(next_id) = self.aliases.get(&current_id) {
            current_id = *next_id;
        }
        current_id
    }

    // circuit.label(&wire, "hoge") でラベル付け
    fn label_common(&mut self, id: WireID, name: &str) {
        let old_name = &self.wires.get(&id).unwrap().name;
        self.assert_explicit_name(name);
        self.assert_conflict_name(old_name, name, "label");
        self.wires
            .entry(id)
            .and_modify(|info| info.name = name.to_string());
    }

    pub fn label(&mut self, wire: &Wire, name: &str) {
        self.assert_circuit_id(wire.0.cid);
        self.label_common(wire.0.id, name);
    }

    pub fn clabel(&mut self, cwire: &CounterWire, name: &str) {
        self.assert_circuit_id(cwire.0.cid);
        self.label_common(cwire.0.id, name);
    }

    //-------------------- Gate Functions ----------------------//

    // 入力チェック, consume, delay 設定
    pub fn process_input(&mut self, a: TimedWire) -> WireID {
        let a_delay = a.1;
        let mut a_key = a.0.0;

        self.assert_circuit_id(a_key.cid);
        a_key.consume();
        self.wires
            .entry(a_key.id)
            .and_modify(|info| info.delay = a_delay);

        return a_key.id;
    }
    // CounterWire は delay = 0 で固定
    pub fn process_counter_input(&mut self, a: CounterWire) -> WireID {
        let mut a_key = a.0;

        self.assert_circuit_id(a_key.cid);
        a_key.consume();

        return a_key.id;
    }

    define_gate_fn!(jtl, jtl_labeled, Jtl, [a]);
    define_gate_fn!(merge, merge_labeled, Merge, [a, b]);
    define_gate_fn!(and, and_labeled, And, [a, b, clk]);
    define_gate_fn!(or, or_labeled, Or, [a, b, clk]);
    define_gate_fn!(xor, xor_labeled, Xor, [a, b, clk]);
    define_gate_fn!(not, not_labeled, Not, [a, clk]);
    define_gate_fn!(xnor, xnor_labeled, Xnor, [a, b, clk]);
    define_gate_fn!(dff, dff_labeled, Dff, [a, clk]);
    define_gate_fn!(ndro, ndro_labeled, Ndro, [a, b, clk]);
    define_gate_fn!(buff, buff_labeled, Buff, [a]);
    define_gate_fn!(zero_async, zero_async_labeled, ZeroAsync, []);

    pub fn split(&mut self, a: TimedWire) -> (Wire, Wire) {
        let a_id = self.process_input(a);

        // ゲート名, 出力 Wire の生成
        let gate_name = format!("SPLIT{}", self.generate_gate_id());
        let q1_name = format!("_{}_q1", gate_name);
        let q2_name = format!("_{}_q2", gate_name);
        let q1_key = self.generate_wire(q1_name);
        let q2_key = self.generate_wire(q2_name);

        // ゲートの作成, 追加
        let gate = Gate::Split {
            name: gate_name,
            a: a_id,
            q1: q1_key.id,
            q2: q2_key.id,
        };
        self.gates.push(gate);

        return (Wire(q1_key), Wire(q2_key));
    }

    pub fn split_labeled(&mut self, a: TimedWire, label1: &str, label2: &str) -> (Wire, Wire) {
        let (wire1, wire2) = self.split(a);
        self.label(&wire1, label1);
        self.label(&wire2, label2);
        return (wire1, wire2);
    }

    pub fn terminate(&mut self, a: TimedWire) {
        let a_id = self.process_input(a);

        let gate_name = format!("TERMINATE{}", self.generate_gate_id());
        let gate = Gate::Terminate {
            name: gate_name,
            a: a_id,
        };
        self.gates.push(gate);
    }

    /* Gates for CounterWire */
    pub fn cbuff(&mut self, q: CounterWire) -> CounterWire {
        let q_id = self.process_counter_input(q);
        // ゲート名, 出力 CounterWire の生成, receive
        let gate_name = format!("BUFF{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let a_key = self.generate_wire(a_name);

        // ゲートの作成, 追加
        let gate = Gate::Buff {
            name: gate_name,
            a: a_key.id,
            q: q_id,
        };
        self.gates.push(gate);

        return CounterWire(a_key);
    }

    pub fn cbuff_labeled(&mut self, q: CounterWire, label: &str) -> CounterWire {
        let cwire = self.cbuff(q);
        self.clabel(&cwire, label);
        return cwire;
    }

    // q1(CounterWire)を受けとりq2(Wire)とa(CounterWire)を返す
    pub fn csplit(&mut self, q1: CounterWire) -> (Wire, CounterWire) {
        let q1_id = self.process_counter_input(q1);

        let gate_name = format!("SPLIT{}", self.generate_gate_id());
        let q2_name = format!("_{}_q2", gate_name);
        let a_name = format!("_{}_a", gate_name);
        let q2_key = self.generate_wire(q2_name);
        let a_key = self.generate_wire(a_name);

        let gate = Gate::Split {
            name: gate_name,
            a: a_key.id,
            q1: q1_id,
            q2: q2_key.id,
        };
        self.gates.push(gate);

        return (Wire(q2_key), CounterWire(a_key));
    }

    pub fn csplit_labeled(
        &mut self,
        q1: CounterWire,
        label_q2: &str,
        label_a: &str,
    ) -> (Wire, CounterWire) {
        let (q2, a) = self.csplit(q1);
        self.label(&q2, label_q2);
        self.clabel(&a, label_a);
        return (q2, a);
    }

    // q1, q2(CounterWire)を受け取りa(CounterWire)を返す
    pub fn csplit2(&mut self, q1: CounterWire, q2: CounterWire) -> CounterWire {
        let q1_id = self.process_counter_input(q1);
        let q2_id = self.process_counter_input(q2);

        let gate_name = format!("SPLIT{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let a_key = self.generate_wire(a_name);

        // ゲートの作成, 追加
        let gate = Gate::Split {
            name: gate_name,
            a: a_key.id,
            q1: q1_id,
            q2: q2_id,
        };
        self.gates.push(gate);

        return CounterWire(a_key);
    }

    pub fn csplit2_labeled(
        &mut self,
        q1: CounterWire,
        q2: CounterWire,
        label: &str,
    ) -> CounterWire {
        let cwire = self.csplit2(q1, q2);
        self.clabel(&cwire, label);
        return cwire;
    }

    pub fn cterminate(&mut self) -> CounterWire {
        let gate_name = format!("TERMINATE{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let a_key = self.generate_wire(a_name);

        let gate = Gate::Terminate {
            name: gate_name,
            a: a_key.id,
        };
        self.gates.push(gate);

        return CounterWire(a_key);
    }

    pub fn cterminate_labeled(&mut self, label: &str) -> CounterWire {
        let cwire = self.cterminate();
        self.clabel(&cwire, label);
        return cwire;
    }

    pub fn subcircuit<const M_I: usize, const M_CI: usize, const M_O: usize, const M_CO: usize>(
        &mut self,
        circuit: &Circuit<M_I, M_CI, M_O, M_CO>,
        inputs: [TimedWire; M_I],
        counter_inputs: [CounterWire; M_CI],
    ) -> ([Wire; M_O], [CounterWire; M_CO]) {
        // 入力Wireの処理
        let input_ids: Vec<WireID> = inputs.map(|tw| self.process_input(tw)).to_vec();
        let counter_input_ids: Vec<WireID> = counter_inputs
            .map(|cw| self.process_counter_input(cw))
            .to_vec();

        let gate_name = format!("{}{}", circuit.name, self.generate_gate_id());

        // 出力Wireの生成
        let output_wires: [Wire; M_O] = circuit.outputs.clone().map(|s| {
            let wire_name = format!("_{}_{}", gate_name, s);
            let wire_key = self.generate_wire(wire_name);
            Wire(wire_key)
        });
        let counter_output_wires: [CounterWire; M_CO] = circuit.counter_outputs.clone().map(|s| {
            let wire_name = format!("_{}_{}", gate_name, s);
            let wire_key = self.generate_wire(wire_name);
            CounterWire(wire_key)
        });

        // ゲートの生成
        let mut gate_inputs: Vec<WireID> = input_ids;
        gate_inputs.extend(counter_output_wires.iter().map(|cw| cw.0.id));

        let mut gate_outputs: Vec<WireID> = output_wires.iter().map(|w| w.0.id).collect();
        gate_outputs.extend(counter_input_ids);

        let gate = Gate::Subcircuit {
            name: gate_name,
            inputs: gate_inputs,
            outputs: gate_outputs,
            circuit: circuit.name.clone(),
        };
        self.gates.push(gate);

        return (output_wires, counter_output_wires);
    }

    //-------------------- Wire Functions ----------------------//

    // 同一のidを持ったWireとCounterWireを生成する
    pub fn gen_loop(&mut self, name: &str) -> (Wire, CounterWire) {
        self.assert_explicit_name(name);
        let key = self.generate_wire(name.to_string());
        let wire = Wire(key.clone());
        let cwire = CounterWire(key);
        return (wire, cwire);
    }

    // Wire と CounterWire を統合
    pub fn unify(&mut self, wire: Wire, cwire: CounterWire) {
        let mut key = wire.0;
        let mut ckey = cwire.0;

        self.assert_circuit_id(key.cid);
        self.assert_circuit_id(ckey.cid);

        // wireがcounter wireをdrive
        key.consume();
        ckey.consume();

        let info1 = self.wires.get(&key.id).unwrap();
        let info2 = self.wires.get(&ckey.id).unwrap();

        // 名前の解決
        let name1 = &info1.name;
        let name2 = &info2.name;
        self.assert_conflict_name(name1, name2, "unify");
        let name = if self.is_explicit_name(name1) {
            name1
        } else {
            name2
        };

        // delay は receive 時に決まるので, already received の CounterWire の方を使用
        let delay = info2.delay;

        // id の小さい方を代表にする
        let (prime_id, sub_id) = if key.id.0 < ckey.id.0 {
            (key.id, ckey.id)
        } else {
            (ckey.id, key.id)
        };
        let new_info = WireInfo {
            name: name.to_string(),
            delay,
        };

        // 情報の更新
        self.aliases.insert(sub_id, prime_id);
        self.wires.insert(prime_id, new_info);
        self.wires.remove(&sub_id);
    }

    // unifyの便利関数
    pub fn unify_array<const N: usize>(&mut self, wires: [Wire; N], cwires: [CounterWire; N]) {
        for (wire, cwire) in wires.into_iter().zip(cwires) {
            self.unify(wire, cwire);
        }
    }
}

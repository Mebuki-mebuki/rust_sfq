use std::collections::HashMap;
use twox_hash::XxHash32;

use crate::gate::Gate;
use crate::id::{CircuitID, WireID};
use crate::wire::{CounterWire, HasWireID, Wire};

pub struct Circuit<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize> {
    pub(crate) name: String,
    id: CircuitID,
    pub(crate) inputs: [String; N_I],
    pub(crate) counter_inputs: [String; N_CI],
    pub(crate) outputs: [String; N_O],
    pub(crate) counter_outputs: [String; N_CO],

    pub(crate) wire_names: HashMap<WireID, String>,
    pub(crate) gates: Vec<Gate>,

    next_wire_id: u32,
    next_gate_id: u32,
}

// 1出力ゲート関数定義用マクロ (関数名, Enumバリアント名, 引数Wireリスト)
macro_rules! define_gate_fn {
    ($fn_name:ident, $fn_name_labeled:ident, $variant:ident, [$($arg:ident),*]) => {
        pub fn $fn_name(&mut self, $(mut $arg: Wire),*) -> Wire {
            // 入力 Wire のチェック, receive
            $(
                assert!($arg.circuit_id() == self.id);
                $arg.receive();
            )*
            // ゲート名, 出力 Wire の生成, drive
            let gate_name = format!("X{}{}", stringify!($fn_name).to_uppercase(), self.generate_gate_id());
            let q_name = format!("_{}_q", gate_name);
            let mut q = self.generate_wire(q_name);
            q.drive();
            // ゲートの作成, 追加
            let gate = Gate::$variant {
                name: gate_name,
                $( $arg: $arg.wire_id(), )*
                q: q.wire_id(),
            };
            self.gates.push(gate);

            return q;
        }

        pub fn $fn_name_labeled (&mut self, $($arg: Wire,)* label: &str) -> Wire {
            let wire = self.$fn_name($($arg),*);
            self.label(&wire, label);
            return wire;
        }
    };
}

impl<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>
    Circuit<N_I, N_CI, N_O, N_CO>
{
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
            name: name.to_string(),
            id: CircuitID(cid),
            inputs: inputs.map(|s| s.to_string()),
            counter_inputs: counter_inputs.map(|s| s.to_string()),
            outputs: outputs.map(|s| s.to_string()),
            counter_outputs: counter_outputs.map(|s| s.to_string()),
            wire_names: HashMap::new(),
            gates: Vec::new(),
            next_wire_id: 1,
            next_gate_id: 1,
        };

        // 入出力に対応する Wire 生成
        let mut input_wires = inputs.map(|s| circuit.generate_wire(s.to_string()));
        let mut counter_input_wires =
            counter_inputs.map(|s| circuit.generate_counter_wire(s.to_string()));
        let mut output_wires = outputs.map(|s| circuit.generate_counter_wire(s.to_string()));
        let mut counter_output_wires =
            counter_outputs.map(|s| circuit.generate_wire(s.to_string()));

        // 初期条件の drive, receive
        input_wires.iter_mut().for_each(|w| w.drive());
        counter_input_wires.iter_mut().for_each(|w| w.receive());
        output_wires.iter_mut().for_each(|w| w.receive());
        counter_output_wires.iter_mut().for_each(|w| w.drive());

        return (
            circuit,
            input_wires,
            counter_input_wires,
            output_wires,
            counter_output_wires,
        );
    }

    fn generate_wire(&mut self, name: String) -> Wire {
        let wid = WireID(self.next_wire_id);
        self.next_wire_id += 1;
        assert!(
            self.wire_names.values().all(|v| v != &name),
            "\"{}\" is already exist!",
            name
        );
        self.wire_names.insert(wid, name);
        return Wire::new(wid, self.id);
    }

    fn generate_counter_wire(&mut self, name: String) -> CounterWire {
        let wid = WireID(self.next_wire_id);
        self.next_wire_id += 1;
        assert!(
            self.wire_names.values().all(|v| v != &name),
            "\"{}\" is already exist!",
            name
        );
        self.wire_names.insert(wid, name);
        return CounterWire::new(wid, self.id);
    }

    fn generate_gate_id(&mut self) -> u32 {
        let res = self.next_gate_id;
        self.next_gate_id += 1;
        return res;
    }

    // circuit.label(&wire, "hoge") でラベル付け
    #[allow(private_bounds)]
    pub fn label<T>(&mut self, wire: &T, label: &str)
    where
        T: HasWireID,
    {
        assert!(wire.circuit_id() == self.id);
        assert!(
            self.wire_names
                .get(&wire.wire_id())
                .unwrap()
                .starts_with("_")
        );
        assert!(!label.starts_with("_"));
        self.wire_names.insert(wire.wire_id(), label.to_string());
    }

    //-------------------- Gate Functions ----------------------//

    define_gate_fn!(jtl, jtl_labeld, Jtl, [a]);
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

    pub fn split(&mut self, mut a: Wire) -> (Wire, Wire) {
        // 入力 Wire のチェック, receive
        assert!(a.circuit_id() == self.id);
        a.receive();
        // ゲート名, 出力 Wire の生成, drive
        let gate_name = format!("XSPLIT{}", self.generate_gate_id());
        let q1_name = format!("_{}_q1", gate_name);
        let q2_name = format!("_{}_q2", gate_name);
        let mut q1 = self.generate_wire(q1_name);
        let mut q2 = self.generate_wire(q2_name);
        q1.drive();
        q2.drive();
        // ゲートの作成, 追加
        let gate = Gate::Split {
            name: gate_name,
            a: a.wire_id(),
            q1: q1.wire_id(),
            q2: q2.wire_id(),
        };
        self.gates.push(gate);

        return (q1, q2);
    }

    pub fn split_labeld(&mut self, a: Wire, label1: &str, label2: &str) -> (Wire, Wire) {
        let (wire1, wire2) = self.split(a);
        self.label(&wire1, label1);
        self.label(&wire2, label2);
        return (wire1, wire2);
    }

    // Gate for CounterWire
    pub fn cbuff(&mut self, mut q: CounterWire) -> CounterWire {
        assert!(q.circuit_id() == self.id);
        q.drive(); // CounterWire を drive
        // ゲート名, 出力 CounterWire の生成, receive
        let gate_name = format!("XBUFF{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let mut a = self.generate_counter_wire(a_name);
        a.receive();
        // ゲートの作成, 追加
        let gate = Gate::Buff {
            name: gate_name,
            a: a.wire_id(),
            q: q.wire_id(),
        };
        self.gates.push(gate);

        return a;
    }

    pub fn cbuff_labeled(&mut self, q: CounterWire, label: &str) -> CounterWire {
        let cwire = self.cbuff(q);
        self.label(&cwire, label);
        return cwire;
    }

    // q1(CounterWire)を受けとりq2(Wire)とa(CounterWire)を返す
    pub fn csplit(&mut self, mut q1: CounterWire) -> (Wire, CounterWire) {
        assert!(q1.circuit_id() == self.id);
        q1.drive();

        let gate_name = format!("XSPLIT{}", self.generate_gate_id());
        let q2_name = format!("_{}_q2", gate_name);
        let a_name = format!("_{}_a", gate_name);
        let mut q2 = self.generate_wire(q2_name);
        let mut a = self.generate_counter_wire(a_name);
        q2.drive();
        a.receive();

        let gate = Gate::Split {
            name: gate_name,
            a: a.wire_id(),
            q1: q1.wire_id(),
            q2: q2.wire_id(),
        };
        self.gates.push(gate);

        return (q2, a);
    }

    pub fn csplit_labeled(
        &mut self,
        q1: CounterWire,
        label_q2: &str,
        label_a: &str,
    ) -> (Wire, CounterWire) {
        let (q2, a) = self.csplit(q1);
        self.label(&q2, label_q2);
        self.label(&a, label_a);
        return (q2, a);
    }

    // q1, q2(CounterWire)を受け取りa(CounterWire)を返す
    pub fn csplit2(&mut self, mut q1: CounterWire, mut q2: CounterWire) -> CounterWire {
        assert!(q1.circuit_id() == self.id);
        assert!(q2.circuit_id() == self.id);
        q1.drive();
        q2.drive();

        let gate_name = format!("XSPLIT{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let mut a = self.generate_counter_wire(a_name);
        a.receive();
        // ゲートの作成, 追加
        let gate = Gate::Split {
            name: gate_name,
            a: a.wire_id(),
            q1: q1.wire_id(),
            q2: q2.wire_id(),
        };
        self.gates.push(gate);

        return a;
    }

    pub fn csplit2_labeled(
        &mut self,
        q1: CounterWire,
        q2: CounterWire,
        label: &str,
    ) -> CounterWire {
        let cwire = self.csplit2(q1, q2);
        self.label(&cwire, label);
        return cwire;
    }

    pub fn subcircuit<const M_I: usize, const M_CI: usize, const M_O: usize, const M_CO: usize>(
        &mut self,
        circuit: &Circuit<M_I, M_CI, M_O, M_CO>,
        mut inputs: [Wire; M_I],
        mut counter_inputs: [CounterWire; N_CI],
    ) -> ([Wire; M_O], [CounterWire; M_CO]) {
        // 入力Wireの処理
        assert!(inputs.iter().all(|w| w.circuit_id() == self.id));
        assert!(counter_inputs.iter().all(|w| w.circuit_id() == self.id));
        inputs.iter_mut().for_each(|w| w.receive());
        counter_inputs.iter_mut().for_each(|cw| cw.drive());

        let gate_name = format!("X{}{}", circuit.name, self.generate_gate_id());

        // 出力Wireの生成
        let output_wires = circuit.outputs.clone().map(|s| {
            let wire_name = format!("_{}_{}", gate_name, s);
            let mut wire = self.generate_wire(wire_name);
            wire.drive();
            wire
        });
        let counter_output_wires = circuit.counter_outputs.clone().map(|s| {
            let wire_name = format!("_{}_{}", gate_name, s);
            let mut cwire = self.generate_counter_wire(wire_name);
            cwire.receive();
            cwire
        });

        // ゲートの生成
        let gate_inputs: Vec<WireID> = inputs
            .iter()
            .map(|w| w.wire_id())
            .chain(counter_output_wires.iter().map(|cw| cw.wire_id()))
            .collect();
        let gate_outputs: Vec<WireID> = output_wires
            .iter()
            .map(|w| w.wire_id())
            .chain(counter_inputs.iter().map(|cw| cw.wire_id()))
            .collect();
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
    pub fn gen_loop(&mut self, label: &str) -> (Wire, CounterWire) {
        let mut wire = self.generate_wire(label.to_string());
        let mut cwire = CounterWire::new(wire.wire_id(), self.id);
        wire.drive();
        cwire.receive();
        return (wire, cwire);
    }

    // Wire と CounterWire を統合
    pub fn unify(&mut self, mut wire: Wire, mut cwire: CounterWire) {
        assert!(wire.circuit_id() == self.id);
        assert!(cwire.circuit_id() == self.id);

        // wireがcounter wireをdrive
        wire.receive();
        cwire.drive();

        // 名前の解決
        let name1 = self.wire_names.get(&wire.wire_id()).unwrap();
        let name2 = self.wire_names.get(&cwire.wire_id()).unwrap();

        let named1 = !name1.starts_with("_");
        let named2 = !name2.starts_with("_");

        if named1 && named2 {
            assert!(name1 == name2, "conflict names: {}, {}", name1, name2);
            self.wire_names.insert(cwire.wire_id(), name1.clone());
        } else if named2 {
            self.wire_names.insert(wire.wire_id(), name2.clone());
        } else {
            self.wire_names.insert(cwire.wire_id(), name1.clone());
        }
    }
}

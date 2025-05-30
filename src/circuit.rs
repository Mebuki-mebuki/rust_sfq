use std::collections::HashMap;
use twox_hash::XxHash32;

use crate::gate::Gate;
use crate::id::{CircuitID, WireID};
use crate::wire::{CounterWire, HasWireID, Wire};

pub struct Circuit<const N_I: usize, const N_CO: usize, const N_O: usize, const N_CI: usize> {
    pub(crate) name: String,
    id: CircuitID,
    pub(crate) inputs: [String; N_I],
    pub(crate) counter_outputs: [String; N_CO],
    pub(crate) outputs: [String; N_O],
    pub(crate) counter_inputs: [String; N_CI],

    pub(crate) wire_names: HashMap<WireID, String>,
    pub(crate) gates: Vec<Gate>,

    next_wire_id: u32,
    next_gate_id: u32,
}

// 1出力ゲート関数定義用マクロ (関数名, Enumバリアント名, 引数Wireリスト)
macro_rules! define_gate_fn {
    ($fn_name:ident, $variant:ident, [$($arg:ident),*]) => {
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
    };
}

impl<const N_I: usize, const N_CO: usize, const N_O: usize, const N_CI: usize>
    Circuit<N_I, N_CO, N_O, N_CI>
{
    pub fn create(
        inputs: [&str; N_I],
        counter_outputs: [&str; N_CO],
        outputs: [&str; N_O],
        counter_inputs: [&str; N_CI],
        name: &str,
    ) -> (
        Self,
        [Wire; N_I],
        [CounterWire; N_CO],
        [CounterWire; N_O],
        [Wire; N_CI],
    ) {
        // 固定のシードでハッシュ化
        let cid: u32 = XxHash32::oneshot(0, name.as_bytes());
        let mut circuit = Self {
            name: name.to_string(),
            id: CircuitID(cid),
            inputs: inputs.map(|s| s.to_string()),
            counter_outputs: counter_outputs.map(|s| s.to_string()),
            outputs: outputs.map(|s| s.to_string()),
            counter_inputs: counter_inputs.map(|s| s.to_string()),
            wire_names: HashMap::new(),
            gates: Vec::new(),
            next_wire_id: 1,
            next_gate_id: 1,
        };

        // 入出力に対応する Wire 生成
        let mut input_wires = inputs.map(|s| circuit.generate_wire(s.to_string()));
        let mut counter_output_wires =
            counter_outputs.map(|s| circuit.generate_counter_wire(s.to_string()));
        let mut output_wires = outputs.map(|s| circuit.generate_counter_wire(s.to_string()));
        let mut counter_input_wires = counter_inputs.map(|s| circuit.generate_wire(s.to_string()));

        // 初期条件の drive, receive
        input_wires.iter_mut().for_each(|w| w.drive());
        counter_output_wires.iter_mut().for_each(|w| w.receive());
        output_wires.iter_mut().for_each(|w| w.receive());
        counter_input_wires.iter_mut().for_each(|w| w.drive());

        return (
            circuit,
            input_wires,
            counter_output_wires,
            output_wires,
            counter_input_wires,
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

    define_gate_fn!(jtl, Jtl, [a]);
    define_gate_fn!(merge, Merge, [a, b]);
    define_gate_fn!(and, And, [a, b, clk]);
    define_gate_fn!(or, Or, [a, b, clk]);
    define_gate_fn!(xor, Xor, [a, b, clk]);
    define_gate_fn!(not, Not, [a, clk]);
    define_gate_fn!(xnor, Xnor, [a, b, clk]);
    define_gate_fn!(dff, Dff, [a, clk]);
    define_gate_fn!(ndro, Ndro, [a, b, clk]);
    define_gate_fn!(buff, Buff, [a]);
    define_gate_fn!(zero_async, ZeroAsync, []);

    pub fn split(&mut self, mut a: Wire) -> [Wire; 2] {
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

        return [q1, q2];
    }

    // wire % circuit.label("hoge") の構文でラベル付け
    #[allow(private_bounds)]
    pub fn label<T>(&mut self, label: &str) -> impl FnOnce(T) -> T
    where
        T: HasWireID,
    {
        return |wire: T| {
            assert!(wire.circuit_id() == self.id);
            assert!(
                self.wire_names
                    .get(&wire.wire_id())
                    .unwrap()
                    .starts_with("_")
            );
            assert!(!label.starts_with("_"));
            self.wire_names.insert(wire.wire_id(), label.to_string());
            return wire;
        };
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

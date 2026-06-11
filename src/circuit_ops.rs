use crate::circuit::Circuit;
use crate::gate::Gate;
use crate::id::{OrderedWireID, WireID};
use crate::location::SourceLocation;
use crate::wire::{CounterWire, OrderedWire, Wire, WireInfo};

// 1出力クロック付きゲート関数定義用マクロ (関数名, Enumバリアント名, 引数Wireリスト)
macro_rules! define_clocked_gate_fn {
    ($fn_name:ident, $variant:ident, [$($arg:ident),*]) => {
        #[track_caller]
        pub fn $fn_name(&mut self, $($arg:OrderedWire),*) -> Wire {
            let location = SourceLocation::caller();
            $(
                let $arg = self.process_ordered_input($arg);
            )*

            let gate_name = format!("{}{}", stringify!($fn_name).to_uppercase(), self.generate_gate_id());
            let q_name = format!("_{}_q", gate_name);
            let q_key = self.generate_wire_at(q_name, location);

            let gate = Gate::$variant {
                name: gate_name,
                $( $arg, )*
                q: q_key.id,
            };
            self.push_gate_at(gate, location);

            return Wire(q_key);
        }
    };
}

// 1出力クロックなしゲート関数定義用マクロ (関数名, Enumバリアント名, 引数Wireリスト)
macro_rules! define_clockless_gate_fn {
    ($fn_name:ident, $variant:ident, [$($arg:ident),*]) => {
        #[track_caller]
        pub fn $fn_name(&mut self, $($arg:Wire),*) -> Wire {
            let location = SourceLocation::caller();
            $(
                let $arg = self.process_input($arg);
            )*

            let gate_name = format!("{}{}", stringify!($fn_name).to_uppercase(), self.generate_gate_id());
            let q_name = format!("_{}_q", gate_name);
            let q_key = self.generate_wire_at(q_name, location);

            let gate = Gate::$variant {
                name: gate_name,
                $( $arg, )*
                q: q_key.id,
            };
            self.push_gate_at(gate, location);

            return Wire(q_key);
        }
    };
}

// パイプライン化ゲート関数定義用マクロ (パイプライン関数名, 元の関数名, 引数Wireリスト(clkは除く))
macro_rules! define_pipelined_gate_fn {
    ($fn_name_p:ident, $fn_name:ident, [$($arg:ident),*]) => {
        #[track_caller]
        pub fn $fn_name_p(&mut self, $($arg:Wire),*, clk: Wire) -> Wire {
            self.$fn_name($($arg % 1),*, clk % 0)
        }
    };
}

impl<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>
    Circuit<N_I, N_CI, N_O, N_CO>
{
    fn label_common(&mut self, id: WireID, name: &str, location: SourceLocation) {
        let old_wire = self.wires.get(&id).unwrap();
        let old_name = old_wire.value.name.clone();
        let old_location = old_wire.location;
        self.assert_explicit_name(name, location);
        self.assert_conflict_name(&old_name, old_location, name, location, "label");
        self.wires
            .entry(id)
            .and_modify(|info| info.value.name = name.to_string());
    }

    #[track_caller]
    pub fn label(&mut self, wire: &Wire, name: &str) {
        let location = SourceLocation::caller();
        self.assert_circuit_id(wire.0.cid, wire.0.location);
        self.label_common(wire.0.id, name, location);
    }

    #[track_caller]
    pub fn clabel(&mut self, cwire: &CounterWire, name: &str) {
        let location = SourceLocation::caller();
        self.assert_circuit_id(cwire.0.cid, cwire.0.location);
        self.label_common(cwire.0.id, name, location);
    }

    pub fn add_delay(&mut self, wire: &Wire, delay: usize) {
        self.assert_circuit_id(wire.0.cid, wire.0.location);
        self.wires
            .entry(wire.0.id)
            .and_modify(|info| info.value.delay += delay);
    }

    pub(crate) fn process_ordered_input(&mut self, a: OrderedWire) -> OrderedWireID {
        let a_order = a.1;
        let mut a_key = a.0.0;

        self.assert_circuit_id(a_key.cid, a_key.location);
        a_key.consume();
        return OrderedWireID {
            id: a_key.id,
            order: a_order,
        };
    }

    pub(crate) fn process_input(&mut self, a: Wire) -> WireID {
        let mut a_key = a.0;

        self.assert_circuit_id(a_key.cid, a_key.location);
        a_key.consume();

        return a_key.id;
    }

    pub(crate) fn process_counter_input(&mut self, a: CounterWire) -> WireID {
        let mut a_key = a.0;

        self.assert_circuit_id(a_key.cid, a_key.location);
        a_key.consume();

        return a_key.id;
    }

    define_clocked_gate_fn!(and, And, [a, b, clk]);
    define_clocked_gate_fn!(or, Or, [a, b, clk]);
    define_clocked_gate_fn!(xor, Xor, [a, b, clk]);
    define_clocked_gate_fn!(not, Not, [a, clk]);
    define_clocked_gate_fn!(xnor, Xnor, [a, b, clk]);
    define_clocked_gate_fn!(dff, Dff, [a, clk]);
    define_clocked_gate_fn!(ndro, Ndro, [a, b, clk]);

    define_pipelined_gate_fn!(and_p, and, [a, b]);
    define_pipelined_gate_fn!(or_p, or, [a, b]);
    define_pipelined_gate_fn!(xor_p, xor, [a, b]);
    define_pipelined_gate_fn!(not_p, not, [a]);
    define_pipelined_gate_fn!(xnor_p, xnor, [a, b]);
    define_pipelined_gate_fn!(dff_p, dff, [a]);

    define_clockless_gate_fn!(jtl, Jtl, [a]);
    define_clockless_gate_fn!(buff, Buff, [a]);
    define_clockless_gate_fn!(merge, Merge, [a, b]);
    define_clockless_gate_fn!(zero_async, ZeroAsync, []);

    #[track_caller]
    pub fn split(&mut self, a: Wire) -> (Wire, Wire) {
        let location = SourceLocation::caller();
        let a_id = self.process_input(a);

        let gate_name = format!("SPLIT{}", self.generate_gate_id());
        let q1_name = format!("_{}_q1", gate_name);
        let q2_name = format!("_{}_q2", gate_name);
        let q1_key = self.generate_wire_at(q1_name, location);
        let q2_key = self.generate_wire_at(q2_name, location);

        let gate = Gate::Split {
            name: gate_name,
            a: a_id,
            q1: q1_key.id,
            q2: q2_key.id,
        };
        self.push_gate_at(gate, location);

        return (Wire(q1_key), Wire(q2_key));
    }

    #[track_caller]
    pub fn terminate(&mut self, a: Wire) {
        let location = SourceLocation::caller();
        let a_id = self.process_input(a);

        let gate_name = format!("TERMINATE{}", self.generate_gate_id());
        let gate = Gate::Terminate {
            name: gate_name,
            a: a_id,
        };
        self.push_gate_at(gate, location);
    }

    #[track_caller]
    pub fn cbuff(&mut self, q: CounterWire) -> CounterWire {
        let location = SourceLocation::caller();
        let q_id = self.process_counter_input(q);
        let gate_name = format!("BUFF{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let a_key = self.generate_wire_at(a_name, location);

        let gate = Gate::Buff {
            name: gate_name,
            a: a_key.id,
            q: q_id,
        };
        self.push_gate_at(gate, location);

        return CounterWire(a_key);
    }

    #[track_caller]
    pub fn csplit(&mut self, q1: CounterWire) -> (Wire, CounterWire) {
        let location = SourceLocation::caller();
        let q1_id = self.process_counter_input(q1);

        let gate_name = format!("SPLIT{}", self.generate_gate_id());
        let q2_name = format!("_{}_q2", gate_name);
        let a_name = format!("_{}_a", gate_name);
        let q2_key = self.generate_wire_at(q2_name, location);
        let a_key = self.generate_wire_at(a_name, location);

        let gate = Gate::Split {
            name: gate_name,
            a: a_key.id,
            q1: q1_id,
            q2: q2_key.id,
        };
        self.push_gate_at(gate, location);

        return (Wire(q2_key), CounterWire(a_key));
    }

    #[track_caller]
    pub fn csplit2(&mut self, q1: CounterWire, q2: CounterWire) -> CounterWire {
        let location = SourceLocation::caller();
        let q1_id = self.process_counter_input(q1);
        let q2_id = self.process_counter_input(q2);

        let gate_name = format!("SPLIT{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let a_key = self.generate_wire_at(a_name, location);

        let gate = Gate::Split {
            name: gate_name,
            a: a_key.id,
            q1: q1_id,
            q2: q2_id,
        };
        self.push_gate_at(gate, location);

        return CounterWire(a_key);
    }

    #[track_caller]
    pub fn cterminate(&mut self) -> CounterWire {
        let location = SourceLocation::caller();
        let gate_name = format!("TERMINATE{}", self.generate_gate_id());
        let a_name = format!("_{}_a", gate_name);
        let a_key = self.generate_wire_at(a_name, location);

        let gate = Gate::Terminate {
            name: gate_name,
            a: a_key.id,
        };
        self.push_gate_at(gate, location);

        return CounterWire(a_key);
    }

    #[track_caller]
    pub fn subcircuit<const M_I: usize, const M_CI: usize, const M_O: usize, const M_CO: usize>(
        &mut self,
        circuit: &Circuit<M_I, M_CI, M_O, M_CO>,
        inputs: [Wire; M_I],
        counter_inputs: [CounterWire; M_CI],
    ) -> ([Wire; M_O], [CounterWire; M_CO]) {
        let location = SourceLocation::caller();
        let input_ids: Vec<WireID> = inputs.map(|w| self.process_input(w)).to_vec();
        let counter_input_ids: Vec<WireID> = counter_inputs
            .map(|cw| self.process_counter_input(cw))
            .to_vec();

        let gate_name = format!("{}{}", circuit.name, self.generate_gate_id());

        let output_wires: [Wire; M_O] = circuit.outputs.clone().map(|s| {
            let wire_name = format!("_{}_{}", gate_name, s);
            let wire_key = self.generate_wire_at(wire_name, location);
            Wire(wire_key)
        });
        let counter_output_wires: [CounterWire; M_CO] = circuit.counter_outputs.clone().map(|s| {
            let wire_name = format!("_{}_{}", gate_name, s);
            let wire_key = self.generate_wire_at(wire_name, location);
            CounterWire(wire_key)
        });

        let mut gate_inputs: Vec<WireID> = input_ids;
        gate_inputs.extend(counter_output_wires.iter().map(|cw| cw.0.id));

        let mut gate_outputs: Vec<WireID> = output_wires.iter().map(|w| w.0.id).collect();
        gate_outputs.extend(counter_input_ids);

        let gate = Gate::Subcircuit {
            name: gate_name,
            inputs: gate_inputs,
            outputs: gate_outputs,
            circuit: circuit.name.clone(),
            circuit_id: circuit.id,
        };
        self.push_gate_at(gate, location);

        return (output_wires, counter_output_wires);
    }

    #[track_caller]
    pub fn gen_loop(&mut self, name: &str) -> (Wire, CounterWire) {
        let location = SourceLocation::caller();
        self.assert_explicit_name(name, location);
        let key = self.generate_wire_at(name.to_string(), location);
        let wire = Wire(key.clone());
        let cwire = CounterWire(key);
        return (wire, cwire);
    }

    pub fn unify(&mut self, wire: Wire, cwire: CounterWire) {
        let mut key = wire.0;
        let mut ckey = cwire.0;

        self.assert_circuit_id(key.cid, key.location);
        self.assert_circuit_id(ckey.cid, ckey.location);

        key.consume();
        ckey.consume();

        let info1 = self.wires.get(&key.id).unwrap();
        let info2 = self.wires.get(&ckey.id).unwrap();

        let name1 = &info1.value.name;
        let name2 = &info2.value.name;
        self.assert_conflict_name(name1, info1.location, name2, info2.location, "unify");
        let location = if self.is_explicit_name(name1) {
            info1.location
        } else {
            info2.location
        };
        let name = if self.is_explicit_name(name1) {
            name1
        } else {
            name2
        };
        let delay = info1.value.delay + info2.value.delay;

        let (prime_id, sub_id) = if key.id.0 < ckey.id.0 {
            (key.id, ckey.id)
        } else {
            (ckey.id, key.id)
        };
        let new_info = WireInfo {
            name: name.to_string(),
            delay,
        };

        self.replace_wire_id_in_gates(sub_id, prime_id);

        self.wires
            .insert(prime_id, crate::location::Located::new(new_info, location));
        self.wires.remove(&sub_id);
    }

    pub fn unify_array<const N: usize>(&mut self, wires: [Wire; N], cwires: [CounterWire; N]) {
        for (wire, cwire) in wires.into_iter().zip(cwires) {
            self.unify(wire, cwire);
        }
    }
}

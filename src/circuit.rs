use colored::Colorize;
use std::collections::{HashMap, HashSet};

use crate::gate::Gate;
use crate::id::{CircuitID, WireID};
use crate::location::{Located, SourceLocation};
use crate::wire::{CounterWire, Wire, WireInfo, WireKey};

pub struct Circuit<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize> {
    pub(crate) id: CircuitID,
    pub(crate) name: String,
    pub(crate) inputs: [String; N_I],
    pub(crate) counter_inputs: [String; N_CI],
    pub(crate) outputs: [String; N_O],
    pub(crate) counter_outputs: [String; N_CO],

    pub(crate) wires: HashMap<WireID, Located<WireInfo>>,
    pub(crate) gates: Vec<Located<Gate>>,
    pub(crate) wire_to_gates: HashMap<WireID, HashSet<usize>>,

    pub(crate) next_wire_id: u32,
    pub(crate) next_gate_id: u32,
}

impl<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>
    Circuit<N_I, N_CI, N_O, N_CO>
{
    /* 検証用の関数群 */
    pub(crate) fn is_explicit_name(&self, name: &str) -> bool {
        !name.starts_with("_")
    }
    pub(crate) fn assert_wire_name_exists(&self, name: &str, location: SourceLocation) {
        let existing = self.wires.values().find(|v| v.value.name == name);
        assert!(
            existing.is_none(),
            "{}",
            format!(
                "Wire `{}` is already exist! existing defined at {}, new defined at {}",
                name,
                existing.unwrap().location,
                location
            )
            .red()
        );
    }
    pub(crate) fn assert_circuit_id(&self, cid: CircuitID, location: SourceLocation) {
        assert!(
            cid == self.id,
            "{}",
            format!(
                "Wire belongs to a different circuit! wire defined at {}",
                location
            )
            .red()
        );
    }
    pub(crate) fn assert_conflict_name(
        &self,
        old_name: &str,
        old_location: SourceLocation,
        name: &str,
        location: SourceLocation,
        src: &str,
    ) {
        assert!(
            old_name == name || !self.is_explicit_name(old_name) || !self.is_explicit_name(name),
            "{}",
            format!(
                "Conflict names in {}: `{}` defined at {}, `{}` defined at {}!",
                src, old_name, old_location, name, location
            )
            .red()
        );
    }
    pub(crate) fn assert_explicit_name(&self, name: &str, location: SourceLocation) {
        assert!(
            self.is_explicit_name(name),
            "{}",
            format!(
                "Wire `{}` must not start with underscore! defined at {}",
                name, location
            )
            .red()
        );
    }

    pub fn id(&self) -> CircuitID {
        self.id
    }

    pub(crate) fn wire_id_by_name(&self, name: &str) -> WireID {
        self.wires
            .iter()
            .find_map(|(id, info)| (info.value.name == name).then_some(*id))
            .unwrap()
    }

    // Circuit の生成とポート Wire の初期化をまとめて行う.
    #[track_caller]
    pub fn create(
        inputs: [&str; N_I],
        counter_inputs: [&str; N_CI],
        outputs: [&str; N_O],
        counter_outputs: [&str; N_CO],
        name: &str,
    ) -> (
        Circuit<N_I, N_CI, N_O, N_CO>,
        [Wire; N_I],
        [CounterWire; N_CI],
        [CounterWire; N_O],
        [Wire; N_CO],
    ) {
        let location = SourceLocation::caller();
        let mut circuit = Circuit {
            id: CircuitID::from_name(name),
            name: name.to_string(),
            inputs: inputs.map(|s| s.to_string()),
            counter_inputs: counter_inputs.map(|s| s.to_string()),
            outputs: outputs.map(|s| s.to_string()),
            counter_outputs: counter_outputs.map(|s| s.to_string()),
            wires: HashMap::new(),
            gates: Vec::new(),
            wire_to_gates: HashMap::new(),
            next_wire_id: 1,
            next_gate_id: 1,
        };

        let input_wires = inputs.map(|name| {
            circuit.assert_explicit_name(name, location);
            Wire(circuit.generate_wire_at(name.to_string(), location))
        });
        let counter_input_wires = counter_inputs.map(|name| {
            circuit.assert_explicit_name(name, location);
            CounterWire(circuit.generate_wire_at(name.to_string(), location))
        });
        let output_wires = outputs.map(|name| {
            circuit.assert_explicit_name(name, location);
            CounterWire(circuit.generate_wire_at(name.to_string(), location))
        });
        let counter_output_wires = counter_outputs.map(|name| {
            circuit.assert_explicit_name(name, location);
            Wire(circuit.generate_wire_at(name.to_string(), location))
        });

        (
            circuit,
            input_wires,
            counter_input_wires,
            output_wires,
            counter_output_wires,
        )
    }

    // WireID を払い出し、Wire の実体情報を Circuit に登録する.
    pub(crate) fn generate_wire_at(&mut self, name: String, location: SourceLocation) -> WireKey {
        self.assert_wire_name_exists(&name, location);

        let id = WireID(self.next_wire_id);
        self.next_wire_id += 1;
        self.wires
            .insert(id, Located::new(WireInfo::new(name), location));

        WireKey::new(id, self.id, location)
    }

    // Gate を登録し、あとから unify で効率よく参照できるよう WireID から逆引きする.
    pub(crate) fn push_gate_at(&mut self, gate: Gate, location: SourceLocation) {
        let gate_index = self.gates.len();
        for wire_id in gate.wire_ids() {
            self.wire_to_gates
                .entry(wire_id)
                .or_default()
                .insert(gate_index);
        }
        self.gates.push(Located::new(gate, location));
    }

    // Gate 名に使う連番を払い出す.
    pub(crate) fn generate_gate_id(&mut self) -> u32 {
        let id = self.next_gate_id;
        self.next_gate_id += 1;
        id
    }

    // unify で消える WireID を参照している Gate だけを更新する.
    pub(crate) fn replace_wire_id_in_gates(&mut self, from: WireID, to: WireID) {
        if from == to {
            return;
        }

        let Some(gate_indices) = self.wire_to_gates.remove(&from) else {
            return;
        };

        for gate_index in &gate_indices {
            self.gates[*gate_index].value.replace_wire_id(from, to);
        }
        self.wire_to_gates
            .entry(to)
            .or_default()
            .extend(gate_indices);
    }
}

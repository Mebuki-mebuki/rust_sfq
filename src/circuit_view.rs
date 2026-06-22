use std::collections::HashMap;

use crate::circuit::Circuit;
use crate::gate::Gate;
use crate::id::{CircuitID, WireID};
use crate::location::Located;
use crate::wire::WireInfo;

// タイミングチェックとバックエンド生成が読む、Circuit の読み取り専用ビュー.
// ユーザー API の const generics はここで隠し、必要な構造だけを公開する.
pub(crate) trait CircuitView {
    fn id(&self) -> CircuitID;
    fn name(&self) -> &str;
    fn in_ports(&self) -> Vec<&str>;
    fn out_ports(&self) -> Vec<&str>;
    fn all_ports(&self) -> Vec<&str>;
    fn gates(&self) -> &[Located<Gate>];
    fn wires(&self) -> &HashMap<WireID, Located<WireInfo>>;
    fn get_wire_name(&self, id: WireID) -> &str;
    fn all_wire_names(&self) -> Vec<&str>;
    fn subcircuit_input_wire_ids(&self) -> Vec<WireID>;
    fn subcircuit_output_wire_ids(&self) -> Vec<WireID>;
}

impl<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize> CircuitView
    for Circuit<N_I, N_CI, N_O, N_CO>
{
    fn id(&self) -> CircuitID {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn in_ports(&self) -> Vec<&str> {
        self.inputs
            .iter()
            .chain(self.counter_outputs.iter())
            .map(|s| s.as_str())
            .collect()
    }

    fn out_ports(&self) -> Vec<&str> {
        self.outputs
            .iter()
            .chain(self.counter_inputs.iter())
            .map(|s| s.as_str())
            .collect()
    }

    fn all_ports(&self) -> Vec<&str> {
        [self.in_ports(), self.out_ports()].concat()
    }

    fn gates(&self) -> &[Located<Gate>] {
        &self.gates
    }

    fn wires(&self) -> &HashMap<WireID, Located<WireInfo>> {
        &self.wires
    }

    fn get_wire_name(&self, id: WireID) -> &str {
        self.wires.get(&id).unwrap().value.name.as_str()
    }

    fn all_wire_names(&self) -> Vec<&str> {
        self.wires
            .values()
            .map(|info| info.value.name.as_str())
            .collect()
    }

    fn subcircuit_input_wire_ids(&self) -> Vec<WireID> {
        self.inputs
            .iter()
            .chain(self.counter_outputs.iter())
            .map(|name| self.wire_id_by_name(name))
            .collect()
    }

    fn subcircuit_output_wire_ids(&self) -> Vec<WireID> {
        self.outputs
            .iter()
            .chain(self.counter_inputs.iter())
            .map(|name| self.wire_id_by_name(name))
            .collect()
    }
}

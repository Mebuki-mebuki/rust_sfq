use crate::id::{OrderedWireID, WireID};

#[derive(Debug)]
pub(crate) enum Gate {
    Jtl {
        name: String,
        a: WireID,
        q: WireID,
    },
    Split {
        name: String,
        a: WireID,
        q1: WireID,
        q2: WireID,
    },
    Merge {
        name: String,
        a: WireID,
        b: WireID,
        q: WireID,
    },
    And {
        name: String,
        a: OrderedWireID,
        b: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Or {
        name: String,
        a: OrderedWireID,
        b: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Xor {
        name: String,
        a: OrderedWireID,
        b: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Not {
        name: String,
        a: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Xnor {
        name: String,
        a: OrderedWireID,
        b: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Dff {
        name: String,
        a: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Ndro {
        name: String,
        a: OrderedWireID,
        b: OrderedWireID,
        clk: OrderedWireID,
        q: WireID,
    },
    Buff {
        name: String,
        a: WireID,
        q: WireID,
    },
    ZeroAsync {
        name: String,
        q: WireID,
    },
    Terminate {
        name: String,
        a: WireID,
    },
    Subcircuit {
        name: String,
        inputs: Vec<WireID>,
        outputs: Vec<WireID>,
        circuit: String,
    },
    _Reserved, // 将来のゲート追加に備えてパターンマッチでワイルドカードを使ってもWarningが出ないようにする用
}

impl Gate {
    pub(crate) fn wire_ids(&self) -> Vec<WireID> {
        match self {
            Gate::Jtl { a, q, .. } => vec![*a, *q],
            Gate::Split { a, q1, q2, .. } => vec![*a, *q1, *q2],
            Gate::Merge { a, b, q, .. } => vec![*a, *b, *q],
            Gate::And { a, b, clk, q, .. }
            | Gate::Or { a, b, clk, q, .. }
            | Gate::Xor { a, b, clk, q, .. }
            | Gate::Xnor { a, b, clk, q, .. }
            | Gate::Ndro { a, b, clk, q, .. } => vec![a.id, b.id, clk.id, *q],
            Gate::Not { a, clk, q, .. } | Gate::Dff { a, clk, q, .. } => {
                vec![a.id, clk.id, *q]
            }
            Gate::Buff { a, q, .. } => vec![*a, *q],
            Gate::ZeroAsync { q, .. } => vec![*q],
            Gate::Terminate { a, .. } => vec![*a],
            Gate::Subcircuit {
                inputs, outputs, ..
            } => inputs.iter().chain(outputs.iter()).copied().collect(),
            Gate::_Reserved => unreachable!(),
        }
    }

    pub(crate) fn replace_wire_id(&mut self, from: WireID, to: WireID) {
        fn replace_id(id: &mut WireID, from: WireID, to: WireID) {
            if *id == from {
                *id = to;
            }
        }

        fn replace_ordered_id(id: &mut OrderedWireID, from: WireID, to: WireID) {
            if id.id == from {
                id.id = to;
            }
        }

        match self {
            Gate::Jtl { a, q, .. } | Gate::Buff { a, q, .. } => {
                replace_id(a, from, to);
                replace_id(q, from, to);
            }
            Gate::Split { a, q1, q2, .. } => {
                replace_id(a, from, to);
                replace_id(q1, from, to);
                replace_id(q2, from, to);
            }
            Gate::Merge { a, b, q, .. } => {
                replace_id(a, from, to);
                replace_id(b, from, to);
                replace_id(q, from, to);
            }
            Gate::And { a, b, clk, q, .. }
            | Gate::Or { a, b, clk, q, .. }
            | Gate::Xor { a, b, clk, q, .. }
            | Gate::Xnor { a, b, clk, q, .. }
            | Gate::Ndro { a, b, clk, q, .. } => {
                replace_ordered_id(a, from, to);
                replace_ordered_id(b, from, to);
                replace_ordered_id(clk, from, to);
                replace_id(q, from, to);
            }
            Gate::Not { a, clk, q, .. } | Gate::Dff { a, clk, q, .. } => {
                replace_ordered_id(a, from, to);
                replace_ordered_id(clk, from, to);
                replace_id(q, from, to);
            }
            Gate::ZeroAsync { q, .. } => replace_id(q, from, to),
            Gate::Terminate { a, .. } => replace_id(a, from, to),
            Gate::Subcircuit {
                inputs, outputs, ..
            } => {
                for id in inputs.iter_mut().chain(outputs.iter_mut()) {
                    replace_id(id, from, to);
                }
            }
            Gate::_Reserved => unreachable!(),
        }
    }
}

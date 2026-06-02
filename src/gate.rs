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

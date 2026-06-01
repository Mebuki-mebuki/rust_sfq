use crate::id::WireID;

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
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
    },
    Or {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
    },
    Xor {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
    },
    Not {
        name: String,
        a: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
    },
    Xnor {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
    },
    Dff {
        name: String,
        a: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
    },
    Ndro {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
        order: Vec<usize>,
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

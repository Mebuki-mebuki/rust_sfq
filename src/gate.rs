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
    },
    Or {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
    },
    Xor {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
    },
    Not {
        name: String,
        a: WireID,
        clk: WireID,
        q: WireID,
    },
    Xnor {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
        q: WireID,
    },
    Dff {
        name: String,
        a: WireID,
        clk: WireID,
        q: WireID,
    },
    Ndro {
        name: String,
        a: WireID,
        b: WireID,
        clk: WireID,
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
    _Reserved, // 将来のゲート追加に備えてパターンマッチでワイルドカードを使ってもWarningが出ないようにする用
}

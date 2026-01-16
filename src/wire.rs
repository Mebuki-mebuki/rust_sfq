use crate::id::{CircuitID, WireID};
use colored::Colorize;

// 配線の本質的な情報
pub(crate) struct WireInfo {
    pub(crate) name: String,
    pub(crate) delay: usize,
}

impl WireInfo {
    pub(crate) fn new(name: String) -> Self {
        Self { name, delay: 0 }
    }
}

// Wire, CounterWireの中身. データへのキーだけを持つ.
#[derive(Clone)]
pub(crate) struct WireKey {
    pub(crate) id: WireID,
    pub(crate) cid: CircuitID,
    used: bool,
}

impl WireKey {
    pub(crate) fn new(id: WireID, cid: CircuitID) -> Self {
        Self {
            id,
            cid,
            used: false,
        }
    }

    // 多重使用チェック (内部実装にミスがない限り発生しない)
    pub(crate) fn consume(&mut self) {
        assert!(!self.used);
        self.used = true;
    }
}

impl Drop for WireKey {
    // 未使用チェック (ユーザーのコードによっては発生する)
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.used, "{}", String::from("wire is unused!").red());
        }
    }
}

// ユーザーに公開する型.
pub struct Wire(pub(crate) WireKey);
pub struct CounterWire(pub(crate) WireKey);

pub type TimedWire = (Wire, usize);

// % 演算子でタイミングを記述する
impl std::ops::Rem<usize> for Wire {
    type Output = TimedWire;

    fn rem(self, rhs: usize) -> Self::Output {
        (self, rhs)
    }
}

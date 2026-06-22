use crate::circuit::Circuit;
use crate::id::{CircuitID, WireID};
use crate::location::SourceLocation;
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
    pub(crate) location: SourceLocation,
    used: bool,
}

impl WireKey {
    pub(crate) fn new(id: WireID, cid: CircuitID, location: SourceLocation) -> Self {
        Self {
            id,
            cid,
            location,
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
            assert!(
                self.used,
                "{}",
                format!("wire is unused! defined at {}", self.location).red()
            );
        }
    }
}

// ユーザーに公開する型.
pub struct Wire(pub(crate) WireKey);
pub struct CounterWire(pub(crate) WireKey);

pub type OrderedWire = (Wire, usize);

// % 演算子でタイミングを記述する
impl std::ops::Rem<usize> for Wire {
    type Output = OrderedWire;

    fn rem(self, rhs: usize) -> Self::Output {
        (self, rhs)
    }
}

impl Wire {
    pub fn label<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        self,
        label: &str,
        circuit: &mut Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> Self {
        circuit.label(&self, label);
        self
    }
}

impl CounterWire {
    pub fn label<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        self,
        label: &str,
        circuit: &mut Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> Self {
        circuit.clabel(&self, label);
        self
    }
}

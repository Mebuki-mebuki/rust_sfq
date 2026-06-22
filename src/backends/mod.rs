mod logical_verilog;
mod rsfqlib_spice;
mod rsfqlib_verilog;

use crate::circuit_view::CircuitView;

pub use logical_verilog::LogicalVerilog;
pub use rsfqlib_spice::RsfqlibSpice;
pub use rsfqlib_verilog::RsfqlibVerilog;

// Backend 実装にだけ渡す Circuit の読み取りビュー.
// new は crate 内に閉じ、ユーザーは Design 経由でのみ生成できるようにする.
pub struct BackendCircuit<'a> {
    view: &'a dyn CircuitView,
}

impl<'a> BackendCircuit<'a> {
    pub(crate) fn new(view: &'a dyn CircuitView) -> Self {
        Self { view }
    }

    pub(crate) fn view(&self) -> &dyn CircuitView {
        self.view
    }
}

// Design がタイミングチェックを済ませた Circuit から各形式の文字列を生成する.
pub trait Backend {
    fn generate(&self, circuit: &BackendCircuit<'_>) -> String;
}

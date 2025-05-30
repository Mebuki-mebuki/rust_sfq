mod rsfqlib_spice;
mod rsfqlib_verilog;

use crate::circuit::Circuit;

pub use rsfqlib_spice::RsfqlibSpice;
pub use rsfqlib_verilog::RsfqlibVerilog;

pub trait Backend {
    fn generate<const N_I: usize, const N_CO: usize, const N_O: usize, const N_CI: usize>(
        circuit: &Circuit<N_I, N_CO, N_O, N_CI>,
    ) -> String;
}

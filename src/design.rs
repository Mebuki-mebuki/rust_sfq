use crate::backends::{Backend, BackendCircuit};
use crate::circuit::Circuit;
use crate::circuit_view::CircuitView;
use crate::timing::{TimingLibrary, check_timing};

// 複数の回路をサブ回路から親回路の順にまとめた、生成単位.
pub struct Design<'a> {
    circuits: Vec<&'a dyn CircuitView>,
}

impl<'a> Design<'a> {
    pub fn new() -> Self {
        Self {
            circuits: Vec::new(),
        }
    }

    pub fn add<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        &mut self,
        circuit: &'a Circuit<N_I, N_CI, N_O, N_CO>,
    ) {
        self.circuits.push(circuit);
    }

    pub fn circuit<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        mut self,
        circuit: &'a Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> Self {
        self.add(circuit);
        self
    }

    pub fn generate<B: Backend>(&self, backend: B) -> String {
        let mut timing_library = TimingLibrary::new();
        let mut generated = Vec::new();

        for circuit in &self.circuits {
            let timing_edges = check_timing(*circuit, &timing_library);
            timing_library.insert(circuit.id(), timing_edges);
            generated.push(backend.generate(&BackendCircuit::new(*circuit)));
        }

        generated.join("\n")
    }

    pub fn print<B: Backend>(&self, backend: B) {
        println!("{}", self.generate(backend));
    }
}

impl Default for Design<'_> {
    fn default() -> Self {
        Self::new()
    }
}

use crate::gate::gate_base::Gate;
use crate::state::QuantumState;
use itertools::zip;
use ndarray::prelude::*;
use num_complex::Complex;

pub struct HGate {
    idx: u32,
    pub name: &'static str,
}

impl HGate {
    pub fn new(target: u32) -> Self {
        HGate {
            idx: target,
            name: "H",
        }
    }
}

impl Gate for HGate {
    fn apply(&self, state: &mut QuantumState) {
        let mut hqubits: Array1<Complex<f32>> = Array::zeros(state.qubits.raw_dim());
        let indices_shifted = &state.indices & (1 << &self.idx);
        let mut idx0 = vec![];
        let mut idx1 = vec![];
        for (i, idx) in indices_shifted.iter().enumerate() {
            if *idx == 0 {
                idx0.push(i);
            } else {
                idx1.push(i);
            }
        }
        for (i0, i1) in zip(&idx0, &idx1) {
            hqubits[*i0] = state.qubits[*i0] + state.qubits[*i1];
            hqubits[*i1] = state.qubits[*i0] - state.qubits[*i1];
        }
        state.qubits = hqubits.mapv(|hqubits| hqubits / 2.0_f32.sqrt());
    }
}

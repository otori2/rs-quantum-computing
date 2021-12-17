use crate::state::QuantumState;
use ndarray::prelude::*;
use ndarray::Array;
use num_complex::Complex;

pub struct Observable {
    operator: Array2<Complex<f32>>,
}

impl Observable {
    pub fn new(n_qubits: u32) -> Self {
        let nn: usize = usize::try_from(2_i32.pow(n_qubits)).unwrap();

        Observable {
            operator: Array::ones((nn, nn)),
        }
    }

    pub fn add_operator(&mut self, operator: &Array2<Complex<f32>>) {
        self.operator = operator.dot(&self.operator);
    }

    pub fn get_expectation_value(&self, state: &QuantumState) -> f32 {
        let mut qconj: Array1<Complex<f32>> = Array::zeros(state.qubits.dim());
        for i in 0..state.qubits.len() {
            qconj[[i]] = state.qubits[[i]].conj();
        }
        qconj.t().dot(&self.operator.dot(&state.qubits)).re
    }
}

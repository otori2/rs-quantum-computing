extern crate blas_src;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate rand;
use ndarray::prelude::*;
use ndarray::Array;
use num_complex::Complex;
use rand::Rng;

pub struct QuantumState {
    pub n_qubits: u32,
    n_size: usize,
    pub qubits: Array1<Complex<f32>>,
    pub indices: Array1<u32>,
}

impl QuantumState {
    pub fn new(n_qubits: u32) -> Self {
        // let qubits: Array1<Complex<f64>> =
        //     ndarray_linalg::random(usize::try_from(2_i32.pow(n_qubits)).unwrap());
        let n_size: usize = usize::try_from(2_i32.pow(n_qubits)).unwrap();
        let mut qubits: Array1<Complex<f32>> = Array::zeros(n_size);
        qubits[0] = Complex::new(1.0, 0.0);
        let mut indices: Array1<u32> = Array::zeros(n_size);
        for i in 0..indices.len() {
            indices[i] = i as u32;
        }

        QuantumState {
            n_qubits,
            n_size,
            qubits,
            indices,
        }
    }

    pub fn set_zero_state(&mut self) {
        self.qubits = Array::zeros(self.n_size);
        self.qubits[0] = Complex::new(1.0, 0.0);
    }

    pub fn get_vector(&self) -> Array1<Complex<f32>> {
        self.qubits.clone()
    }

    pub fn get_zero_probability(&self, idx: u32) -> f32 {
        let indices_shifted = &self.indices & (1 << idx);
        let mut prob = 0.0;
        for i in 0..self.n_size {
            if indices_shifted[i] == 0 {
                prob += (self.qubits[i].conj() * self.qubits[i]).re;
            }
        }
        prob
    }

    pub fn measure(&mut self, idx: u32) -> u32 {
        let indices_shifted = &self.indices & (1 << idx);
        let p0 = self.get_zero_probability(idx);
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < p0 {
            for i in 0..self.n_size {
                if indices_shifted[i] != 0 {
                    self.qubits[i] = Complex::new(0.0, 0.0);
                }
            }
            self.qubits.mapv_inplace(|a| a / p0);
            0
        } else {
            for i in 0..self.n_size {
                if indices_shifted[i] == 0 {
                    self.qubits[i] = Complex::new(0.0, 0.0);
                }
            }
            self.qubits.mapv_inplace(|a| a / (1.0 - p0));
            1
        }
    }
}

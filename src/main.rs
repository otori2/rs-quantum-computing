extern crate blas_src;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate rand;

use ndarray::prelude::*;
use num_complex::Complex;

use rs_quantum_simulator::circuit::QuantumCircuit;
use rs_quantum_simulator::gate::gate_hadamard::HGate;
use rs_quantum_simulator::observable::Observable;
use rs_quantum_simulator::state::QuantumState;

fn main() {
    let n_qubits = 3;
    let mut state = QuantumState::new(n_qubits);
    state.set_zero_state();

    println!("{}", state.get_vector());

    // println!("{}", state.n_qubits);
    // println!("{}", state.qubits);
    // println!("{}", state.indices);
    // for i in 0..n_qubits {
    //     println!("{}", state.get_zero_probability(i));
    // }
    // println!("{}", state.get_vector());
    // for i in 0..n_qubits {
    //     println!("{}", state.measure(i));
    // }

    // let target_idx = 1;
    // let H = HGate::new(target_idx);
    // let H = Rc::new(HGate::new(target_idx));
    // let H = Box::new(HGate::new(target_idx));
    // println!("Gate: {}", H.name);
    // println!("{}", state.get_vector());
    // H.apply(&mut state);
    // println!("{}", state.get_vector());

    let H0 = Box::new(HGate::new(0));
    let H1 = Box::new(HGate::new(1));

    let mut circuit = QuantumCircuit::new(n_qubits);
    // circuit.add_gate(H0);
    circuit.add_gate(H1);
    circuit.update_quantum_state(&mut state);

    println!("{}", state.get_vector());

    let nn: usize = usize::try_from(2_i32.pow(n_qubits)).unwrap();
    let mut A: Array2<Complex<f32>> = ndarray_linalg::random((nn, nn));
    let mut B: Array2<Complex<f32>> = ndarray_linalg::random((nn, nn));

    let mut observable = Observable::new(n_qubits);
    // observable.add_operator(&A);
    // observable.add_operator(&A);
    // observable.add_operator(&B);
    println!("{}", observable.get_expectation_value(&state));

    println!("{}", state.get_zero_probability(1));
    println!("{}", state.measure(0));

    println!("OK");
}

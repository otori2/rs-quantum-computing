use crate::gate::gate_base::Gate;
use crate::state::QuantumState;

pub struct QuantumCircuit {
    n_qubits: u32,
    // gates: Vec<Rc<dyn Gate>>,
    gates: Vec<Box<dyn Gate>>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: u32) -> Self {
        QuantumCircuit {
            n_qubits,
            gates: vec![],
        }
    }

    pub fn add_gate(&mut self, gate: Box<dyn Gate>) {
        self.gates.push(gate);
    }

    pub fn update_quantum_state(&self, state: &mut QuantumState) {
        for gate in &self.gates {
            gate.apply(state);
        }
    }
}

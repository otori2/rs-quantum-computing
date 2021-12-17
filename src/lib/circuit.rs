use crate::gate::gate_base::Gate;
use crate::state::QuantumState;

pub struct QuantumCircuit {
    // gates: Vec<Rc<dyn Gate>>,
    gates: Vec<Box<dyn Gate>>,
}

impl Default for QuantumCircuit {
    fn default() -> Self {
        QuantumCircuit { gates: vec![] }
    }
}

impl QuantumCircuit {
    pub fn add_gate(&mut self, gate: Box<dyn Gate>) {
        self.gates.push(gate);
    }

    pub fn update_quantum_state(&self, state: &mut QuantumState) {
        for gate in &self.gates {
            gate.apply(state);
        }
    }
}

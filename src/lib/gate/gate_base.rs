use crate::state::QuantumState;
pub trait Gate {
    fn apply(&self, state: &mut QuantumState);
}

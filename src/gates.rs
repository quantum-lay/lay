use crate::Layer;

/// Layers for which implements Pauli gates.
pub trait PauliGate : Layer {}

/// Layers for which implements Hadamard gates.
pub trait HGate : Layer {}

/// Layers for which implements S gates.
pub trait SGate : Layer {}

/// Layers for which implements T gates.
pub trait TGate : Layer {}

/// Layers for which implements CNOT gates.
pub trait CXGate : Layer {}

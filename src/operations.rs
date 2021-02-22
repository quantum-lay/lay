//! Traits for operations.

use crate::Layer;
use crate::gates::{PauliGate, HGate, SGate, TGate, CXGate};

mod opsvec;
pub use opsvec::OpsVec;

mod opargs;
pub use opargs::{opid, OpArgs};

/// Provides operations for initialize and measurement.
pub trait Operation<L> where L: Layer + ?Sized {
    fn initialize() -> Self;
    fn measure(q: L::Qubit, s: L::Slot) -> Self;
}

/// Provides operations for Pauli X, Y, Z gate.
pub trait PauliOperation<L> where L: Layer + PauliGate + ?Sized {
    fn x(q: L::Qubit) -> Self;
    fn y(q: L::Qubit) -> Self;
    fn z(q: L::Qubit) -> Self;
}

/// Provides operations for Hadamard gate.
pub trait HOperation<L> where L: Layer + HGate + ?Sized {
    fn h(q: L::Qubit) -> Self;
}

/// Provides operations for S and S† gate.
pub trait SOperation<L> where L: Layer + SGate + ?Sized {
    fn s(q: L::Qubit) -> Self;
    fn sdg(q: L::Qubit) -> Self;
}

/// Provides operations for T and T† gate.
pub trait TOperation<L> where L: Layer + TGate + ?Sized {
    fn t(q: L::Qubit) -> Self;
    fn tdg(q: L::Qubit) -> Self;
}

/// Provides operations for CNOT gate.
pub trait CXOperation<L> where L: Layer + CXGate + ?Sized {
    fn cx(c: L::Qubit, t: L::Qubit) -> Self;
}

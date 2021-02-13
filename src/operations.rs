use crate::Layer;
use crate::gates::{PauliGate, HGate, SGate, TGate, CXGate};

mod opsvec;
pub use opsvec::OpsVec;

mod opargs;
pub use opargs::{opid, OpArgs};

pub trait Operation<L> where L: Layer + ?Sized {
    fn initialize() -> Self;
    fn measure(q: L::Qubit, s: L::Slot) -> Self;
}

pub trait PauliOperation<L> where L: Layer + PauliGate + ?Sized {
    fn x(q: L::Qubit) -> Self;
    fn y(q: L::Qubit) -> Self;
    fn z(q: L::Qubit) -> Self;
}

pub trait HOperation<L> where L: Layer + HGate + ?Sized {
    fn h(q: L::Qubit) -> Self;
}

pub trait SOperation<L> where L: Layer + SGate + ?Sized {
    fn s(q: L::Qubit) -> Self;
    fn sdg(q: L::Qubit) -> Self;
}

pub trait TOperation<L> where L: Layer + TGate + ?Sized {
    fn t(q: L::Qubit) -> Self;
    fn tdg(q: L::Qubit) -> Self;
}

pub trait CXOperation<L> where L: Layer + CXGate + ?Sized {
    fn cx(c: L::Qubit, t: L::Qubit) -> Self;
}

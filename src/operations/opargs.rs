use std::any::Any;
use std::fmt::Debug;

use crate::Layer;
use crate::gates::{PauliGate, HGate, SGate, TGate, CXGate};
use crate::operations::{Operation, PauliOperation, HOperation, SOperation, TOperation, CXOperation};

/// Builtin operation IDs.
pub mod opid {
    /// Initialize
    pub const INIT: u16 = 1;
    /// Measurement
    pub const MEAS: u16 = 2;
    /// Pauli X gate
    pub const X: u16 = 3;
    /// Pauli Y gate
    pub const Y: u16 = 4;
    /// Pauli Z gate
    pub const Z: u16 = 5;
    /// Hadamard gate
    pub const H: u16 = 6;
    /// S gate
    pub const S: u16 = 7;
    /// S† gate
    pub const SDG: u16 = 8;
    /// T gate
    pub const T: u16 = 9;
    /// T† gate
    pub const TDG: u16 = 10;
    /// CNOT gate
    pub const CX: u16 = 11;
    /// When library user defines new operation ID,
    /// the value should greater than or equal to this value.
    pub const USERDEF: u16 = 256;
}

/// An reference implementation of `Operation`.
#[derive(Debug)]
pub enum OpArgs<L: Layer + ?Sized> {
    Empty(u16),
    Q(u16, L::Qubit),
    QQ(u16, L::Qubit, L::Qubit),
    QS(u16, L::Qubit, L::Slot),
    QF(u16, L::Qubit, f32),
    QD(u16, L::Qubit, f64),
    QFF(u16, L::Qubit, f32, f32),
    Var(u16, Box<dyn Any + Send>),
}

impl<L> Operation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + ?Sized {
    fn initialize() -> OpArgs<L> {
        OpArgs::Empty(opid::INIT)
    }

    fn measure(q: L::Qubit, s: L::Slot) -> OpArgs<L> {
        OpArgs::QS(opid::MEAS, q, s)
    }
}

impl<L> PauliOperation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + PauliGate + ?Sized {
    fn x(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::X, q)
    }

    fn y(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::Y, q)
    }

    fn z(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::Z, q)
    }
}

impl<L> HOperation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + HGate + ?Sized {
    fn h(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::H, q)
    }
}

impl<L> SOperation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + SGate + ?Sized {
    fn s(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::S, q)
    }

    fn sdg(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::SDG, q)
     }
}

impl<L> TOperation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + TGate + ?Sized {
    fn t(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::T, q)
     }

    fn tdg(q: L::Qubit) -> OpArgs<L> {
        OpArgs::Q(opid::TDG, q)
    }
}

impl<L> CXOperation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + CXGate + ?Sized {
    fn cx(c: L::Qubit, t: L::Qubit) -> OpArgs<L> {
        OpArgs::QQ(opid::CX, c, t)
    }
}

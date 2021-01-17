use std::any::Any;
use std::convert::{AsRef, AsMut};
use std::fmt::Debug;
use crate::gates::*;
use crate::Layer;

pub mod opid {
    pub const INIT: u16 = 1;
    pub const MEAS: u16 = 2;
    pub const X: u16 = 3;
    pub const Y: u16 = 4;
    pub const Z: u16 = 5;
    pub const H: u16 = 6;
    pub const S: u16 = 7;
    pub const SDG: u16 = 8;
    pub const T: u16 = 9;
    pub const TDG: u16 = 10;
    pub const CX: u16 = 11;
    pub const USERDEF: u16 = 256;
}

pub trait Operation<L> where L: Layer + ?Sized {
    fn initialize() -> Self;
    fn measure(q: L::Qubit, s: L::Slot) -> Self;
    fn x(q: L::Qubit) -> Self where L: PauliGate;
    fn y(q: L::Qubit) -> Self where L: PauliGate;
    fn z(q: L::Qubit) -> Self where L: PauliGate;
    fn h(q: L::Qubit) -> Self where L: HGate;
    fn s(q: L::Qubit) -> Self where L: SGate;
    fn sdg(q: L::Qubit) -> Self where L: SGate;
    fn t(q: L::Qubit) -> Self where L: TGate;
    fn tdg(q: L::Qubit) -> Self where L: TGate;
    fn cx(c: L::Qubit, t: L::Qubit) -> Self where L: CXGate;
}

#[derive(Debug)]
pub enum OpArgs<L: Layer + ?Sized> {
    Empty(u16),
    Q(u16, L::Qubit),
    QQ(u16, L::Qubit, L::Qubit),
    QS(u16, L::Qubit, L::Slot),
    QF(u16, L::Qubit, f32),
    QD(u16, L::Qubit, f64),
    QFF(u16, L::Qubit, f32, f32),
    Var(u16, Box<dyn Any>),
}

impl<L> Operation<L> for OpArgs<L> where L: Layer<Operation=OpArgs<L>> + ?Sized {
    fn initialize() -> OpArgs<L> {
        OpArgs::Empty(opid::INIT)
    }

    fn measure(q: L::Qubit, s: L::Slot) -> OpArgs<L> {
        OpArgs::QS(opid::MEAS, q, s)
    }

    fn x(q: L::Qubit) -> OpArgs<L> where L: PauliGate {
        OpArgs::Q(opid::X, q)
    }

    fn y(q: L::Qubit) -> OpArgs<L> where L: PauliGate {
        OpArgs::Q(opid::Y, q)
    }

    fn z(q: L::Qubit) -> OpArgs<L> where L: PauliGate {
        OpArgs::Q(opid::Z, q)
    }

    fn h(q: L::Qubit) -> OpArgs<L> where L: HGate {
        OpArgs::Q(opid::H, q)
    }

    fn s(q: L::Qubit) -> OpArgs<L> where L: SGate {
        OpArgs::Q(opid::S, q)
    }

    fn sdg(q: L::Qubit) -> OpArgs<L> where L: SGate {
        OpArgs::Q(opid::SDG, q)
     }

    fn t(q: L::Qubit) -> OpArgs<L> where L: TGate {
        OpArgs::Q(opid::T, q)
     }

    fn tdg(q: L::Qubit) -> OpArgs<L> where L: TGate {
        OpArgs::Q(opid::TDG, q)
    }

    fn cx(c: L::Qubit, t: L::Qubit) -> OpArgs<L> where L: CXGate {
        OpArgs::QQ(opid::CX, c, t)
    }
}

#[derive(Debug)]
pub struct OpsVec<L: Layer> where {
    inner: Vec<L::Operation>,
}

impl<L> OpsVec<L> where L: Layer {
    pub fn new() -> Self {
        OpsVec::from_vec(vec![])
    }

    pub fn from_vec(v: Vec<L::Operation>) -> Self {
        OpsVec { inner: v }
    }

    pub fn into_vec(self) -> Vec<L::Operation> {
        self.inner
    }

    pub fn as_slice(&self) -> &[L::Operation] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [L::Operation] {
        &mut self.inner
    }

    pub fn as_vec(&self) -> &Vec<L::Operation> {
        &self.inner
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<L::Operation> {
        &mut self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&L::Operation> {
        self.inner.iter()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

impl<L: Layer> AsRef<[L::Operation]> for OpsVec<L> {
    fn as_ref(&self) -> &[L::Operation] {
        self.as_slice()
    }
}

impl<L: Layer> AsMut<[L::Operation]> for OpsVec<L> {
    fn as_mut(&mut self) -> &mut [L::Operation] {
        self.as_mut_slice()
    }
}

impl<L> OpsVec<L> where L: Layer, L::Operation: Operation<L> {
    pub fn initialize(&mut self) {
        self.inner.push(L::Operation::initialize());
    }
    pub fn measure(&mut self, q: L::Qubit, s: L::Slot) {
        self.inner.push(L::Operation::measure(q, s));
    }

    pub fn x(&mut self, q: <L as Layer>::Qubit) where L: PauliGate {
        self.inner.push(L::Operation::x(q));
    }

    pub fn y(&mut self, q: <L as Layer>::Qubit) where L: PauliGate {
        self.inner.push(L::Operation::y(q));
    }

    pub fn z(&mut self, q: <L as Layer>::Qubit) where L: PauliGate {
        self.inner.push(L::Operation::z(q));
    }

    pub fn h(&mut self, q: <L as Layer>::Qubit) where L: HGate {
        self.inner.push(L::Operation::h(q));
    }

    pub fn s(&mut self, q: <L as Layer>::Qubit) where L: SGate {
        self.inner.push(L::Operation::s(q));
    }

    pub fn sdg(&mut self, q: <L as Layer>::Qubit) where L: SGate {
        self.inner.push(L::Operation::sdg(q));
    }

    pub fn t(&mut self, q: <L as Layer>::Qubit) where L: TGate {
        self.inner.push(L::Operation::t(q));
    }

    pub fn tdg(&mut self, q: <L as Layer>::Qubit) where L: TGate {
        self.inner.push(L::Operation::tdg(q));
    }

    pub fn cx(&mut self, c: <L as Layer>::Qubit, t: <L as Layer>::Qubit) where L: CXGate {
        self.inner.push(L::Operation::cx(c, t));
    }
}

use std::fmt::Debug;
use std::convert::{AsRef, AsMut};
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

pub trait OpArgs: Debug {}

#[derive(Debug)]
pub enum Operation<L: Layer + ?Sized> {
    Empty(u16),
    Q(u16, L::Qubit),
    QQ(u16, L::Qubit, L::Qubit),
    QS(u16, L::Qubit, L::Slot),
    QF(u16, L::Qubit, f32),
    QD(u16, L::Qubit, f64),
    QFF(u16, L::Qubit, f32, f32),
    Var(u16, Box<dyn OpArgs>),
}

fn initialize<L: Layer>() -> Operation<L> {
    Operation::Empty(opid::INIT)
}

fn measure<L: Layer>(q: L::Qubit, s: L::Slot) -> Operation<L> {
    Operation::QS(opid::MEAS, q, s)
}

fn x<L: PauliGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::X, q)
}

fn y<L: PauliGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::Y, q)
}

fn z<L: PauliGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::Z, q)
}

fn h<L: HGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::H, q)
}

fn s<L: SGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::S, q)
}

fn sdg<L: SGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::SDG, q)
}

fn t<L: TGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::T, q)
}

fn tdg<L: TGate>(q: L::Qubit) -> Operation<L> {
    Operation::Q(opid::TDG, q)
}

fn cx<L: CXGate>(c: L::Qubit, t: L::Qubit) -> Operation<L> {
    Operation::QQ(opid::CX, c, t)
}

#[derive(Debug)]
pub struct OpsVec<L: Layer> {
    inner: Vec<Operation<L>>
}

impl<L: Layer> OpsVec<L> {
    pub fn new() -> OpsVec<L> {
        OpsVec::from_vec(vec![])
    }

    pub fn from_vec(v: Vec<Operation<L>>) -> OpsVec<L> {
        OpsVec { inner: v }
    }

    pub fn into_vec(self) -> Vec<Operation<L>> {
        self.inner
    }

    pub fn as_slice(&self) -> &[Operation<L>] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [Operation<L>] {
        &mut self.inner
    }

    pub fn as_vec(&self) -> &Vec<Operation<L>> {
        &self.inner
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<Operation<L>> {
        &mut self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&Operation<L>> {
        self.inner.iter()
    }

    pub fn initialize(&mut self) {
        self.inner.push(initialize());
    }
    pub fn measure(&mut self, q: L::Qubit, s: L::Slot) {
        self.inner.push(measure(q, s));
    }
}

impl<L: Layer> AsRef<[Operation<L>]> for OpsVec<L> {
    fn as_ref(&self) -> &[Operation<L>] {
        self.as_slice()
    }
}

impl<L: Layer> AsMut<[Operation<L>]> for OpsVec<L> {
    fn as_mut(&mut self) -> &mut [Operation<L>] {
        self.as_mut_slice()
    }
}

impl<L: PauliGate> OpsVec<L> {
    pub fn x(&mut self, q: L::Qubit) {
        self.inner.push(x(q));
    }

    pub fn y(&mut self, q: L::Qubit) {
        self.inner.push(y(q));
    }

    pub fn z(&mut self, q: L::Qubit) {
        self.inner.push(z(q));
    }
}

impl<L: HGate> OpsVec<L> {
    pub fn h(&mut self, q: L::Qubit) {
        self.inner.push(h(q));
    }
}

impl<L: SGate> OpsVec<L> {
    pub fn s(&mut self, q: L::Qubit) {
        self.inner.push(s(q));
    }

    pub fn sdg(&mut self, q: L::Qubit) {
        self.inner.push(sdg(q));
    }
}

impl<L: TGate> OpsVec<L> {
    pub fn t(&mut self, q: L::Qubit) {
        self.inner.push(t(q));
    }

    pub fn tdg(&mut self, q: L::Qubit) {
        self.inner.push(tdg(q));
    }
}

impl<L: CXGate> OpsVec<L> {
    pub fn cx(&mut self, c: L::Qubit, t: L::Qubit) {
        self.inner.push(cx(c, t));
    }
}

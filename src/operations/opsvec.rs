use std::convert::{AsRef, AsMut};

use crate::Layer;
use crate::gates::{PauliGate, HGate, SGate, TGate, CXGate};
use crate::operations::{Operation, PauliOperation, HOperation, SOperation, TOperation, CXOperation};

/// Vec wrapper for building slice of `Operation`s.
#[derive(Debug)]
pub struct OpsVec<L: Layer + ?Sized> {
    inner: Vec<L::Operation>,
}

impl<L> OpsVec<L> where L: Layer + ?Sized {
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

impl<L: Layer + ?Sized> AsRef<[L::Operation]> for OpsVec<L> {
    fn as_ref(&self) -> &[L::Operation] {
        self.as_slice()
    }
}

impl<L: Layer + ?Sized> AsMut<[L::Operation]> for OpsVec<L> {
    fn as_mut(&mut self) -> &mut [L::Operation] {
        self.as_mut_slice()
    }
}

impl<L> OpsVec<L> where L: Layer + ?Sized, L::Operation: Operation<L> {
    pub fn initialize(&mut self) {
        self.inner.push(L::Operation::initialize());
    }
    pub fn measure(&mut self, q: L::Qubit, s: L::Slot) {
        self.inner.push(L::Operation::measure(q, s));
    }
}

impl<L> OpsVec<L> where L: Layer + PauliGate + ?Sized, L::Operation: PauliOperation<L> {
    pub fn x(&mut self, q: <L as Layer>::Qubit) {
        self.inner.push(L::Operation::x(q));
    }

    pub fn y(&mut self, q: <L as Layer>::Qubit) {
        self.inner.push(L::Operation::y(q));
    }

    pub fn z(&mut self, q: <L as Layer>::Qubit) {
        self.inner.push(L::Operation::z(q));
    }
}

impl<L> OpsVec<L> where L: Layer + HGate + ?Sized, L::Operation: HOperation<L> {
    pub fn h(&mut self, q: <L as Layer>::Qubit) {
        self.inner.push(L::Operation::h(q));
    }
}

impl<L> OpsVec<L> where L: Layer + SGate + ?Sized, L::Operation: SOperation<L> {
    pub fn s(&mut self, q: <L as Layer>::Qubit) where L: SGate {
        self.inner.push(L::Operation::s(q));
    }

    pub fn sdg(&mut self, q: <L as Layer>::Qubit) where L: SGate {
        self.inner.push(L::Operation::sdg(q));
    }
}

impl<L> OpsVec<L> where L: Layer + TGate + ?Sized, L::Operation: TOperation<L> {
    pub fn t(&mut self, q: <L as Layer>::Qubit) where L: TGate {
        self.inner.push(L::Operation::t(q));
    }

    pub fn tdg(&mut self, q: <L as Layer>::Qubit) where L: TGate {
        self.inner.push(L::Operation::tdg(q));
    }
}

impl<L> OpsVec<L> where L: Layer + CXGate + ?Sized, L::Operation: CXOperation<L> {
    pub fn cx(&mut self, c: <L as Layer>::Qubit, t: <L as Layer>::Qubit) where L: CXGate {
        self.inner.push(L::Operation::cx(c, t));
    }
}

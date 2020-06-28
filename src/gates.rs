use crate::Operations;

pub trait CliffordGate : Operations {
    fn x(&mut self, q: Self::Qubit);
    fn y(&mut self, q: Self::Qubit);
    fn z(&mut self, q: Self::Qubit);
    fn h(&mut self, q: Self::Qubit);
    fn s(&mut self, q: Self::Qubit);
    fn sdg(&mut self, q: Self::Qubit);
    fn cx(&mut self, c: Self::Qubit, t: Self::Qubit);
}

pub trait TGate : Operations {
    fn t(&mut self, q: Self::Qubit);
    fn tdg(&mut self, q: Self::Qubit);
}

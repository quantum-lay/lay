use crate::Qubit;

pub trait CliffordGate {
    fn x(&mut self, q: Qubit);
    fn y(&mut self, q: Qubit);
    fn z(&mut self, q: Qubit);
    fn h(&mut self, q: Qubit);
    fn s(&mut self, q: Qubit);
    fn sdg(&mut self, q: Qubit);
    fn cx(&mut self, c: Qubit, t: Qubit);
}

pub trait TGate {
    fn t(&mut self, q: Qubit);
    fn tdg(&mut self, q: Qubit);
}

use crate::Qubit;

pub trait PauliGate {
    fn x(&mut self, q: Qubit);
    fn y(&mut self, q: Qubit);
    fn z(&mut self, q: Qubit);
}

pub trait CXGate {
    fn cx(&mut self, c: Qubit, t: Qubit);
}

use crate::Qubit;

pub trait XGate {
    fn x(&mut self, q: Qubit);
}

pub trait YGate {
    fn y(&mut self, q: Qubit);
}

pub trait ZGate {
    fn z(&mut self, q: Qubit);
}

pub trait CXGate {
    fn cx(&mut self, c: Qubit, t: Qubit);
}

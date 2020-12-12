use crate::Layer;

pub trait PauliGate : Layer {
    fn x(&mut self, q: Self::Qubit);
    fn y(&mut self, q: Self::Qubit);
    fn z(&mut self, q: Self::Qubit);
}

pub trait HGate : Layer {
    fn h(&mut self, q: Self::Qubit);
}

pub trait SGate : Layer {
    fn s(&mut self, q: Self::Qubit);
    fn sdg(&mut self, q: Self::Qubit);
}

pub trait TGate : Layer {
    fn t(&mut self, q: Self::Qubit);
    fn tdg(&mut self, q: Self::Qubit);
}

pub trait CXGate : Layer {
    fn cx(&mut self, c: Self::Qubit, t: Self::Qubit);
}

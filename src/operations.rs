use crate::gates::*;
use crate::Layer;

pub mod gateid {
    const MEAS: u16 = 1;
    const X: u16 = 2;
    const Y: u16 = 3;
    const Z: u16 = 4;
    const H: u16 = 5;
    const S: u16 = 6;
    const SDG: u16 = 7;
    const T: u16 = 8;
    const TDG: u16 = 9;
    const CX: u16 = 10;
    const USERDEF: u16 = 256;
}

pub struct OpsVec<L: Layer> {
}

enum Operation<L: Layer> {
    Q(u16, L::Qubit),
    QQ(u16, L::Qubit, L::Slot),
    QS(u16, L::Qubit, L::Slot),
    QF(u16, L::Qubit, f32),
    QD(u16, L::Qubit, f64),
    QFF(u16, L::Qubit, f32, f32),
    QFFF(u16, L::Qubit, f32, f32, f32),
}

impl<L: PauliGate> PauliGate {
    fn x(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::X, q));
    }
    fn y(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::Y, q));
    }
    fn z(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::Z, q));
    }
}

pub trait HGate : Layer {
    fn h(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::H, q));
    }
}

pub trait SGate : Layer {
    fn s(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::S, q));
    }
    fn sdg(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::SDG, q));
    }
}

pub trait TGate : Layer {
    fn t(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::T, q));
    }
    fn tdg(&mut self, q: Self::Qubit) {
        self.vec.push(Operation<L>::Q(gateid::TDG, q));
    }
}

pub trait CXGate : Layer {
    fn cx(&mut self, c: Self::Qubit, t: Self::Qubit) {
        self.vec.push(Operation<L>::QQ(gateid::CX, c, t));
    }
}

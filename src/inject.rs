use std::marker::PhantomData;
use crate::{Layer,
            gates::{PauliGate, HGate, SGate, TGate, CXGate},
            operations::{Operation, PauliOperation, HOperation, SOperation, TOperation, CXOperation}};

pub struct InjectLayer<L: Layer,
                        F: Fn(&mut L, &[L::Operation]) -> L::Requested,
                        G: Fn(&mut L, &mut L::Buffer) -> L::Response,
                        H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
{
    layer: L,
    f_send: F,
    f_receive: G,
    f_send_receive: H,
}

impl<L: Layer,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> InjectLayer<L, F, G, H>
{
    pub fn new(layer: L, f_send: F, f_receive: G, f_send_receive: H) -> Self {
        InjectLayer { layer, f_send, f_receive, f_send_receive }
    }
}

impl<L: Layer,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> Layer for InjectLayer<L, F, G, H>
{
    type Operation = InjectOperation<L, F, G, H>;
    type Qubit = L::Qubit;
    type Slot = L::Slot;
    type Buffer = L::Buffer;
    type Requested = L::Requested;
    type Response = L::Response;

    fn make_buffer(&self) -> L::Buffer {
        self.layer.make_buffer()
    }

    fn send(&mut self, ops: &[Self::Operation]) -> L::Requested {
        (self.f_send)(&mut self.layer, unsafe { std::mem::transmute(ops) })
    }

    fn receive(&mut self, buf: &mut L::Buffer) -> L::Response {
        (self.f_receive)(&mut self.layer, buf)
    }

    fn send_receive(&mut self, ops: &[Self::Operation], buf: &mut L::Buffer) -> L::Response {
        (self.f_send_receive)(&mut self.layer, unsafe { std::mem::transmute(ops) }, buf)
    }
}

impl<L: Layer + PauliGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> PauliGate for InjectLayer<L, F, G, H> {}
impl<L: Layer + HGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> HGate for InjectLayer<L, F, G, H> {}
impl<L: Layer + SGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> SGate for InjectLayer<L, F, G, H> {}
impl<L: Layer + TGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> TGate for InjectLayer<L, F, G, H> {}
impl<L: Layer + CXGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> CXGate for InjectLayer<L, F, G, H> {}

#[repr(transparent)]
pub struct InjectOperation<L: Layer,
                            F: Fn(&mut L, &[L::Operation]) -> L::Requested,
                            G: Fn(&mut L, &mut L::Buffer) -> L::Response,
                            H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
(L::Operation, PhantomData<(F, G, H)>);

impl<L: Layer,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> InjectOperation<L, F, G, H>
{
    pub fn new(op: L::Operation) -> Self {
        Self (op, PhantomData)
    }
}


impl<L: Layer,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
 Operation<InjectLayer<L, F, G, H>> for InjectOperation<L, F, G, H>
        where L::Operation: Operation<L>
 {
     fn initialize() -> Self {
        Self::new(L::Operation::initialize())
    }

     fn measure(q: L::Qubit, s: L::Slot) -> Self {
         Self::new(L::Operation::measure(q, s))
    }
 }

impl<L: Layer + PauliGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
PauliOperation<InjectLayer<L, F, G, H>> for InjectOperation<L, F, G, H>
    where L::Operation: Operation<L> + PauliOperation<L>
{
    fn x(q: L::Qubit) -> Self {
        Self::new(L::Operation::x(q))
    }

    fn y(q: L::Qubit) -> Self {
        Self::new(L::Operation::y(q))
    }

    fn z(q: L::Qubit) -> Self {
        Self::new(L::Operation::z(q))
    }
}

impl<L: Layer + HGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
HOperation<InjectLayer<L, F, G, H>> for InjectOperation<L, F, G, H>
    where L::Operation: Operation<L> + HOperation<L>
{
    fn h(q: L::Qubit) -> Self {
        Self::new(L::Operation::h(q))
    }
}

impl<L: Layer + SGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
SOperation<InjectLayer<L, F, G, H>> for InjectOperation<L, F, G, H>
    where L::Operation: Operation<L> + SOperation<L>
{
    fn s(q: L::Qubit) -> Self {
        Self::new(L::Operation::s(q))
    }

    fn sdg(q: L::Qubit) -> Self {
        Self::new(L::Operation::sdg(q))
    }
}

impl<L: Layer + TGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
TOperation<InjectLayer<L, F, G, H>> for InjectOperation<L, F, G, H>
    where L::Operation: Operation<L> + TOperation<L>
{
    fn t(q: L::Qubit) -> Self {
        Self::new(L::Operation::t(q))
    }

    fn tdg(q: L::Qubit) -> Self {
        Self::new(L::Operation::tdg(q))
    }
}

impl<L: Layer + CXGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response>
CXOperation<InjectLayer<L, F, G, H>> for InjectOperation<L, F, G, H>
    where L::Operation: Operation<L> + CXOperation<L>
{
    fn cx(c: L::Qubit, t: L::Qubit) -> Self {
        Self::new(L::Operation::cx(c, t))
    }
}

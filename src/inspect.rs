use crate::{Layer, gates::{PauliGate, HGate, SGate, TGate, CXGate}};

pub struct InspectLayer<L: Layer,
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
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> InspectLayer<L, F, G, H>
{
    pub fn new(layer: L, f_send: F, f_receive: G, f_send_receive: H) -> Self {
        InspectLayer { layer, f_send, f_receive, f_send_receive }
    }
}

impl<L: Layer,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> Layer for InspectLayer<L, F, G, H>
{
    type Operation = L::Operation;
    type Qubit = L::Qubit;
    type Slot = L::Slot;
    type Buffer = L::Buffer;
    type Requested = L::Requested;
    type Response = L::Response;

    fn make_buffer(&self) -> L::Buffer {
        self.layer.make_buffer()
    }

    fn send(&mut self, ops: &[L::Operation]) -> L::Requested {
        (self.f_send)(&mut self.layer, ops)
    }

    fn receive(&mut self, buf: &mut L::Buffer) -> L::Response {
        (self.f_receive)(&mut self.layer, buf)
    }

    fn send_receive(&mut self, ops: &[L::Operation], buf: &mut L::Buffer) -> L::Response {
        (self.f_send_receive)(&mut self.layer, ops, buf)
    }
}

impl<L: Layer + PauliGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> PauliGate for InspectLayer<L, F, G, H> {}
impl<L: Layer + HGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> HGate for InspectLayer<L, F, G, H> {}
impl<L: Layer + SGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> SGate for InspectLayer<L, F, G, H> {}
impl<L: Layer + TGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> TGate for InspectLayer<L, F, G, H> {}
impl<L: Layer + CXGate,
     F: Fn(&mut L, &[L::Operation]) -> L::Requested,
     G: Fn(&mut L, &mut L::Buffer) -> L::Response,
     H: Fn(&mut L, &[L::Operation], &mut L::Buffer) -> L::Response> CXGate for InspectLayer<L, F, G, H> {}

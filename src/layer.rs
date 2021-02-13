use crate::{Measured, OpsVec};

pub trait Layer {
    type Operation;
    type Qubit;
    type Slot;
    type Buffer: Measured<Slot=Self::Slot>;
    type Requested;
    type Response;

    fn send(&mut self, ops: &[Self::Operation]) -> Self::Requested;

    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;

    fn send_receive(&mut self, ops: &[Self::Operation], buf: &mut Self::Buffer) -> Self::Response {
        self.send(ops);
        self.receive(buf)
    }

    fn make_buffer(&self) -> Self::Buffer;

    fn opsvec(&self) -> OpsVec<Self> {
        OpsVec::new()
    }
}

//! Trait for abstract quantum computer.
use crate::{Measured, OpsVec};

/// Sends operations and receives result.
pub trait Layer {
    type Operation;
    type Qubit;
    type Slot;
    type Buffer: Measured<Slot=Self::Slot>;
    type Requested;
    type Response;

    /// Sends operations.
    fn send(&mut self, ops: &[Self::Operation]) -> Self::Requested;

    /// Receives measured result.
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;

    /// Sends and receives.
    fn send_receive(&mut self, ops: &[Self::Operation], buf: &mut Self::Buffer) -> Self::Response {
        self.send(ops);
        self.receive(buf)
    }

    /// Make new buffer for receiving result.
    fn make_buffer(&self) -> Self::Buffer;

    /// Make new operations vec.
    fn opsvec(&self) -> OpsVec<Self> {
        OpsVec::new()
    }
}

use std::fmt::Debug;
pub mod gates;
pub mod operations;

pub use gates::{PauliGate, HGate, SGate, TGate, CXGate};
pub use operations::{Operation, OpsVec};

pub trait Layer {
    type Qubit: Debug;
    type Slot: Debug;
    type Buffer;
    type Requested;
    type Response;

    fn send(&mut self, ops: &[Operation<Self>]) -> Self::Requested;
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;
    fn send_receive(&mut self, ops: &[Operation<Self>], buf: &mut Self::Buffer) -> Self::Response;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

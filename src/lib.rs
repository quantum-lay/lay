use std::fmt::Debug;
pub mod gates;
pub mod operations;

pub trait Layer<Operation = operations::Operation<Self>> {
    type Qubit: Debug;
    type Slot: Debug;
    type Buffer;
    type Requested;
    type Response;

    fn send(&mut self, ops: &[Operation]) -> Self::Requested;
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;
    fn send_receive(&mut self, ops: &[Operation], buf: &mut Self::Buffer) -> Self::Response;
}

pub use gates::{PauliGate, HGate, SGate, TGate, CXGate};
pub use operations::{Operation, OpsVec};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

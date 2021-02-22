//! An interface for quantum computing.
//!
//! Lay provides interface for sending and receiving operations for
//! quantum computer hardwares or simulators.

pub mod gates;
pub mod operations;
pub mod convert;
pub mod inject;

pub use gates::{PauliGate, HGate, SGate, TGate, CXGate};
pub use operations::OpsVec;

mod layer;
pub use layer::Layer;

mod measured;
pub use measured::Measured;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

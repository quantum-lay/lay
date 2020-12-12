pub mod gates;
pub mod operations;

pub trait Layer {
    type Qubit;
    type Slot;
    type Buffer;
    type Requested;
    type Response;
    type ReqRes;

    fn initialize(&mut self);
    fn measure(&mut self, q: Self::Qubit, ch: Self::Slot);

    fn send(&mut self) -> Self::Requested;
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;
    fn send_receive(&mut self, buf: &mut Self::Buffer) -> Self::ReqRes;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

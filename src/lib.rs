pub mod gates;

// これは特に必須ではないのでは?
pub trait Operations {
    type Qubit;
    type Slot;
    fn initialize(&mut self);
    fn measure(&mut self, q: Self::Qubit, ch: Self::Slot);
}

// これ、厳しいんじゃないか。
// 何をもらってくるのがいいか?
pub trait Layer {
    type Operations;
    type Buffer; // 何らかのトレイトにしたい
    type Requested;
    type Response;
    type ReqRes;
    fn send(&mut self, ops: &Self::Operations) -> Self::Requested;
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;
    fn send_receive(&mut self, ops: &Self::Operations, buf: &mut Self::Buffer) -> Self::ReqRes;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

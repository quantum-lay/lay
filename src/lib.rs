pub mod gates;

// 何をもらってくるのがいいか?
pub trait Layer {
    type Qubit;
    type Slot;
    type Receive; // 何らかのトレイトにしたい
    fn initialize(&mut self);
    fn send(&mut self) {}
    // 速度考えるなら、receiveのバッファは引数渡しした方がよくないか?
    fn receive(&mut self) -> Self::Receive; // 名前これでいいのか?
    fn measure(&mut self, q: Self::Qubit, ch: Self::Slot);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

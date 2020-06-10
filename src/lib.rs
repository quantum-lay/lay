pub type Qubit = u32;

pub mod gates;

// 何をもらってくるのがいいか?
pub trait Layer {
    type Receive; // 何らかのトレイトにしたい
    fn initialize(&mut self);
    fn receive(&mut self) -> Self::Receive; // 名前これでいいのか?
    // measure関連:
    // - 測るだけ測って結果見ないやつ←下と混ぜられそう
    // - 測るけど結果は後で取るやつ
    // - 測って結果を待つやつ←いる?
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

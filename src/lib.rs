pub type Qubit = u32;

pub mod gates;

pub trait Laysur {
    fn initialize(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod gates;
pub mod operations;
pub mod convert;
pub mod inspect;

use num_traits::cast::{NumCast, cast};

pub use gates::{PauliGate, HGate, SGate, TGate, CXGate};
pub use operations::OpsVec;

pub trait Layer {
    type Operation;
    type Qubit;
    type Slot;
    type Buffer: Measured<Slot=Self::Slot>;
    type Requested;
    type Response;

    fn send(&mut self, ops: &[Self::Operation]) -> Self::Requested;
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;
    fn send_receive(&mut self, ops: &[Self::Operation], buf: &mut Self::Buffer) -> Self::Response;
    fn make_buffer(&self) -> Self::Buffer;
}

pub trait Measured {
    type Slot;

    fn get(&self, n: Self::Slot) -> bool;

    fn get_range_u8(&self, start: usize, stop: usize) -> u8
        where Self::Slot : NumCast
    {
        assert!(start <= stop && stop - start <= 8, "Invalid range.");
        let mut result = 0;
        for i in start..stop {
            result |= (self.get(cast(i).unwrap()) as u8) << (i - start);
        }
        result
    }

    fn get_range_u16(&self, start: usize, stop: usize) -> u16
        where Self::Slot : NumCast
    {
        assert!(start <= stop && stop - start <= 16, "Invalid range.");
        if stop - start <= 8 {
            self.get_range_u8(start, stop) as u16
        } else {
            self.get_range_u8(start, start + 8) as u16 | ((self.get_range_u8(start + 8, stop) as u16) << 8)
        }
    }

    fn get_range_u32(&self, start: usize, stop: usize) -> u32
        where Self::Slot : NumCast
    {
        assert!(start <= stop && stop - start <= 32, "Invalid range.");
        if stop - start <= 16 {
            self.get_range_u16(start, stop) as u32
        } else {
            self.get_range_u16(start, start + 16) as u32 | ((self.get_range_u16(start + 16, stop) as u32) << 16)
        }
    }

    fn get_range_u64(&self, start: usize, stop: usize) -> u64
        where Self::Slot : NumCast
    {
        assert!(start <= stop && stop - start <= 64, "Invalid range.");
        if stop - start <= 32 {
            self.get_range_u32(start, stop) as u64
        } else {
            self.get_range_u32(start, start + 32) as u64 | ((self.get_range_u32(start + 32, stop) as u64) << 32)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

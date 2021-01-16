use std::fmt::Debug;
pub mod gates;
pub mod operations;

pub trait Layer<Operation = operations::Operation<Self>> {
    type Qubit: Debug;
    type Slot: Debug;
    type Buffer;
    type Requested;
    type Response: Measured<Slot=Self::Slot>;

    fn send(&mut self, ops: &[Operation]) -> Self::Requested;
    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response;
    fn send_receive(&mut self, ops: &[Operation], buf: &mut Self::Buffer) -> Self::Response;
}

pub trait Measured {
    type Slot;

    fn get(&self, n: Self::Slot) -> bool;

    fn get_range_u8(&self, start: usize, stop: usize) -> u8
        where Self::Slot : From<usize>
    {
        assert!(start <= stop && stop - start <= 8, "Invalid range.");
        let mut result = 0;
        for i in start..stop {
            result |= (self.get(i.into()) as u8) << (i - start);
        }
        result
    }

    fn get_range_u16(&self, start: usize, stop: usize) -> u16
        where Self::Slot : From<usize>
    {
        assert!(start <= stop && stop - start <= 16, "Invalid range.");
        if stop - start <= 8 {
            self.get_range_u8(start, stop) as u16
        } else {
            self.get_range_u8(start, start + 8) as u16 | ((self.get_range_u8(start + 8, stop) as u16) << 8)
        }
    }

    fn get_range_u32(&self, start: usize, stop: usize) -> u32
        where Self::Slot : From<usize>
    {
        assert!(start <= stop && stop - start <= 32, "Invalid range.");
        if stop - start <= 16 {
            self.get_range_u16(start, stop) as u32
        } else {
            self.get_range_u16(start, start + 16) as u32 | ((self.get_range_u16(start + 16, stop) as u32) << 16)
        }
    }

    fn get_range_u64(&self, start: usize, stop: usize) -> u64
        where Self::Slot : From<usize>
    {
        assert!(start <= stop && stop - start <= 64, "Invalid range.");
        if stop - start <= 32 {
            self.get_range_u32(start, stop) as u64
        } else {
            self.get_range_u32(start, start + 32) as u64 | ((self.get_range_u32(start + 32, stop) as u64) << 32)
        }
    }
}

impl<T: Measured, E> Measured for Result<T, E> {
    type Slot = <T as Measured>::Slot;

    fn get(&self, n: Self::Slot) -> bool {
        self.as_ref().map(|ok| ok.get(n)).unwrap_or(false)
    }

    fn get_range_u8(&self, start: usize, stop: usize) -> u8
        where Self::Slot : From<usize>
    {
        self.as_ref().map(|ok| ok.get_range_u8(start, stop)).unwrap_or(0)
    }

    fn get_range_u16(&self, start: usize, stop: usize) -> u16
        where Self::Slot : From<usize>
    {
        self.as_ref().map(|ok| ok.get_range_u16(start, stop)).unwrap_or(0)
    }

    fn get_range_u32(&self, start: usize, stop: usize) -> u32
        where Self::Slot : From<usize>
    {
        self.as_ref().map(|ok| ok.get_range_u32(start, stop)).unwrap_or(0)
    }

    fn get_range_u64(&self, start: usize, stop: usize) -> u64
        where Self::Slot : From<usize>
    {
        self.as_ref().map(|ok| ok.get_range_u64(start, stop)).unwrap_or(0)
    }
}

// pub trait Measured<Slot: From<usize>>{}
pub use gates::{PauliGate, HGate, SGate, TGate, CXGate};
pub use operations::{Operation, OpsVec};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use num_traits::cast::{NumCast, cast};

/// Gets the measured result.
pub trait Measured {
    type Slot;

    /// Gets a single measured result.
    fn get(&self, n: Self::Slot) -> bool;

    /// Gets sequential measured result as u8.
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

    /// Gets sequential measured result as u16.
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

    /// Gets sequential measured result as u32.
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

    /// Gets sequential measured result as u64.
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

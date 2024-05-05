#[derive(Clone, Copy)]
pub struct SizedInteger<const N: usize>(pub u64);

impl<const N: usize> core::fmt::Debug for SizedInteger<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

#[derive(Debug)]
pub struct NumberTooLarge;

impl<const N: usize> TryFrom<u32> for SizedInteger<N> {
    type Error = NumberTooLarge;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (value.leading_zeros() as usize) < 32 - N {
            return Err(NumberTooLarge);
        }
        Ok(Self(value as u64))
    }
}

impl<const N: usize> From<SizedInteger<N>> for u32 {
    fn from(value: SizedInteger<N>) -> Self {
        value.0 as u32
    }
}

impl<const N: usize> TryFrom<u64> for SizedInteger<N> {
    type Error = NumberTooLarge;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if (value.leading_zeros() as usize) < 64 - N {
            return Err(NumberTooLarge);
        }
        Ok(Self(value))
    }
}

impl<const N: usize> From<SizedInteger<N>> for u64 {
    fn from(value: SizedInteger<N>) -> Self {
        value.0
    }
}

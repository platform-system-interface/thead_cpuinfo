use crate::types::SizedInteger;
use bit_field::BitField;

pub struct Presence(pub bool);

impl core::fmt::Debug for Presence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", if self.0 { "yes" } else { "no" })
    }
}

pub struct CacheSize(pub SizedInteger<4>);

impl From<CacheSize> for u64 {
    fn from(value: CacheSize) -> Self {
        value.0.into()
    }
}

impl core::fmt::Debug for CacheSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut r = 0u64;
        r.set_bits(0..=3, self.0.into());
        write!(f, "{}K", 2u32.pow(r as u32 - 1))
    }
}

pub struct MMUSize(pub SizedInteger<4>);

impl From<MMUSize> for u64 {
    fn from(value: MMUSize) -> Self {
        value.0.into()
    }
}

impl core::fmt::Debug for MMUSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut r = 0u64;
        r.set_bits(0..=3, self.0.into());
        write!(f, "{} JTLB items", 2u32.pow(r as u32))
    }
}

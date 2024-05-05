use crate::ext_types::{CacheSize, MMUSize, Presence};
use crate::types::SizedInteger;
use bit_field::BitField;

#[derive(Debug)]
struct THeadCPUID0 {
    index: SizedInteger<4>,
    arch: SizedInteger<2>,
    family: SizedInteger<4>,
    class: SizedInteger<4>,
    model: SizedInteger<10>,
    isa_revision: SizedInteger<5>,
    version: SizedInteger<3>,
}

impl From<u32> for THeadCPUID0 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            arch: value.get_bits(26..=27).try_into().unwrap(),
            family: value.get_bits(22..=25).try_into().unwrap(),
            class: value.get_bits(18..=21).try_into().unwrap(),
            model: value.get_bits(8..=17).try_into().unwrap(),
            isa_revision: value.get_bits(3..=7).try_into().unwrap(),
            version: value.get_bits(0..=2).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
struct THeadCPUID1 {
    index: SizedInteger<4>,
    revision: SizedInteger<4>,
    sub_revision: SizedInteger<6>,
    patch_revision: SizedInteger<6>,
    product_id: SizedInteger<12>,
}

impl From<u32> for THeadCPUID1 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            revision: value.get_bits(24..=27).try_into().unwrap(),
            sub_revision: value.get_bits(18..=23).try_into().unwrap(),
            patch_revision: value.get_bits(12..=17).try_into().unwrap(),
            product_id: value.get_bits(0..=11).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
struct THeadCPUID2 {
    index: SizedInteger<4>,
    bus0: SizedInteger<4>,
    bus1: SizedInteger<4>,
    interrupt_controller: Presence,
    _unknown: Presence,
    ctim: SizedInteger<2>,
    coprocessor: SizedInteger<16>,
}

impl From<u32> for THeadCPUID2 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            bus0: value.get_bits(24..=27).try_into().unwrap(),
            bus1: value.get_bits(20..=23).try_into().unwrap(),
            interrupt_controller: Presence(value.get_bit(19)),
            _unknown: Presence(value.get_bit(18)),
            ctim: value.get_bits(16..=17).try_into().unwrap(),
            coprocessor: value.get_bits(0..=15).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
struct THeadCPUID3 {
    index: SizedInteger<4>,
    _unused: SizedInteger<3>,
    ibp: SizedInteger<3>,
    btb: SizedInteger<3>,
    bht: SizedInteger<3>,
    dspm: SizedInteger<4>,
    ispm: SizedInteger<4>,
    dcache: CacheSize,
    icache: CacheSize,
}

impl From<u32> for THeadCPUID3 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            _unused: value.get_bits(25..=27).try_into().unwrap(),
            ibp: value.get_bits(22..=24).try_into().unwrap(),
            btb: value.get_bits(19..=21).try_into().unwrap(),
            bht: value.get_bits(16..=18).try_into().unwrap(),
            dspm: value.get_bits(12..=15).try_into().unwrap(),
            ispm: value.get_bits(8..=11).try_into().unwrap(),
            dcache: CacheSize(value.get_bits(4..=7).try_into().unwrap()),
            icache: CacheSize(value.get_bits(0..=3).try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
struct THeadCPUID4 {
    index: SizedInteger<4>,
    icache_way_info: SizedInteger<2>,
    icache_line_size: SizedInteger<2>,
    icache_ecc: SizedInteger<2>,
    dcache_way_info: SizedInteger<2>,
    dcache_line_size: SizedInteger<2>,
    dcache_ecc: SizedInteger<2>,
    _unused: SizedInteger<4>,
    l2cache_way_info: SizedInteger<3>,
    l2cache_line_size: SizedInteger<2>,
    l2cache_ecc: SizedInteger<2>,
    l2cache_size: SizedInteger<4>,
}

impl From<u32> for THeadCPUID4 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            icache_way_info: value.get_bits(26..=27).try_into().unwrap(),
            icache_line_size: value.get_bits(24..=25).try_into().unwrap(),
            icache_ecc: value.get_bits(22..=23).try_into().unwrap(),
            dcache_way_info: value.get_bits(20..=21).try_into().unwrap(),
            dcache_line_size: value.get_bits(18..=19).try_into().unwrap(),
            dcache_ecc: value.get_bits(16..=17).try_into().unwrap(),
            _unused: value.get_bits(12..=15).try_into().unwrap(),
            l2cache_way_info: value.get_bits(9..=11).try_into().unwrap(),
            l2cache_line_size: value.get_bits(7..=8).try_into().unwrap(),
            l2cache_ecc: value.get_bits(4..=6).try_into().unwrap(),
            l2cache_size: value.get_bits(0..=3).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
struct THeadCPUID5 {
    index: SizedInteger<4>,
    _unused: SizedInteger<24>,
    secondary_interface: Presence,
    core_number: SizedInteger<3>,
}

impl From<u32> for THeadCPUID5 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            _unused: value.get_bits(4..=27).try_into().unwrap(),
            secondary_interface: Presence(value.get_bit(3)),
            core_number: value.get_bits(0..=2).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
struct THeadCPUID6 {
    index: SizedInteger<4>,
    _unused: SizedInteger<16>,
    mmu_tlb: MMUSize,
    mgu_zone_size: SizedInteger<4>,
    mgu_zone_num: SizedInteger<4>,
}

impl From<u32> for THeadCPUID6 {
    fn from(value: u32) -> Self {
        Self {
            index: value.get_bits(28..=31).try_into().unwrap(),
            _unused: value.get_bits(12..=27).try_into().unwrap(),
            mmu_tlb: MMUSize(value.get_bits(8..=11).try_into().unwrap()),
            mgu_zone_size: value.get_bits(4..=7).try_into().unwrap(),
            mgu_zone_num: value.get_bits(0..=3).try_into().unwrap(),
        }
    }
}

pub fn print_cpuid(id_array: [u32; 7]) {
    for id in id_array {
        let index = id >> 28;
        match index {
            0 => println!("{:#?}", THeadCPUID0::from(id)),
            1 => println!("{:#?}", THeadCPUID1::from(id)),
            2 => println!("{:#?}", THeadCPUID2::from(id)),
            3 => println!("{:#?}", THeadCPUID3::from(id)),
            4 => println!("{:#?}", THeadCPUID4::from(id)),
            5 => println!("{:#?}", THeadCPUID5::from(id)),
            6 => println!("{:#?}", THeadCPUID6::from(id)),
            // This should never occur.
            _ => println!("  undefined index"),
        }
    }
}

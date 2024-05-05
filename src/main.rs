mod cpuid;
mod ext_types;
mod types;

// NOTE: The following fixtures were obtained from a Milk-V Duo S (SG200x).
// The cores on this SoC are called "C906B" ("big") and "C906L" ("little").

const MCPUID_C906B: [u32; 7] = [
    0x0910090d, 0x12046000, 0x260c0001, 0x30030076, 0x42180000, 0x50000000, 0x60000853,
];

const MCPUID_C906L: [u32; 7] = [
    0x0910010d, 0x12046000, 0x260c0001, 0x30030054, 0x42180000, 0x50000000, 0x60000753,
];

fn main() {
    println!("MCPUID_C906B");
    cpuid::print_cpuid(MCPUID_C906B);
    println!("------");
    println!("MCPUID_C906L");
    cpuid::print_cpuid(MCPUID_C906L);
}

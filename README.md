# T-Head cpuinfo

This is a simple CPU information decoder for T-Head cores.

Detailed information is not available regarding model etc. (`CPUINFO0`).

Per the [C906 manual](https://github.com/T-head-Semi/openc906/tree/main/doc):

> The machine mode processor model register (MCPUID) stores the processor
> model information. Its reset value is determined by the product itself and
> complies with the Pingtouge product definition specifications to facilitate
> software identification. By continuously reading the MCPUID register, up to
> 7 different return values can be obtained to represent C906 product
> information, as shown in Figure ??.

(translated by Google)

To read out the register, RISC-V CSR `0xfc0`:

```rs
/// T-Head CPU model register
fn print_cpuid() {
    let mut id: u32;
    for i in 0..7 {
        unsafe { asm!("csrr {}, 0xfc0", out(reg) id) };
        println!("MCPUID {i}: {id:08x}");
    }
}
```

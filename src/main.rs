extern crate raw_cpuid;

use raw_cpuid::*;

const HYPER_V_ENLIGHTMENT_1: u32 = 0x40000003;
const HYPER_V_ENLIGHTMENT_2: u32 = 0x40000004;

fn main() {
    let cpuid_1 : CpuIdResult = cpuid!(HYPER_V_ENLIGHTMENT_1);
    let cpuid_2 : CpuIdResult = cpuid!(HYPER_V_ENLIGHTMENT_2);

    println!(
        "CPUID({:#x}):\teax - {:#b}\tebx - {:#b}\tecx - {:#b}\tedx - {:#b}",
        HYPER_V_ENLIGHTMENT_1, cpuid_1.eax, cpuid_1.ebx, cpuid_1.ecx, cpuid_1.edx
    );

    println!(
        "CPUID({:#x}):\teax - {:#b}\tebx - {:#b}\tecx - {:#b}\tedx - {:#b}",
        HYPER_V_ENLIGHTMENT_2, cpuid_2.eax, cpuid_2.ebx, cpuid_2.ecx, cpuid_2.edx
    );
}

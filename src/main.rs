#[cfg(any(target_arch="x86", target_arch="x86_64"))]
mod x86;
#[cfg(target_arch = "arm")]
mod arm;

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
fn main() {
    let cpu_name = x86::x86::x86_cpuid();
    println!("CPU Manufacturer ID: {}", cpu_name);
}

#[cfg(target_arch = "arm")]
fn main() {
    let rid = arm::arm::arm_rid();
    println!("Processor Revision ID (RID): 0x{:x}", rid);
}

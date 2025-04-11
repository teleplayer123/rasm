use std::arch::asm;

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
fn main() {
    let mut buf = [0_u8; 12];
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi+4], edx",
            "mov [rdi+8], ecx",
            "pop rbx",
            in("rdi") buf.as_mut_ptr(),
            //select cpuid 0
            inout("eax") 0 => _,
            //clean up cpuid clobbers
            out("ecx") _,
            out("edx") _,
        );
    }

    let cpu_name = core::str::from_utf8(&buf).unwrap();
    println!("CPU Manufacturer ID: {}", cpu_name);
}

#[cfg(target_arch = "arm")]
use core::arch::assemble;
#[cfg(target_arch = "arm")]
fn main() {
    let mut pid = 0;
    let mut rid = 0;

    // Get processor ID (PID)
    unsafe {
        core::arch::asm!("getreg r0, r2"
             : "={r0}"(pid)
             : "r2"
             :: "volatile");
    }
    println!("Processor ID (PID): 0x{:x}", pid);

    // Get processor revision ID (RID)
    unsafe {
        core::arch::asm!("getreg r2, r1"
             : "={r2}"(rid)
             : "r1"
             :: "volatile");
    }
    println!("Processor Revision ID (RID): 0x{:x}", rid);
}

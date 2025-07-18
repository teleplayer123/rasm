pub mod x86 {
    use std::arch::asm;
    use std::env;

    pub fn x86_cpuid() -> String{
            let mut buf = [0_u8; 12];
        let mut arch = match env::var("TARGET_ARCH") {
            Ok(arch) => arch,
            Err(_) => "Not Linux".to_string(),
        };
        if arch == "Not Linux" {
            arch = match env::var("PROCESSOR_ARCHITECTURE") {
                Ok(arch) => arch,
                Err(_) => "Unknow Arch".to_string(),
            };
        }
        println!("Arch: {}", arch);
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
        cpu_name.to_string()
    }
}
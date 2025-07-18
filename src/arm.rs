pub mod arm {
    use core::arch::asm;

    pub fn arm_rid() -> u32 {
        let mut rid: u32 = 0;
        unsafe {
            asm!(
                "mrs {rid}, MIDR_EL1",
                rid = out(reg) rid,
            );
        }
        rid
    }
}
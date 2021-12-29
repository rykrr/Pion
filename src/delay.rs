pub fn delay(count: u32) {
    let mut _x: u32 = count;
    unsafe {
        core::arch::asm!(
            "1:",
            "   subs {0:x}, {0:x}, #1",
            "   bne 1b",
            inout(reg) _x
        );
    }
}

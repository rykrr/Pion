#![no_std]
#![no_main]

#![allow(unused_imports)]
#![allow(dead_code)]

use core::ptr;

mod delay; use delay::*;
mod io;
use io::uart::*;

extern "C" { fn _start(); }
core::arch::global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart_init();
    puts(b"Hello World!\n");
    puts(b"This was a triumph\n");
    puts(b"I'm making a note here; \"Huge success\"\n");
    halt();
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

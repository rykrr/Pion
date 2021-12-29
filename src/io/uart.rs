use crate::delay;
use super::address::*;
use super::mmio::*;
use super::bits::*;

registers! {
    name GPIO, base MMIO_BASE, offset 0x20_0000;
    GPPUD,      0x94;
    GPPUDCLK0,  0x98
}

registers! {
    prefix UART, base MMIO_BASE, offset 0x20_1000;
    DR,     0x00; // Data Register
    RSRECR, 0x04; // No Description
    FR,     0x18; // Flag Register
    ILPR,   0x20; // Unused
    IBRD,   0x24; // Integer Baud Rate Divisor
    FBRD,   0x28; // Fractional Baud Rate Divisor
    LCRH,   0x2C; // Line Control Register
    CR,     0x30; // Control Register
    IFLS,   0x34; // Interrupt FIFO Level Select Register
    IMSC,   0x38; // Interrupt Mask Set Clear Register
    RIS,    0x3C; // Raw Interrupt Status Register
    MIS,    0x40; // Masked Interrupt Status Register
    ICR,    0x44; // Interrupt Clear Register
    DMACR,  0x48; // DMA Control Register
    ITCR,   0x80; // Test Control Register
    ITIP,   0x84; // Integration Test Input Register
    ITOP,   0x88; // Integration Test Output Registert
    ITR,    0x8C  // Test Data Register
}

// Based on https://wiki.osdev.org/Raspberry_Pi_Bare_Bones#Writing_a_kernel_in_C
pub fn uart_init() {
    UART_CR.write(0);
    GPPUD.write(0);
    delay(150);

    GPPUDCLK0.write(bits!(14, 15));
    delay(150);

    GPPUDCLK0.write(0);
    UART_ICR.write(0x7FF);

    UART_IBRD.write(1);
    UART_FBRD.write(40);

    UART_LCRH.write(bits!(4, 5, 6));
    UART_IMSC.write(bits!(1, 4, 5, 6, 7, 8, 9, 10));
    UART_CR.write(bits!(0, 8, 9));
}

pub fn putc(c: u8) {
    delay(0x0400_0000);
    while UART_FR.read() & bits!(5) == bits!(5) {}
    UART_DR.write(c.into())
}

pub fn puts(s: &[u8]) {
    for c in s { putc(*c) }
}

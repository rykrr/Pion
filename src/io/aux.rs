use super::address::*;
use super::mmio::*;

registers! {
    prefix AUX, base MMIO_BASE, offset 0x21_5000;
    IRQ,      0x00;
    ENABLES,  0x04
}

registers! {
    prefix AUX_MU, base AUX_BASE, offset 0;
    IO_REG,   0x40;
    IER_REG,  0x44;
    IIR_REG,  0x48;
    LCR_REG,  0x4C;
    MCR_REG,  0x50;
    LSR_REG,  0x54;
    MSR_REG,  0x58;
    SCRATCH,  0x5C;
    CNTL_REG, 0x60;
    STAT_REG, 0x64;
    BAUD_REG, 0x68
}

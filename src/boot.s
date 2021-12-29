.section .text.boot
.globl _start
_start:
    // Place stack before _start
    ldr     x5, =_start
    mov     sp, x5

    // Only use the main core
    mrs     x0, mpidr_el1
    and     x0, x0, #3
    cbz     x0, _clear_bss
    b       _halt

_clear_bss:
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
1:  cbz     w6, _main
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 1b
;   b       _main

_main:
    bl      kernel_main
;   b       _halt

_halt:
    wfi
    b       _halt

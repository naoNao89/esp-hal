/* Minimal reset path for ESP32-P4 pre-v3 bootstrap.
 * Sets stack, clears BSS, jumps to rust_main. No PAC/HAL/clocks.
 *
 * Flash-mapped layout:
 *   DROM @ 0x40000020 : esp_app_desc (Rust) + p4v1_flash_rodata
 *   IROM @ 0x40010020 : p4v1_flash_probe (this file)
 */

.section .text.reset, "ax"
.global _start
.type _start, @function
_start:
    csrci mstatus, 8
    la sp, _stack_top

    la t0, _sbss
    la t1, _ebss
1:
    bgeu t0, t1, 2f
    sw zero, 0(t0)
    addi t0, t0, 4
    j 1b
2:
    call rust_main
3:
    j 3b
.size _start, . - _start

/*
 * Flash-mapped executable probe in IROM @ 0x40010020.
 * Loads four bytes from p4v1_flash_rodata (DROM) and returns 1 if "P4V1".
 */
.section .p4v1.flash.text, "ax"
.align 4
.global p4v1_flash_probe
.type p4v1_flash_probe, @function
p4v1_flash_probe:
    la a0, p4v1_flash_rodata

    lbu t0, 0(a0)
    lbu t1, 1(a0)
    lbu t2, 2(a0)
    lbu t3, 3(a0)

    li t4, 0x50
    bne t0, t4, 9f
    li t4, 0x34
    bne t1, t4, 9f
    li t4, 0x56
    bne t2, t4, 9f
    li t4, 0x31
    bne t3, t4, 9f

    li a0, 1
    ret
9:
    li a0, 0
    ret
.size p4v1_flash_probe, . - p4v1_flash_probe

/*
 * Flash-mapped DROM probe bytes after esp_app_desc.
 * Expected address: 0x40000120 (0x40000020 + 0x100).
 */
.section .p4v1.flash.rodata, "a"
.align 4
.global p4v1_flash_rodata
.type p4v1_flash_rodata, @object
p4v1_flash_rodata:
    .byte 0x50, 0x34, 0x56, 0x31
    .align 4
.size p4v1_flash_rodata, . - p4v1_flash_rodata

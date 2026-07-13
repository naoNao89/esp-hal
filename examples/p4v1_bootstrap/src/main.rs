//! ESP32-P4 rev v1.3 monorepo-native bootstrap example.
//!
//! Reproduces the hardware-proven markers:
//! - `P4V1_RUST_ENTRY_OK`
//! - `P4V1_FLASH_XIP_OK`
//!
//! Depends on support crates only (no `esp-hal`, no PAC).

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("reset.S"));

// Application descriptor via support crate.
// Boot-critical fields match the proven OV5647 rev1.3 image:
//   min_efuse_blk_rev_full = 0
//   max_efuse_blk_rev_full = 199
//   mmu_page_size = 65536 (stored as log2 = 16)
esp_bootloader_esp_idf::esp_app_desc!(
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_NAME"),
    "00:00:00",
    "Jul 13 2026",
    "v5.5.4-1219-g897cf407aa7",
    65536u32,
    0u16,
    199u16,
    0u32
);

unsafe extern "C" {
    /// Flash-mapped probe in `.p4v1.flash.text` (IROM @ 0x40010020).
    fn p4v1_flash_probe() -> u32;
}

#[unsafe(link_section = ".data")]
#[used]
static MARKER_ENTRY: [u8; 20] = *b"P4V1_RUST_ENTRY_OK\r\n";

#[unsafe(link_section = ".data")]
#[used]
static MARKER_XIP_OK: [u8; 19] = *b"P4V1_FLASH_XIP_OK\r\n";

#[unsafe(link_section = ".data")]
#[used]
static MARKER_XIP_BAD: [u8; 20] = *b"P4V1_FLASH_XIP_BAD\r\n";

fn write_marker(bytes: &[u8]) {
    // ROM UART via esp-rom-sys (pre-v3 map selected by esp32p4v1).
    esp_rom_sys::rom::ets_install_uart_printf();
    for &byte in bytes {
        if byte == 0 {
            break;
        }
        esp_rom_sys::rom::ets_write_char_uart(byte);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_marker(&MARKER_ENTRY);

    // SAFETY: p4v1_flash_probe is a real flash-resident function that returns.
    let ok = unsafe { p4v1_flash_probe() };
    if ok == 1 {
        write_marker(&MARKER_XIP_OK);
    } else {
        write_marker(&MARKER_XIP_BAD);
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

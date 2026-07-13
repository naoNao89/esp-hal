/* Pre-v3 ESP32-P4 memory regions for the Rust-entry bootstrap.
 *
 * Low SRAM:
 *   ESP-IDF 897cf407aa70586c15cf8c6b3fe9458066cf7fce
 *   memory.ld.in CONFIG_ESP32P4_SELECTS_REV_LESS_V3
 *     SRAM_LOW 0x4FF00000..0x4FF2CBD0
 *
 * Flash-mapped topology:
 *   Proven OV5647 build-waveshare-serial image places app_desc in the first
 *   DROM/IROM image segment. Bootloader reads esp_app_desc_t from segment 0.
 *
 *   Segment 0 (DROM): app_desc @ 0x40000020 (256 bytes) + probe rodata
 *   Segment 1 (IROM): executable flash probe @ 0x40010020
 *
 *   MMU page size 64 KiB; congruence offset 0x20.
 *   Reference min_efuse_blk_rev_full=0, max=199, mmu_page_size log2=16.
 */

MEMORY
{
    SRAM_LOW (RWX) : ORIGIN = 0x4FF00000, LENGTH = 0x2CBD0

    /* First mapped image segment: descriptor + DROM probe data. */
    FLASH_DROM (R) : ORIGIN = 0x40000020, LENGTH = 0x200

    /* Second mapped image segment: executable flash probe. */
    FLASH_IROM (RX) : ORIGIN = 0x40010020, LENGTH = 0x100
}

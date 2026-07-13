/* Pre-v3 ESP32-P4 bootstrap linker script (hardware-proven layout).
 *
 * Source of truth:
 *   /Users/henri89/Documents/GitHub/esp32p4-v1-bootstrap
 *   ESP-IDF 897cf407aa70586c15cf8c6b3fe9458066cf7fce
 *
 * Bootloader contract:
 *   - exactly two flash-mapped segments in 0x40000000..0x44000000
 *   - segment 0 begins with valid esp_app_desc_t (magic 0xABCD5432)
 *   - second mapped segment holds executable IROM probe
 *
 * Pre-v3 ROM symbols come from esp-rom-sys/esp32p4v1 (not ECO5).
 */

INCLUDE memory.x

ENTRY(_start)

SECTIONS
{
    /* Executable path remains in low SRAM. */
    .text ORIGIN(SRAM_LOW) :
    {
        KEEP(*(.text.reset));
        *(.text .text.*);
        *(.rodata .rodata.*);
        . = ALIGN(4);
        _etext = .;
    } > SRAM_LOW

    .data : ALIGN(4)
    {
        _sdata = .;
        *(.data .data.*);
        *(.sdata .sdata.*);
        . = ALIGN(4);
        _edata = .;
    } > SRAM_LOW

    .bss (NOLOAD) : ALIGN(4)
    {
        _sbss = .;
        *(.bss .bss.*);
        *(.sbss .sbss.*);
        *(COMMON);
        . = ALIGN(4);
        _ebss = .;
    } > SRAM_LOW

    /*
     * DROM segment 0:
     *   [0x000..0x0ff] esp_app_desc from esp-bootloader-esp-idf::esp_app_desc!
     *   [0x100..]      p4v1_flash_rodata probe bytes
     */
    .p4v1_flash_appdesc ORIGIN(FLASH_DROM) :
    {
        /* Support-crate macro places the descriptor in .flash.appdesc */
        KEEP(*(.flash.appdesc));
        KEEP(*(.p4v1.flash.appdesc));
        . = ALIGN(16);
    } > FLASH_DROM

    .p4v1_flash_rodata :
    {
        KEEP(*(.p4v1.flash.rodata));
        . = ALIGN(4);
    } > FLASH_DROM

    /* IROM segment: executable flash probe */
    .p4v1_flash_text ORIGIN(FLASH_IROM) :
    {
        KEEP(*(.p4v1.flash.text));
        . = ALIGN(4);
    } > FLASH_IROM

    _stack_top = ORIGIN(SRAM_LOW) + LENGTH(SRAM_LOW);
    _stack_bottom = _stack_top - 4K;

    ASSERT(_ebss <= _stack_bottom, "BSS collides with stack in pre-v3 low SRAM")
    ASSERT(SIZEOF(.p4v1_flash_appdesc) == 0x100, "esp_app_desc must be exactly 256 bytes")
}

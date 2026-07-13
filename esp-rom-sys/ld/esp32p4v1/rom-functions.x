/* Pre-v3 ESP32-P4 ROM selection for esp32p4v1.
 * Source baseline: ESP-IDF 897cf407aa70586c15cf8c6b3fe9458066cf7fce
 * components/esp_rom/esp32p4/ld/esp32p4.rom.ld (not ECO5).
 */
INCLUDE "rom/esp32p4.rom.api.ld"
INCLUDE "rom/esp32p4.rom.ld"
INCLUDE "rom/esp32p4.rom.libgcc.ld"
INCLUDE "rom/esp32p4.rom.rvfp.ld"
INCLUDE "rom/esp32p4.rom.version.ld"
INCLUDE "rom/additional.ld"

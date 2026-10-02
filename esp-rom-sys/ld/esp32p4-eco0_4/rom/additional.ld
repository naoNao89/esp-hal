/* Ref: esp-idf v6.1 esp32p4/ld/esp32p4.rom.eco0_4.libc.ld */
syscall_table_ptr = 0x4ff3ffe4;
ets_update_cpu_frequency = 0x4fc00044;
ets_printf = 0x4fc00024;
PROVIDE(ets_delay_us = 0x4fc0003c);

memset = 0x4fc00268;
memcpy = 0x4fc0026c;
memmove = 0x4fc00270;
memcmp = 0x4fc00274;
strcpy = 0x4fc00278;
strncpy = 0x4fc0027c;
strcmp = 0x4fc00280;
strncmp = 0x4fc00284;

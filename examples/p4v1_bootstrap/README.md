# ESP32-P4 rev v1.3 bootstrap (monorepo)

Standalone support-crate example that reproduces the hardware-proven markers:

```text
P4V1_RUST_ENTRY_OK
P4V1_FLASH_XIP_OK
```

## Scope

- Uses `esp-bootloader-esp-idf/esp32p4v1` for the application descriptor
- Uses `esp-rom-sys/esp32p4v1` for pre-v3 ROM UART symbols (not ECO5)
- Uses an isolated interrupt-free startup (`src/reset.S`); full `esp-riscv-rt` CLIC path is **not** enabled
- Does **not** depend on `esp-hal` or a PAC

## Build

```bash
cargo build --release
```

## Image

```bash
python -m esptool --chip esp32p4 elf2image \
  --version 3 --min-rev-full 103 --max-rev-full 199 \
  --flash_mode dio --flash_freq 80m --flash_size 16MB \
  -o target/riscv32imafc-unknown-none-elf/release/p4v1-bootstrap.bin \
  target/riscv32imafc-unknown-none-elf/release/p4v1-bootstrap
```

## Flash

```bash
python -m esptool --chip esp32p4 --port /dev/cu.usbmodem5B910470321 \
  --baud 460800 write_flash 0x10000 \
  target/riscv32imafc-unknown-none-elf/release/p4v1-bootstrap.bin
```

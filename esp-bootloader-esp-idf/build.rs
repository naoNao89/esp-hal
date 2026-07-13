use std::{env, error::Error};

use esp_config::generate_config_from_yaml_definition;
use esp_metadata_generated::{Chip, assert_unique_features};
use jiff::Timestamp;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rustc-check-cfg=cfg(embedded_test)");
    println!("cargo::rustc-check-cfg=cfg(esp32p4v1)");

    // Log and defmt are mutually exclusive features. The main technical reason is
    // that allowing both would make the exact panicking behaviour a fragile
    // implementation detail.
    assert_unique_features!("log-04", "defmt");

    let build_time = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => Timestamp::from_microsecond(val.parse::<i64>()?).unwrap(),
        Err(_) => Timestamp::now(),
    };

    let build_time_formatted = build_time.strftime("%H:%M:%S");
    let build_date_formatted = build_time.strftime("%Y-%m-%d");

    println!("cargo::rustc-env=ESP_BOOTLOADER_BUILD_TIME={build_time_formatted}");
    println!("cargo::rustc-env=ESP_BOOTLOADER_BUILD_DATE={build_date_formatted}");

    // Ensure that exactly one chip has been specified (unless the "std" feature is enabled)
    let chip = if cfg!(feature = "std") {
        None
    } else if cfg!(feature = "esp32p4v1") {
        // Pre-v3 path reuses P4 metadata for config generation only.
        // Upstream `esp32p4` feature remains the v3+ target and is exclusive.
        if std::env::var("CARGO_FEATURE_ESP32P4").is_ok() {
            panic!("esp32p4v1 is mutually exclusive with esp32p4");
        }
        println!("cargo:rustc-cfg=esp32p4v1");
        // Use Esp32p4 chip metadata for yaml defaults (MMU page size etc.).
        Some(Chip::Esp32p4)
    } else {
        Some(Chip::from_cargo_feature().unwrap())
    };

    // emit config
    println!("cargo:rerun-if-changed=./esp_config.yml");
    let cfg_yaml = std::fs::read_to_string("./esp_config.yml")
        .expect("Failed to read esp_config.yml for esp-bootloader-esp-idf");
    generate_config_from_yaml_definition(&cfg_yaml, true, true, chip).unwrap();

    Ok(())
}

use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let self_version = std::env::var("CARGO_PKG_VERSION")?;
    let self_version: Vec<&str> = self_version.split('.').collect();
    if self_version[0] != "0" || self_version[1] != "1" {
        panic!("The 'esp-rom-sys' crate is not allowed to get bumped to anything above 0.1.x");
    }

    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // Pre-v3 ESP32-P4 path: distinct from upstream esp32p4 (ECO5/v3).
    // Does not use Chip::from_cargo_feature so it does not require a PAC.
    if cfg!(feature = "esp32p4v1") {
        // Mutually exclusive with other chip features.
        let other_chips = [
            "CARGO_FEATURE_ESP32",
            "CARGO_FEATURE_ESP32C2",
            "CARGO_FEATURE_ESP32C3",
            "CARGO_FEATURE_ESP32C5",
            "CARGO_FEATURE_ESP32C6",
            "CARGO_FEATURE_ESP32C61",
            "CARGO_FEATURE_ESP32H2",
            "CARGO_FEATURE_ESP32P4",
            "CARGO_FEATURE_ESP32S2",
            "CARGO_FEATURE_ESP32S3",
        ];
        for env in other_chips {
            if std::env::var(env).is_ok() {
                panic!(
                    "esp32p4v1 is mutually exclusive with other chip features (found {env})"
                );
            }
        }

        println!("cargo:rustc-cfg=esp32p4v1");
        println!("cargo:rustc-check-cfg=cfg(esp32p4v1)");
        // ROM helper modules used by esp-bootloader-esp-idf (same as P4).
        println!("cargo:rustc-cfg=rom_crc_le");
        println!("cargo:rustc-cfg=rom_crc_be");
        println!("cargo:rustc-cfg=rom_md5_bsd");
        println!("cargo:rustc-check-cfg=cfg(rom_crc_le,rom_crc_be,rom_md5_bsd)");
        copy_dir_all("./ld/esp32p4v1/", &out)?;
        copy_dir_all("./libs/esp32p4v1/", &out)?;
        include_libs("./libs/esp32p4v1/")?;
        println!("cargo:rustc-link-lib=esp_rom_sys");
        return Ok(());
    }

    let chip = esp_metadata_generated::Chip::from_cargo_feature()?;

    // Define all necessary configuration symbols for the configured device:
    chip.define_cfgs();

    copy_dir_all(format!("./ld/{}/", chip.name()), &out)?;
    copy_dir_all(format!("./libs/{}/", chip.name()), &out)?;

    include_libs(format!("./libs/{}/", chip.name()))?;

    // exploit the fact that linkers treat an unknown library format as a linker
    // script
    println!("cargo:rustc-link-lib=esp_rom_sys");

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
    Ok(())
}

fn include_libs(path: impl AsRef<Path>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(path.as_ref())? {
        let file_name = entry?.file_name().into_string().unwrap();
        if let Some(lib_name) = file_name
            .strip_prefix("lib")
            .and_then(|f| f.strip_suffix(".a"))
        {
            println!("cargo:rustc-link-lib=static={lib_name}");
        }
    }
    Ok(())
}

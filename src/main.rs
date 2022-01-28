use std::{
    ffi::CString,
    fs,
    io::{Read, Write},
    ptr,
};

use anyhow::Result;
use esp_idf_sys::{self as _, esp}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let base_path = CString::new("/spiffs")?;
    let spiffs_conf = esp_idf_sys::esp_vfs_spiffs_conf_t {
        base_path: base_path.as_ptr(),
        partition_label: ptr::null(),
        max_files: 10,
        format_if_mount_failed: true,
    };

    esp!(unsafe { esp_idf_sys::esp_vfs_spiffs_register(&spiffs_conf) })?;

    {
        if let Ok(mut file) = fs::OpenOptions::new().read(true).open("/spiffs/logfile") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("{}", contents);
        }
    }

    {
        if let Ok(mut file) = fs::OpenOptions::new().append(true).open("/spiffs/logfile") {
            file.write_all(b"hi\n")?;
        }
    }

    println!("Hello, world!");

    Ok(())
}

#![windows_subsystem = "windows"]
use std::{env, ffi::OsStr, fs::File, os::windows::prelude::OsStrExt, path::PathBuf};

use winapi::{
    self,
    shared::minwindef::{FALSE, HKEY, MAX_PATH, TRUE},
    um::{
        winnt::REG_NOTIFY_CHANGE_LAST_SET,
        winreg::{RegNotifyChangeKeyValue, RegOpenKeyW, HKEY_CURRENT_USER},
        winuser::{
            SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETDESKWALLPAPER,
            SPI_SETDESKWALLPAPER,
        },
    },
};

fn main() {
    unsafe {
        let path = env::temp_dir().join("wallpaper.jpg");

        let file_path: Vec<u16> = OsStr::new(&path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // println!("Started!");
        if has_changed(&file_path) {
            change(&path, &file_path);
        }

        let key_path = OsStr::new(r"Control Panel\Desktop")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();

        let mut h_key: HKEY = std::ptr::null_mut();
        RegOpenKeyW(HKEY_CURRENT_USER, key_path.as_ptr(), &mut h_key);

        loop {
            RegNotifyChangeKeyValue(
                h_key,
                TRUE,
                REG_NOTIFY_CHANGE_LAST_SET,
                std::ptr::null_mut(),
                FALSE,
            );

            // println!("Changed!");
            if !has_changed(&file_path) {
                continue;
            }

            change(&path, &file_path);
        }
    }
}

unsafe fn has_changed(file_path: &Vec<u16>) -> bool {
    let mut buffer: Vec<u16> = Vec::with_capacity(MAX_PATH);

    SystemParametersInfoW(
        SPI_GETDESKWALLPAPER,
        MAX_PATH as u32,
        buffer.as_ptr() as _,
        0,
    );
    buffer.set_len(file_path.len());

    &buffer != file_path
}

unsafe fn change(path: &PathBuf, file_path: &Vec<u16>) {
    if !path.exists() {
        let mut file = File::create(path).unwrap();
        let url = "https://raw.githubusercontent.com/nuggetInc/Wallpaper/main/wallpapers/windows 11 red.jpg";
        reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

        // println!("Downloaded!");
    }

    SystemParametersInfoW(
        SPI_SETDESKWALLPAPER,
        0,
        file_path.as_ptr() as _,
        SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
    );
}

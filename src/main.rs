#![windows_subsystem = "windows"]
use std::{env, ffi::OsStr, os::windows::prelude::OsStrExt, path::PathBuf};

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
        let path = env::temp_dir().join(option_env!("WALLPAPER_PATH").unwrap_or("wallpaper.jpg"));
        // println!("{path:?}");

        let buffer: Box<[u16]> = Box::new([0; MAX_PATH]);
        let path_wide = to_wide_boxed(&path);

        // println!("Started!");
        if has_changed(&path_wide, &buffer) {
            change(&path, &path_wide);
        }

        let key_path: Box<[u16]> = to_wide_boxed(r"Control Panel\Desktop");

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
            if !has_changed(&path_wide, &buffer) {
                continue;
            }

            change(&path, &path_wide);
        }
    }
}

unsafe fn has_changed(file_path: &Box<[u16]>, buffer: &Box<[u16]>) -> bool {
    SystemParametersInfoW(
        SPI_GETDESKWALLPAPER,
        MAX_PATH as u32,
        buffer.as_ptr() as _,
        0,
    );

    buffer != file_path
}

unsafe fn change(path: &PathBuf, path_wide: &Box<[u16]>) {
    #[cfg(feature = "download")]
    if !path.exists() {
        use std::fs::File;

        let mut file = File::create(path).unwrap();
        let url = option_env!("WALLPAPER_URL").unwrap_or(
            "https://raw.githubusercontent.com/nuggetInc/Wallpaper/main/wallpapers/windows 11 red.jpg"
        );
        reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

        // println!("Downloaded!");
    }

    SystemParametersInfoW(
        SPI_SETDESKWALLPAPER,
        0,
        path_wide.as_ptr() as _,
        SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
    );
}

fn to_wide_boxed<S>(value: &S) -> Box<[u16]>
where
    S: AsRef<OsStr> + ?Sized,
{
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

// #![windows_subsystem = "windows"]
#[cfg(feature = "url")]
mod url;
#[cfg(feature = "xkcd")]
mod xkcd;

use std::{env, error::Error, ffi::OsStr, os::windows::prelude::OsStrExt, path::PathBuf};

use once_cell::sync::Lazy;
#[cfg(any(feature = "url", feature = "xkcd"))]
use reqwest::blocking::Client;
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

#[cfg(any(feature = "url", feature = "xkcd"))]
pub(crate) static CLIENT: Lazy<Client> = Lazy::new(Client::new);

static PATH: Lazy<Option<PathBuf>> =
    Lazy::new(|| option_env!("WALLPAPER_PATH").map(|path| PathBuf::from(path)));
static PATH_WIDE: Lazy<Option<Box<[u16]>>> = Lazy::new(|| PATH.as_ref().map(to_wide));

fn main() {
    let path = env::temp_dir().join(option_env!("WALLPAPER_PATH").unwrap_or("wallpaper.jpg"));
    let path_wide = to_wide(&path);

    let buffer: Box<[u16]> = Box::new([0; MAX_PATH]);

    unsafe {
        // println!("Started!");
        if has_changed(&path_wide, &buffer) {
            update();
        }

        let key_path: Box<[u16]> = to_wide(r"Control Panel\Desktop");

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
            if has_changed(&path_wide, &buffer) {
                update();
            }
        }
    }
}

unsafe fn has_changed(path_wide: &Box<[u16]>, buffer: &Box<[u16]>) -> bool {
    SystemParametersInfoW(
        SPI_GETDESKWALLPAPER,
        MAX_PATH as u32,
        buffer.as_ptr() as _,
        0,
    );

    buffer != path_wide
}

unsafe fn update() {
    #[cfg(feature = "xkcd")]
    if let Err(error) = xkcd::download() {
        eprintln!("{error}");
        eprintln!("{error:?}");
    } else {
        set_path(&xkcd::PATH_WIDE);
        return;
    }

    // Only download image if url feature is enabled
    // Sometimes a fallback if xkcd download failed
    #[cfg(feature = "url")]
    if let Err(error) = url::download() {
        eprintln!("{error}");
        eprintln!("{error:?}");
    } else {
        set_path(&url::PATH_WIDE);
        return;
    }

    // Only run if both the url feature and the xkcd feature are disabled
    if PATH.is_some() && PATH.as_ref().unwrap().exists() {
        set_path(&PATH_WIDE.as_ref().unwrap());
    }
}

unsafe fn set_path(path_wide: &Box<[u16]>) {
    SystemParametersInfoW(
        SPI_SETDESKWALLPAPER,
        0,
        path_wide.as_ptr() as _,
        SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
    );
}

pub(crate) fn to_wide<S>(value: &S) -> Box<[u16]>
where
    S: AsRef<OsStr> + ?Sized,
{
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

use std::{ffi::OsStr, os::windows::prelude::OsStrExt, time::Duration};

use winapi::{
    self,
    shared::{
        minwindef::{FALSE, HKEY, MAX_PATH, TRUE},
        windef::HWND,
    },
    um::{
        winnt::REG_NOTIFY_CHANGE_LAST_SET,
        winreg::{RegNotifyChangeKeyValue, RegOpenKeyW, HKEY_CURRENT_USER},
        winuser::{
            CreateWindowExW, SendMessageW, SystemParametersInfoW, CW_USEDEFAULT,
            SPIF_UPDATEINIFILE, SPI_GETDESKWALLPAPER, SPI_SETDESKWALLPAPER, WM_SETTINGCHANGE,
            WS_DISABLED, WS_EX_LEFT,
        },
    },
};

fn main() {
    let key_path = OsStr::new(r"Control Panel\Desktop")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let path = r"C:\users\stage\wallpaper.jpg";

    let file_path: Vec<u16> = OsStr::new(&path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let h_wnd = CreateWindowExW(
            WS_EX_LEFT,
            &0,
            &0,
            WS_DISABLED,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        let mut h_key: HKEY = std::ptr::null_mut();
        RegOpenKeyW(HKEY_CURRENT_USER, key_path.as_ptr(), &mut h_key);

        // println!("Started!");
        if has_changed(&file_path) {
            change(h_wnd, &file_path);
        }

        loop {
            RegNotifyChangeKeyValue(
                h_key,
                TRUE,
                REG_NOTIFY_CHANGE_LAST_SET,
                std::ptr::null_mut(),
                FALSE,
            );

            // println!("Changed!");
            std::thread::sleep(Duration::from_millis(10));
            if has_changed(&file_path) {
                change(h_wnd, &file_path);
            }
        }
    }
}

unsafe fn has_changed(file_path: &Vec<u16>) -> bool {
    let mut buffer: Vec<u16> = Vec::with_capacity(file_path.len());
    buffer.set_len(file_path.len());

    SystemParametersInfoW(
        SPI_GETDESKWALLPAPER,
        MAX_PATH as u32,
        buffer.as_ptr() as _,
        SPIF_UPDATEINIFILE,
    );

    return &buffer != file_path;
}

unsafe fn change(h_wnd: HWND, file_path: &Vec<u16>) {
    SystemParametersInfoW(
        SPI_SETDESKWALLPAPER,
        0,
        file_path.as_ptr() as _,
        SPIF_UPDATEINIFILE,
    );

    // println!("Updated!");
    SendMessageW(h_wnd, WM_SETTINGCHANGE, 0, 0);
}

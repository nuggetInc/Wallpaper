use std::{env, ffi::OsStr, fs::File, path::PathBuf, str::FromStr};

use once_cell::sync::Lazy;
use reqwest::{blocking::Client, Url};

use crate::{to_wide, Result};

static URL: Lazy<Url> = Lazy::new(|| {
    let url = option_env!("WALLPAPER_URL").expect(
        "No wallpaper url specified. Set wallpaper by setting the `WALLPAPER_URL` env variable.",
    );
    Url::from_str(url).unwrap()
});

static PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = env::temp_dir().join("wallpaper_url");

    let url_path = PathBuf::from(URL.path());
    let extension = url_path.extension().unwrap_or(OsStr::new(""));
    path.set_extension(extension);

    path
});
pub(crate) static PATH_WIDE: Lazy<Box<[u16]>> = Lazy::new(|| to_wide(&*PATH));

pub(crate) fn download(client: &Client) -> Result<()> {
    if PATH.exists() {
        return Ok(());
    }

    let mut response = client.get(URL.clone()).send()?;
    let mut file = File::create(&*PATH)?;
    response.copy_to(&mut file)?;

    Ok(())
}

use std::{env, fs::File, path::PathBuf};

use once_cell::sync::Lazy;
use rand::Rng;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{to_wide, Result};

static PATH: Lazy<PathBuf> = Lazy::new(|| env::temp_dir().join("wallpaper_xkcd.png"));
pub(crate) static PATH_WIDE: Lazy<Box<[u16]>> = Lazy::new(|| to_wide(&*PATH));

pub(crate) fn download(client: &Client) -> Result<()> {
    let url = format!(
        "https://xkcd.com/{}/info.0.json",
        rand::thread_rng().gen_range(1..=get_count(client)?)
    );
    let response = client.get(url).send()?;
    let text = response.text()?;

    let xkcd: Xkcd = serde_json::from_str(&text)?;

    let mut response = client.get(xkcd.img).send()?;
    let mut file = File::create(&*PATH)?;
    response.copy_to(&mut file)?;

    Ok(())
}

fn get_count(client: &Client) -> Result<usize> {
    let response = client.get("https://xkcd.com/info.0.json").send()?;
    let text = response.text()?;

    let xkcd: Xkcd = serde_json::from_str(&text)?;

    Ok(xkcd.num)
}

#[derive(Deserialize)]
struct Xkcd {
    num: usize,
    img: String,
}

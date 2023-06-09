use std::{env, error::Error, fs::File, path::PathBuf};

use once_cell::sync::Lazy;
use rand::Rng;
use serde::Deserialize;

use crate::{to_wide, CLIENT};

static PATH: Lazy<PathBuf> = Lazy::new(|| env::temp_dir().join("wallpaper_xkcd.png"));
pub(crate) static PATH_WIDE: Lazy<Box<[u16]>> = Lazy::new(|| to_wide(&*PATH));

pub(crate) fn download() -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://xkcd.com/{}/info.0.json",
        rand::thread_rng().gen_range(1..=get_count()?)
    );
    let response = CLIENT.get(url).send()?;
    let text = response.text()?;

    let xkcd: Xkcd = serde_json::from_str(&text)?;

    let mut response = CLIENT.get(xkcd.img).send()?;
    let mut file = File::create(&*PATH)?;
    response.copy_to(&mut file)?;

    Ok(())
}

fn get_count() -> Result<usize, Box<dyn Error>> {
    let response = CLIENT.get("https://xkcd.com/info.0.json").send()?;
    let text = response.text()?;

    let xkcd: Xkcd = serde_json::from_str(&text)?;

    Ok(xkcd.num)
}

#[derive(Deserialize)]
struct Xkcd {
    num: usize,
    img: String,
}

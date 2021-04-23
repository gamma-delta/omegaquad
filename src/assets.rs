#![allow(clippy::eval_order_dependence)]

use std::path::PathBuf;

use macroquad::{
    audio::{load_sound, Sound},
    prelude::{load_texture, FilterMode, Texture2D},
};
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Assets {
    pub textures: Textures,
    pub sounds: Sounds,
}

impl Assets {
    pub async fn init() -> Self {
        Self {
            textures: Textures::init().await,
            sounds: Sounds::init().await,
        }
    }
}

#[derive(Clone)]
pub struct Textures {
    pub title_banner: Texture2D,
}

impl Textures {
    async fn init() -> Self {
        Self {
            title_banner: texture("title/banner").await,
        }
    }
}

#[derive(Clone)]
pub struct Sounds {
    pub title_jingle: Sound,
}

impl Sounds {
    async fn init() -> Self {
        Self {
            title_jingle: sound("title/jingle").await,
        }
    }
}

/// Path to the assets root
static ASSETS_ROOT: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(target_arch = "wasm32") {
        PathBuf::from("../assets")
    } else if cfg!(debug_assertions) {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets"))
    } else {
        todo!("assets path for release hasn't been finalized yet ;-;")
    }
});

async fn texture(path: &str) -> Texture2D {
    let with_extension = path.to_owned() + ".png";
    let tex = load_texture(
        ASSETS_ROOT
            .join("textures")
            .join(with_extension)
            .to_string_lossy()
            .as_ref(),
    )
    .await
    .unwrap();
    tex.set_filter(FilterMode::Nearest);
    tex
}

async fn sound(path: &str) -> Sound {
    let with_extension = path.to_owned() + ".ogg";
    load_sound(
        ASSETS_ROOT
            .join("sounds")
            .join(with_extension)
            .to_string_lossy()
            .as_ref(),
    )
    .await
    .unwrap()
}

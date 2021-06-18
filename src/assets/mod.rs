#![allow(clippy::eval_order_dependence)]

use macroquad::{
    audio::{load_sound, Sound},
    miniquad::*,
    prelude::*,
};
use once_cell::sync::Lazy;

use std::path::PathBuf;

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

pub struct Textures {
    pub fonts: Fonts,

    pub title_banner: Texture2D,
    pub billboard_patch9: Texture2D,
}

impl Textures {
    async fn init() -> Self {
        Self {
            fonts: Fonts::init().await,
            title_banner: texture("title/banner").await,
            billboard_patch9: texture("ui/billboard_patch9").await,
        }
    }
}

pub struct Fonts {
    pub small: Texture2D,
    pub medium: Texture2D,
}

impl Fonts {
    async fn init() -> Self {
        Self {
            small: texture("ui/font_small").await,
            medium: texture("ui/font_medium").await,
        }
    }
}

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
        PathBuf::from("./assets")
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

async fn material_vert_frag(vert_stub: &str, frag_stub: &str, params: MaterialParams) -> Material {
    let full_stub = ASSETS_ROOT.join("shaders");
    let vert = load_string(
        full_stub
            .join(vert_stub)
            .with_extension("vert")
            .to_string_lossy()
            .as_ref(),
    )
    .await
    .unwrap();
    let frag = load_string(
        full_stub
            .join(frag_stub)
            .with_extension("frag")
            .to_string_lossy()
            .as_ref(),
    )
    .await
    .unwrap();
    load_material(&vert, &frag, params).unwrap()
}

async fn material(path_stub: &str, params: MaterialParams) -> Material {
    material_vert_frag(path_stub, path_stub, params).await
}

use macroquad::{prelude::ImageFormat, texture::Texture2D};

const TILES_TEXTURE: &[u8] = include_bytes!("../res/tiles.png");

pub struct Resources {
    tiles_texture: Texture2D,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tiles_texture: Texture2D::from_file_with_format(TILES_TEXTURE, Some(ImageFormat::Png)),
        }
    }
}

impl Resources {
    pub fn tiles_texture(&self) -> Texture2D {
        self.tiles_texture.clone()
    }
}
use macroquad::{math::Rect, prelude::ImageFormat, texture::Texture2D};

const TILES_TEXTURE: &[u8] = include_bytes!("../res/tiles.png");
const FONT_TEXTURE:  &[u8] = include_bytes!("../res/font.png");

pub struct Resources {
    tiles_atlas: Texture2D,
    font_atlas:  Texture2D,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tiles_atlas: Texture2D::from_file_with_format(TILES_TEXTURE, Some(ImageFormat::Png)),
            font_atlas:  Texture2D::from_file_with_format(FONT_TEXTURE,  Some(ImageFormat::Png)),
        }
    }
}

impl Resources {
    pub fn tiles_atlas(&self) -> &Texture2D {
        &self.tiles_atlas
    }

    pub fn font_atlas(&self) -> &Texture2D {
        &self.font_atlas
    }
}
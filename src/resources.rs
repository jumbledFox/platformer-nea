use macroquad::{math::Rect, prelude::ImageFormat, texture::Texture2D};

const TILES_TEXTURE: &[u8] = include_bytes!("../res/tiles.png");

pub struct Resources {
    tiles_atlas: Texture2D,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tiles_atlas: Texture2D::from_file_with_format(TILES_TEXTURE, Some(ImageFormat::Png)),
        }
    }
}

impl Resources {
    pub fn tiles_atlas(&self) -> &Texture2D {
        &self.tiles_atlas
    }
    pub fn tile_rect(texture: usize) -> Rect {
        Rect::new(
            (texture % 16) as f32 * 16.0,
            (texture / 16) as f32 * 16.0,
            16.0,
            16.0,
        )
    }
}
use macroquad::{prelude::ImageFormat, texture::Texture2D};

const TILES_TEXTURE: &[u8] = include_bytes!("../res/tiles.png");
const FONT_TEXTURE:  &[u8] = include_bytes!("../res/font.png");

pub struct Resources {
    tiles_atlas: Texture2D,
    font_atlas:  Texture2D,
    tile_animation_timer: f64,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tiles_atlas: Texture2D::from_file_with_format(TILES_TEXTURE, Some(ImageFormat::Png)),
            font_atlas:  Texture2D::from_file_with_format(FONT_TEXTURE,  Some(ImageFormat::Png)),
            tile_animation_timer: 0.0,
        }
    }
}

impl Resources {
    pub fn update_tile_animation_timer(&mut self, deltatime: f32) {
        self.tile_animation_timer += deltatime as f64;
    }

    pub fn tiles_atlas(&self) -> &Texture2D {
        &self.tiles_atlas
    }

    pub fn font_atlas(&self) -> &Texture2D {
        &self.font_atlas
    }

    pub fn tile_animation_timer(&self) -> f64 {
        self.tile_animation_timer
    }
}
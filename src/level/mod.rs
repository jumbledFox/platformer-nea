// A bunch of tiles, doors, etc...

use macroquad::{math::Vec2, texture::Texture2D};

pub mod tile;

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<usize>,
    
    // signs (ill do this much later)
    // doors
    // player  start point
    // checkpoints
    // enemy   start points
    // powerup start points

    // For rendering
    tiles_below: Vec<(Vec2, usize)>,
    tiles_above: Vec<(Vec2, usize)>,
}



impl Level {
    // Prepares the below and above tiles
    pub fn prepare_tiles(&mut self) {

    }

    // Renders a bunch of tiles
    pub fn render_tiles(tiles: Vec<(Vec2, usize)>, texture: &Texture2D) {

    }
}
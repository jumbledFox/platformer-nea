use macroquad::math::Rect;

use crate::{level::Level, resources::Resources};

pub mod col_test;

pub trait Entity {
    fn hitbox(&self) -> Rect;
    fn update(&mut self, others: &mut dyn Iterator<Item = &Box<dyn Entity>>, level: &mut Level, deltatime: f32);
    fn draw(&self, resources: &Resources, debug: bool);
}
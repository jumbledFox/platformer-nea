// The current level being played along with the stuff it needs
// e.g. level, player, enemies, timer, etc

use entity::{col_test::ColTest, Entity};
use macroquad::{color::{GREEN, ORANGE, WHITE}, math::vec2};
use player::Player;

use crate::{level::Level, resources::Resources, text_renderer::{render_text, Align}};

pub mod collision;
pub mod entity;
pub mod player;

pub struct Scene {
    level: Level,
    timer: f32,
    chips: usize,
    player: Player,
    entities: Vec<Box<dyn Entity>>
    // enemies
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            level: Level::default(),
            timer: 100.0,
            chips: 42,
            player: Player::default(),
            entities: vec![
                Box::new(ColTest::new(vec2(30.0, 30.0),vec2(0.0,0.0), false)),
                Box::new(ColTest::new(vec2(50.0, 30.0),vec2(0.0,0.0), false)),
                Box::new(ColTest::new(vec2(60.0, 10.0),vec2(0.0,0.0), false)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(10.0, 10.0),vec2(0.0,0.0), true)),
            ],
        }
    }
}

impl Scene {
    pub fn foo(&mut self) {
        self.level.update_tile_render_data();
    }

    pub fn update(&mut self, deltatime: f32,) {
        self.player.update(&mut self.level, deltatime);

        for i in 0..self.entities.len() {
            let (left, right) = self.entities.split_at_mut(i);
            // The unwrap is safe as 'i' is always valid!
            let (entity, right) = right.split_first_mut().unwrap();

            let others: Vec<&mut Box<dyn Entity>> = left
                .iter_mut()
                .chain(right.iter_mut())
                .collect();

            entity.update(&others, &mut self.level, deltatime);
        }

        self.level.update_bumped_tiles(deltatime);
    }

    pub fn draw(&self, lives: usize, resources: &Resources, debug: bool) {
        self.level.render_below(resources);
        for entity in &self.entities {
            entity.draw(resources, debug);
        }
        self.player.draw(resources, debug);
        self.level.render_above(resources);
        self.level.render_bumped_tiles(resources);
        
        // Draw the UI
        render_text("- fox -", ORANGE, vec2( 40.0,  8.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("*",   WHITE,  vec2( 40.0, 24.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("BOOTS",   WHITE,  vec2(176.0, 10.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("HELMET",  WHITE,  vec2(176.0, 22.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("420",     WHITE,  vec2(305.0,  3.0), vec2(1.0, 1.0), Align::End, resources.font_atlas());
        render_text("69",      GREEN,  vec2(305.0, 19.0), vec2(1.0, 1.0), Align::End, resources.font_atlas());
    }
}
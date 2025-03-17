// A bunch of levels to be played, the global chip counter, etc.
// Loaded from a level pack

use scene::Scene;

use crate::{level_pack_data::LevelPackData, resources::Resources, ui::Ui, GameState};

pub mod level;
pub mod scene;
pub mod collision;
pub mod entity;
pub mod player;

pub struct Game {
    level_pack: LevelPackData,
    current_level: usize,

    // levels
    // player types
    scene: Scene,
    lives: usize,
    chips: usize,
}

impl Game {
    pub fn new(level_pack: LevelPackData) -> Self {
        Self {
            level_pack,
            current_level: 0,
            scene: todo!(),
            lives: 3,
            chips: 0,
        }
    }
}

impl GameState for Game {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources) {
        self.scene.update(&mut self.lives, deltatime, resources);
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, debug: bool) {
        self.scene.draw(self.chips, self.lives, resources, debug);
    }
}
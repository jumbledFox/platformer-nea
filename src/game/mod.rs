// A bunch of levels to be played, the global chip counter, etc.
// Loaded from a level pack

use scene::Scene;

use crate::{resources::Resources, ui::Ui, GameState};

pub mod level;
pub mod scene;
pub mod collision;
pub mod entity;
pub mod player;

pub struct Game {
    // levels
    // player types
    scene: Scene,
    global_chips: usize,
    lives: usize,
}

impl Game {
    pub fn new() -> Self {
        Self { scene: todo!(), global_chips: 0, lives: 3 }
    }
}

impl GameState for Game {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources) {
        self.scene.update(&mut self.lives, deltatime, resources);
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, debug: bool) {
        self.scene.draw(self.global_chips, self.lives, resources, debug);
    }
}
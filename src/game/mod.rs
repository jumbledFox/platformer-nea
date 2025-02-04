// A bunch of levels to be played, the global chip counter, etc.
// Loaded from a level pack

use scene::Scene;

use crate::GameState;

pub mod level;
pub mod scene;

pub struct Game {
    // levels
    // player types
    scene: Scene,
    chips: usize,
    lives: usize,
}

impl Game {
    pub fn new() -> Self {
        Self { scene: Scene::default(), chips: 0, lives: 3 }
    }
}

impl GameState for Game {
    fn update(&mut self, deltatime: f32, resources: &crate::resources::Resources) {
        self.scene.update(deltatime, resources);
    }

    fn draw(&self, resources: &crate::resources::Resources, debug: bool) {
        self.scene.draw(self.lives, resources, debug);
    }
}
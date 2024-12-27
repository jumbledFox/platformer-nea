// A bunch of levels to be played, the global chip counter, etc.
// Loaded from a level pack

use crate::scene::Scene;

pub struct Game {
    // levels
    // player types
    scene: Scene,
    chips: usize,
    lives: usize,
}

// The current level being played along with the stuff it needs
// e.g. level, player, enemies, timer, etc

use crate::level::Level;

pub struct Scene {
    level: Level,
    timer: f32,
    chips: usize,
    // player
    // enemies
}

// A bunch of levels to be played, the global chip counter, etc.
// Loaded from a level pack

use macroquad::{color::BLACK, window::clear_background};
use scene::Scene;
use transition::Transition;

use crate::{level_pack_data::LevelPackData, resources::Resources, ui::Ui, GameState};

pub mod transition;
pub mod level;
pub mod scene;
pub mod collision;
pub mod entity;
pub mod player;

#[derive(PartialEq, Eq)]
enum EndAction {
    Finish, Intro, GameOver,
}

pub struct Game {
    transition: Transition, // Based?
    end_action: Option<EndAction>,

    level_pack: LevelPackData,
    current_level: usize,

    level_name: String,
    world_name: String,

    // player types
    scene: Scene,
    lives: usize,
    chips: usize,
}

impl Game {
    pub fn new(level_pack: LevelPackData, resources: &mut Resources) -> Self {
        // would crash if there are no levels... but that's never the case!!
        // literally impossible
        let first_level = level_pack.levels().get(0).unwrap();
        let mut scene = Scene::new(first_level, None, None);
        // Update the scene so we can load all the entities and stuff
        scene.update(&mut 3, 0.0, resources);

        // let level_name = first_level.name();
        // let world_name = level_pack.worlds().get(first_level.world() - 1);

        Self {
            transition: Transition::new(&level_pack),
            end_action: Some(EndAction::Intro),
            
            level_pack,
            current_level: 0,

            level_name: String::from("BLARG"),
            world_name: String::from("AAAAH!"),

            scene,
            lives: 3,
            chips: 0,
        }
    }
}

impl GameState for Game {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources, next_state: &mut Option<Box<dyn GameState>>) {
        // if transitioning the pack intro, update and return

        if let Some(end_action) = self.end_action.take() {
            let (mut head_powerup, mut feet_powerup) = (None, None);

            let mut next_level: Option<usize> = None;

            match end_action {
                EndAction::Intro => {
                    
                }
                EndAction::Finish => {
                    self.current_level += 1;
                    head_powerup = self.scene.head_powerup();
                    feet_powerup = self.scene.feet_powerup();
                    next_level = Some(self.current_level + 1);
                }
                EndAction::GameOver => {
                    // Go back to the first level in the world
                    next_level = Some(0);
                }
            }

            // Load the new level
            if let Some(level_data) = self.level_pack.levels().get(self.current_level) {
                // Load the next scene and update it once to load all the entities and shit
                self.scene = Scene::new(level_data, head_powerup, feet_powerup);
                self.scene.update(&mut self.lives, deltatime, resources);
            } else {
                // Finish pack screen!
            }
        }
        
        /*
        if transitioning, update and return

        update the scene
        */


        self.scene.update(&mut self.lives, deltatime, resources);
        self.end_action = match (self.scene.completed(), false, self.lives) {
            // Finishing level
            (true, _, _) => Some(EndAction::Finish),
            // Game over
            (_, true, 0) => Some(EndAction::GameOver),
            // Respawning
            (_, true, _) => Some(EndAction::Intro),
            // Nothing
            (false, false, _) => None,
        };

        // Update the scene if it exists
        // let mut end_action: Option<LevelEndAction> = None;

        // if let Some(scene) = &mut self.scene {
        //     scene.update(&mut self.lives, deltatime, resources);

        //     end_action = match (scene.completed(), false, self.lives) {
        //         // Finishing level
        //         (true, _, _) => Some(LevelEndAction::Finish),
        //         // Game over
        //         (_, true, 0) => Some(LevelEndAction::GameOver),
        //         // Respawning
        //         (_, true, _) => Some(LevelEndAction::Intro),
        //         // Nothing
        //         (false, false, _) => None,
        //     };

        //     if let Some(end_action) = end_action {
        //         if end_action == LevelEndAction::Intro {
        //             // self.transition.begin_intro(level_name, world, head_powerup, feet_powerup, lives);
        //         }
        //         else if end_action == LevelEndAction::Finish {
        //             // self.transition.begin_finish(scene.player_screen_space_center());
        //         }
        //         else if end_action == LevelEndAction::GameOver {
        //             // self.transition.begin_game_over();
        //         }

        //         self.scene = None;
        //     }
        // }
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, debug: bool) {
        clear_background(BLACK);
        self.scene.draw(self.chips, self.lives, resources, debug);
        // self.transition.draw(resources);
    }
}
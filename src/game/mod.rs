// A bunch of levels to be played, the global chip counter, etc.
// Loaded from a level pack

use macroquad::{color::BLACK, input::{is_key_pressed, KeyCode}, math::Vec2, window::clear_background};
use pause_menu::PauseMenu;
use player::{FeetPowerup, HeadPowerup};
use scene::Scene;
use transition::{Transition, TransitionKind};

use crate::{level_pack_data::LevelPackData, menu::Menu, resources::Resources, ui::Ui, GameState};

pub mod pause_menu;
pub mod transition; // based and redpilled ?
pub mod level;
pub mod scene;
pub mod collision;
pub mod entity;
pub mod player;

#[derive(PartialEq, Eq, Debug)]
enum TransitionAction {
    Finish, Intro, Death, GameOver,
}

pub struct Game {
    transition: Transition, // Based?
    transition_action: Option<TransitionAction>,

    pause_menu: PauseMenu,

    level_pack: LevelPackData,
    current_level: usize,

    level_name: String,
    world_name: String,
    level_num: usize,
    world_num: usize,
    next_powerups: (Option<HeadPowerup>, Option<FeetPowerup>),
    checkpoint: Option<usize>,

    // player types
    scene: Option<Scene>,
    lives: usize,
    chips: usize,
}

impl Game {
    pub fn new(level_pack: LevelPackData) -> Self {
        Self {
            transition: Transition::new(&level_pack),
            transition_action: None,
            
            pause_menu: PauseMenu::default(),

            level_pack,
            current_level: 0,

            level_name: String::from("you'll never see this"),
            world_name: String::from("muahahaha :3 "),
            level_num: 420,
            world_num: 69,
            next_powerups: (None, None),
            checkpoint: None,

            scene: None,
            lives: 3,
            chips: 0,
        }
    }
}

impl GameState for Game {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources, next_state: &mut Option<Box<dyn GameState>>) {
        if is_key_pressed(KeyCode::Escape) {
            self.pause_menu.set_active(!self.pause_menu.active());
        }
        resources.set_anim_timer_update(!self.pause_menu.active());
        if self.pause_menu.active() {
            let exit = self.pause_menu.update(deltatime, ui, resources);
            if exit {
                *next_state = Some(Box::new(Menu::new(Some(self.level_pack.file_name().clone()))));
            }
            return;
        }

        self.transition.update(deltatime);
        
        if self.transition.time_up() {
            match self.transition.kind() {
                TransitionKind::PackStart(..) |
                TransitionKind::Finish(_)     |
                TransitionKind::GameOver(_)   |
                TransitionKind::Death(_)  => self.transition_action = Some(TransitionAction::Intro),
                _ => self.transition_action = None,
            };
            if matches!(self.transition.kind(), TransitionKind::Death(_)) {
                self.lives -= 1;
            }
            if matches!(self.transition.kind(), TransitionKind::GameOver(_)) {
                // Go back to the first level in the world
                if let Some(level) = self.level_pack.levels().get(self.current_level) {
                    let world = level.world();
                    self.current_level = self.level_pack.levels()
                        .iter()
                        .position(|l| l.world() == world)
                        .unwrap_or_default();
                }
                self.lives = 3;
                self.chips = 0;
            }
            self.transition.set_none();
        }

        if let Some(transition_action) = self.transition_action.take() {
            match transition_action {
                TransitionAction::Intro => {
                    resources.reset_tile_animation_timer();
                    // Load the level and begin the transition
                    if let Some(level_data) = self.level_pack.levels().get(self.current_level) {
                        let mut scene = Scene::new(level_data, self.checkpoint, self.next_powerups.0, self.next_powerups.1);
                        // Update the scene so we can load all the entities and stuff
                        // kinda hacky passing &mut 1... idk
                        scene.update(&mut 0, &mut 0, 0.0, resources);
                        self.scene = Some(scene);

                        self.level_num = 1 + self.current_level - self.level_pack.levels()
                            .iter()
                            .position(|l| l.world() == level_data.world())
                            .unwrap_or_default(); 
                        self.world_num = level_data.world() as usize;

                        self.level_name = level_data.name().clone();
                        self.world_name = match self.world_num {
                            0     => String::new(),
                            w @ _ => self.level_pack.worlds().get(w-1).cloned().unwrap_or(String::new()),
                        };

                        self.transition.begin_intro(
                            self.level_pack.name().clone(),
                            self.level_pack.author().clone(),
                            self.level_name.clone(),
                            self.world_name.clone(),
                            level_data.world(),
                            self.next_powerups.0.take(),
                            self.next_powerups.1.take(),
                            self.lives
                        );
                    }
                    // Or if we've beaten all the levels, show the end screen!
                    else {
                        self.scene = None;
                        let (head, feet) = self.next_powerups;
                        self.transition.begin_pack_finish(self.level_pack.name().clone(), self.level_pack.author().clone(), head, feet, self.chips, 0, 0, self.level_pack.levels().len());
                    }
                }
                TransitionAction::Finish => {
                    let (center, head, feet) = match &self.scene {
                        Some(s) => (s.player_screen_space_center(), s.head_powerup(), s.feet_powerup()),
                        // Should never happen, but im not gonna unwrap now, am i?!
                        None => (Vec2::ZERO, None, None),
                    };
                    self.current_level += 1;
                    self.next_powerups = (head, feet);
                    self.checkpoint = None;
                    self.transition.begin_finish(center);
                    
                }
                TransitionAction::Death => {
                    let center = match &self.scene {
                        Some(s) => s.player_screen_space_center(),
                        None => Vec2::ZERO,
                    };
                    self.transition.begin_death(center);
                }
                TransitionAction::GameOver => {
                    let center = match &self.scene {
                        Some(s) => s.player_screen_space_center(),
                        None => Vec2::ZERO,
                    };
                    self.checkpoint = None;
                    self.transition.begin_game_over(center);
                }
            }
        }

        if matches!(self.transition.kind(), TransitionKind::PackFinish(..)) {
            if is_key_pressed(KeyCode::Space) {
                *next_state = Some(Box::new(Menu::new(Some(self.level_pack.file_name().clone()))));
                return;
            }
        }
        
        // If transitioning, don't update the scene
        if !self.transition.can_update() {
            resources.set_anim_timer_update(matches!(self.transition.kind(), TransitionKind::Death(_) | TransitionKind::GameOver(_)));
            return;
        }
        resources.set_anim_timer_update(true);
        if let Some(scene) = &mut self.scene {
            scene.update(&mut self.chips, &mut self.lives, deltatime, resources);
            self.checkpoint = scene.checkpoint();

            self.transition_action = match (scene.completed(), scene.dead(), self.lives) {
                // Finishing level
                (true, _, _) => Some(TransitionAction::Finish),
                // Game over
                (_, true, 0) => Some(TransitionAction::GameOver),
                // Respawning
                (_, true, _) => Some(TransitionAction::Death),
                // Nothing
                _=> None,
            };
        }
    }

    fn draw(&self, ui: &Ui, resources: &Resources, debug: bool) {
        clear_background(BLACK);

        if let Some(scene) = &self.scene {
            scene.draw(Some((self.world_num, self.level_num)), self.chips, self.lives, resources, debug);
        }
        self.transition.draw(resources, debug);

        if self.pause_menu.active() {
            self.pause_menu.draw(ui, resources);
        }
    }
}
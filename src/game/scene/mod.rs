// The current level being played along with the stuff it needs
// e.g. level, player, enemies, timer, etc

use camera::Camera;
use entity_spawner::EntitySpawner;
use fader::Fader;
// use entity::{col_test::ColTest, frog::Frog, player::Player, Entity};
use macroquad::{color::{Color, GREEN, ORANGE, RED, WHITE}, input::{is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}};
use particles::Particles;
use sign_display::SignDisplay;

use crate::{editor::editor_level::EditorLevel, game::level::{tile::LockColor, Level}, level_pack_data::{level_pos_to_pos, LevelData}, resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, draw_rect_lines}, VIEW_SIZE};

use super::{entity::{crate_entity::Crate, key::Key, Entity, EntityKind, Id}, player::{FeetPowerup, HeadPowerup, Player}};

pub mod camera;
pub mod entity_spawner;
pub mod particles;
pub mod fader;
pub mod sign_display;

pub const PHYSICS_STEP: f32 = 1.0 / 120.0;
pub const MAX_FALL_SPEED: f32 = 2.0;
pub const GRAVITY: f32 = 0.045;

pub struct Scene {
    level: Level,
    timer: f32,

    camera: Camera,
    player: Player,
    entities: Vec<Box<dyn Entity>>,
    entity_spawner: EntitySpawner,
    particles: Particles,

    fader: Fader,
    sign_display: SignDisplay,
    physics_update_timer: f32,
}

impl Scene {
    pub fn from_editor_level(editor_level: &EditorLevel, player_spawn: Option<Vec2>) -> Self {
        // TODO: ... We can set world as 0 since we don't really care at all, as we're in the editor... for now?! 
        let level = LevelData::from_editor_level(editor_level, 0)
            .to_level();

        let player_spawn = player_spawn.unwrap_or(editor_level.spawn());

        Scene {
            level,
            timer: 0.0,

            camera: Camera::new(player_spawn),
            player: Player::new(player_spawn),
            entities:     Vec::with_capacity(64),
            entity_spawner: EntitySpawner::default(),
            particles: Particles::default(),

            fader: Fader::default(),
            sign_display: SignDisplay::default(),
            physics_update_timer: PHYSICS_STEP,
        }
    }

    pub fn update(&mut self, lives: &mut usize, deltatime: f32, resources: &mut Resources) {
        // See if we should add any entities
        if self.camera.should_update_entities() {
            for (&spawn_pos, &k) in self.level.entity_spawns().iter() {
                let id = Id::Level(spawn_pos);
                // If the spawn pos isn't in the camera's rect, or it exists, or it's being carried, don't spawn it!
                if !self.camera.entity_in_rect(spawn_pos)
                || self.entities.iter().any(|e| e.id() == id)
                || self.player.holding_id() == Some(id) {
                    continue;
                }
                // Otherwise... spawn it!
                self.entity_spawner.add_entity(level_pos_to_pos(spawn_pos) + k.tile_offset(), Vec2::ZERO, k, Some(spawn_pos));
            }
            // Spawn all of them
            self.entity_spawner.spawn_entities(&mut self.entities);
        }

        self.timer -= deltatime;
        self.fader.update(deltatime);
        self.sign_display.update();

        let mut freeze = self.fader.fading();
        if let Some(dest) = self.fader.move_player() {
            freeze = false;
            self.player.set_pos(dest);
        }
        freeze |= self.sign_display.active();

        resources.set_anim_timer_update(!freeze);

        self.player.update_move_dir();
        if freeze { return; }
        self.player.update(&mut self.entities, &mut self.fader, &mut self.sign_display, &mut self.level, resources);

        for e in &mut self.entities {
            e.update(resources);
        }

        // Update all of the physics in a fixed time-step
        self.physics_update_timer += deltatime;
        while self.physics_update_timer >= PHYSICS_STEP {
            self.particles.update(&self.camera);

            self.player.physics_update(&mut self.entities, &mut self.entity_spawner, &mut self.particles, &mut self.level, resources);

            // Update all of the entities
            let mut others: Vec<&mut Box<dyn Entity>>;
            for i in 0..self.entities.len() {
                let (left, right) = self.entities.split_at_mut(i);
                // The unwrap is safe as 'i' is always valid!
                let (entity, right) = right.split_first_mut().unwrap();
                others = left
                    .iter_mut()
                    .chain(right.iter_mut())
                    .collect();
                entity.physics_update(&mut self.player, &mut others, &mut self.entity_spawner, &mut self.particles, &mut self.level, &mut self.camera, resources);
            }
            self.entity_spawner.spawn_entities(&mut self.entities);
            
            self.player.hurt_check(&mut self.entities, &mut self.level, resources);

            // Remove all the entities that need to be destroyed
            // Also collects chips / lives / powerups...
            // TODO: Soft remove all entities out of the screen with a spawn_pos id 
            let mut disable_respawn = |id: Id| {
                if let Id::Level(spawn_pos) = id {
                    self.level.remove_entity_spawn(spawn_pos);
                }
            };
            for i in (0..self.entities.len()).rev() {
                if self.entities[i].should_destroy() {
                    disable_respawn(self.entities[i].id());
                    self.entities[i].destroy(&mut self.entity_spawner, &mut self.particles);
                    self.entities.remove(i);
                    continue;
                }
                // TODO: Particles, letting flying crates and keys and stuff collect chips
                // Collecting chips
                if matches!(self.entities[i].kind(), EntityKind::Chip(_)) {
                    if self.entities[i].hitbox().overlaps(&self.player.chip_hitbox()) {
                        self.player.give_chip();
                        disable_respawn(self.entities[i].id());
                        self.entities.remove(i);
                        continue;
                    }
                }
                // Collecting lives
                else if matches!(self.entities[i].kind(), EntityKind::Life(_)) {
                    if self.entities[i].hitbox().overlaps(&self.player.chip_hitbox()) {
                        disable_respawn(self.entities[i].id());
                        self.entities.remove(i);
                        *lives += 1;
                        continue;
                    }
                }
                // TODO: Collecting powerups...
            }
            self.level.fixed_update();
            self.physics_update_timer -= PHYSICS_STEP;

            // Sorting them every frame?! idk man...... it works..
            self.entities.sort_by(|e1, e2| e1.kind().cmp(&e2.kind()));

            self.camera.update(self.player.pos(), deltatime);
        }

        self.level.update_bumped_tiles(deltatime);
        self.level.update_if_should(resources);
    }

    pub fn draw(&self, chips: usize, lives: usize, resources: &Resources, debug: bool) {
        let camera_pos = self.camera.pos();

        draw_rect(Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y), self.level.bg_col());
        self.level.render_bg(camera_pos, resources);
        self.level.render_below(camera_pos, resources);

        for e in &self.entities {
            e.draw(camera_pos, resources);
        }
        self.player.draw(camera_pos, resources, debug);
        self.particles.draw(camera_pos, resources);


        // Draw the entities in reverse so the player is always on top
        // for (i, entity) in self.entities.iter().enumerate().rev() {
        //     entity.draw(resources, i, debug);
        // }
        self.level.render_above(camera_pos, resources, debug);
        self.level.render_bumped_tiles(camera_pos, resources);

        // Draw the UI
        // Lives
        render_text("- fox -",           ORANGE, vec2( 40.0,  8.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text("*",                 WHITE,  vec2( 40.0, 24.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text(&format!("{lives}"), WHITE,  vec2( 60.0, 24.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        resources.draw_rect(vec2(13.0, 16.0), Rect::new(192.0, 16.0, 16.0, 15.0), false, false, WHITE, resources.entity_atlas());
        
        // Powerups
        let render_powerup_text = |text: &str, col: u32, y: f32| {
            render_text(text, Color::from_hex(col), vec2(176.0, y), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        };

        let mut powerup_y = 10.0;
        if let Some(powerup) = self.player.head_powerup() {
            let (name, col) = match powerup {
                HeadPowerup::Helmet => ("Helmet", 0xe43b44),
            };
            render_powerup_text(name, col, powerup_y);
            powerup_y += 12.0;
        }
        if let Some(powerup) = self.player.feet_powerup() {
            let (name, col) = match powerup {
                FeetPowerup::MoonShoes => ("Moon Shoes", 0xbc41c7),
                FeetPowerup::Boots => ("Boots", 0x855a55),
            };
            render_powerup_text(name, col, powerup_y);
        }

        // Timer and points
        render_text(&format!("{:?}", self.timer.floor() as usize), WHITE,  vec2(305.0,  3.0), vec2(1.0, 1.0), Align::End, Font::Large, resources);
        render_text(&format!("{:?}", self.player.chips() + chips), GREEN,  vec2(305.0, 19.0), vec2(1.0, 1.0), Align::End, Font::Large, resources);

        if debug {
            render_text(&format!("pos: [{:8.3}, {:8.3}]", self.player.pos().x, self.player.pos().y), RED, vec2(10.0, 50.0), Vec2::ONE, Align::End, Font::Small, resources);
            render_text(&format!("vel: [{:8.3}, {:8.3}]", self.player.vel().x, self.player.vel().y), RED, vec2(10.0, 60.0), Vec2::ONE, Align::End, Font::Small, resources);    
            render_text(&format!("state: {:?}", self.player.state()), RED, vec2(10.0, 70.0), Vec2::ONE, Align::End, Font::Small, resources);    
        }

        self.sign_display.draw(resources);
        self.fader.draw();
    }
}
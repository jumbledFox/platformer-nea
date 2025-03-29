// The current level being played along with the stuff it needs
// e.g. level, player, enemies, timer, etc

use std::f32::consts::PI;

use camera::Camera;
use entity_spawner::EntitySpawner;
use fader::Fader;
// use entity::{col_test::ColTest, frog::Frog, player::Player, Entity};
use macroquad::{color::{Color, GREEN, RED, WHITE}, math::{vec2, Rect, Vec2}, rand::gen_range};
use particles::{ParticleKind, Particles};
use sign_display::SignDisplay;

use crate::{editor::editor_level::EditorLevel, game::level::Level, level_pack_data::{level_pos_to_pos, LevelData}, resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, rect}, VIEW_SIZE};

use super::{entity::{chip::Chip, Entity, EntityKind, Id}, player::{FeetPowerup, HeadPowerup, Invuln, Player, PowerupKind}};

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

    camera: Camera,
    player: Player,
    entities: Vec<Box<dyn Entity>>,
    entity_spawner: EntitySpawner,
    particles: Particles,

    fader: Fader,
    sign_display: SignDisplay,
    physics_update_timer: f32,

    completed: bool,
}

// TODO: show_mouse(false);

impl Scene {
    pub fn new(level_data: &LevelData, checkpoint: Option<usize>, head_powerup: Option<HeadPowerup>, feet_powerup: Option<FeetPowerup>) -> Self {
        let mut level = level_data.to_level();

        let spawn = match checkpoint {
            Some(c) => {
                level.set_checkpoint(c);
                level.checkpoints().get(c).cloned().unwrap_or_default()
            }
            None => level.spawn(),
        };

        let mut scene = Self {
            level,

            camera: Camera::new(spawn),
            player: Player::new(spawn, head_powerup, feet_powerup),
            entities:       Vec::with_capacity(128),
            entity_spawner: EntitySpawner::default(),
            particles:      Particles::default(),

            fader:        Fader::default(),
            sign_display: SignDisplay::default(),
            physics_update_timer: PHYSICS_STEP,

            completed: false,
        };
        scene.spawn_all_entities();
        scene
    }

    pub fn from_editor_level(editor_level: &EditorLevel, player_spawn: Option<Vec2>) -> Self {
        // TODO: ... We can set world as 0 since we don't really care at all, as we're in the editor... for now?! 
        let level = LevelData::from_editor_level(editor_level, 0)
            .to_level();

        let player_spawn = player_spawn.unwrap_or(editor_level.spawn());

        let mut scene = Self {
            level,

            camera: Camera::new(player_spawn),
            player: Player::new(player_spawn, None, None),
            entities:     Vec::with_capacity(64),
            entity_spawner: EntitySpawner::default(),
            particles: Particles::default(),

            fader: Fader::default(),
            sign_display: SignDisplay::default(),
            physics_update_timer: PHYSICS_STEP,

            completed: false,
        };
        scene.spawn_all_entities();
        scene
    }

    fn spawn_all_entities(&mut self) {
        // INFO
        // So, when I first made this project, entities only spawned in when they entered
        // the screen. This was good, however I decided to change it so that entities were
        // ALWAYS spawned in, but only UPDATED when on screen.
        // The reason I did this was because of cannons, I wanted them to shoot when they were
        // slightly off-screen, which worked after they were spawned in, but not beforehand!
        // Sooo, the whole entity id thing is now useless... but like... it's nice to have ids i guess?
        // my bad... i've got less than a week left and i've not done much writing,
        // My head is filled with caffeine, confusion, and an inexplicable sense of impending doom.

        for (&spawn_pos, &k) in self.level.entity_spawns().iter() {
            self.entity_spawner.add_entity(level_pos_to_pos(spawn_pos) + k.tile_offset(), Vec2::ZERO, k, Some(spawn_pos));
        }
        // Spawn all of them
        self.entity_spawner.spawn_entities(&mut self.entities);
    }

    pub fn checkpoint(&self) -> Option<usize> {
        self.level.checkpoint()
    }
    pub fn completed(&self) -> bool {
        self.completed
    }
    pub fn dead(&self) -> bool {
        self.player.dead_stop()
    }
    pub fn player_screen_space_center(&self) -> Vec2 {
        self.player.pos() + 8.0 - self.camera.pos()
    }
    pub fn head_powerup(&self) -> Option<HeadPowerup> {
        self.player.head_powerup()
    }
    pub fn feet_powerup(&self) -> Option<FeetPowerup> {
        self.player.feet_powerup()
    }
    pub fn chips(&self) -> usize {
        self.player.chips()
    }

    pub fn update(&mut self, lives: &mut usize, deltatime: f32, resources: &mut Resources) {
        self.fader.update(deltatime);
        self.sign_display.update();

        let mut freeze = self.fader.fading();
        if let Some(dest) = self.fader.move_player() {
            freeze = false;
            self.camera.offset_center(dest - self.player.pos());
            self.player.set_pos(dest);
        }

        freeze |= self.sign_display.active();

        resources.set_anim_timer_update(!freeze);

        self.player.update_move_dir();
        if freeze { return; }
        self.player.update(&mut self.entities, &mut self.camera, &mut self.fader, &mut self.sign_display, &mut self.level, resources);

        for e in &mut self.entities {
            e.update(resources);
        }

        // Update all of the physics in a fixed time-step
        self.physics_update_timer += deltatime;
        while self.physics_update_timer >= PHYSICS_STEP &&!self.completed {

            self.player.physics_update(&mut self.entities, &mut self.entity_spawner, &mut self.particles, &mut self.level, resources);

            if self.player.dead() {
                self.physics_update_timer -= PHYSICS_STEP;
                self.particles.update(&self.camera);
                continue;
            }

            // Update all of the entities
            let mut others: Vec<&mut Box<dyn Entity>>;
            for i in 0..self.entities.len() {
                // Only update entities that are on screen
                if self.entities[i].destroy_offscreen() || self.entities[i].update_far() {
                    if !self.camera.on_screen_far(self.entities[i].pos()) {
                        continue;
                    }
                } else {
                    if !self.camera.on_screen(self.entities[i].pos()) {
                        continue;
                    }
                }
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
            
            self.player.hurt_check(&mut self.entities, &mut self.particles, &self.level, resources);

            // Remove all the entities that need to be destroyed
            // Also collects chips / lives / powerups...
            let mut disable_respawn = |id: Id| {
                if let Id::Level(spawn_pos) = id {
                    self.level.remove_entity_spawn(spawn_pos);
                }
            };
            let mut collected_powerup = false;
            for i in (0..self.entities.len()).rev() {
                if self.entities[i].should_destroy()
                || (self.entities[i].destroy_offscreen() && !self.camera.on_screen_far(self.entities[i].pos()))
                {
                    disable_respawn(self.entities[i].id());
                    self.entities[i].destroy(&mut self.entity_spawner, &mut self.particles);
                    self.entities.remove(i);
                    continue;
                }
                
                let mut particle_col = None;
                // Collecting chips
                if matches!(self.entities[i].kind(), EntityKind::Chip(_)) {
                    if self.entities[i].hitbox().overlaps(&self.player.chip_hitbox()) {
                        particle_col = Some((self.entities[i].hitbox().center(), Chip::particle_color(false)));
                        self.player.give_chip();
                        disable_respawn(self.entities[i].id());
                        self.entities.remove(i);
                    }
                }
                // Collecting lives
                else if matches!(self.entities[i].kind(), EntityKind::Life(_)) {
                    if self.entities[i].hitbox().overlaps(&self.player.chip_hitbox()) {
                        particle_col = Some((self.entities[i].hitbox().center(), Chip::particle_color(true)));
                        self.particles.add_particle(self.entities[i].hitbox().center(), vec2(0.0, -0.5), ParticleKind::OneUp);
                        disable_respawn(self.entities[i].id());
                        self.entities.remove(i);
                        *lives += 1;
                    }
                }
                // Collecting powerups
                else if !collected_powerup {
                    if let EntityKind::Powerup(kind, _, false) = self.entities[i].kind() {
                        if !self.entities[i].hitbox().overlaps(&self.player.chip_hitbox()) {
                            continue;
                        }
                        match (kind, self.player.invuln()) {
                            (PowerupKind::Head(_), Invuln::Powerup(PowerupKind::Head(_), _)) |
                            (PowerupKind::Feet(_), Invuln::Powerup(PowerupKind::Feet(_), _)) => continue,
                            (PowerupKind::Head(kind), ..) if self.player.head_powerup() == Some(kind) => continue,
                            (PowerupKind::Feet(kind), ..) if self.player.feet_powerup() == Some(kind) => continue,
                            _ => {}
                        }
                        let center = self.entities[i].hitbox().center();
                        particle_col = Some((center, kind.particle_color()));
                        collected_powerup = true;
                        self.player.collect_powerup(kind, center, &mut self.particles, &mut self.entity_spawner);
                        disable_respawn(self.entities[i].id());
                        self.entities.remove(i);
                    }
                }
                
                if let Some((center, col)) = particle_col {
                    for i in 0..4 {
                        let angle = Vec2::from_angle((PI * 2.0 / 4.0) * (i as f32 + 0.5));
                        self.particles.add_particle(center + angle, angle * gen_range(0.1, 0.6), ParticleKind::Sparkle(col));
                    }
                }
            }

            // Finishing
            let fin_hitbox = rect(self.level.finish(), vec2(16.0, 16.0));
            if fin_hitbox.contains(self.player.pos() + 8.0) {
                self.completed = true;
            }

            self.level.fixed_update();
            self.physics_update_timer -= PHYSICS_STEP;

            // Sorting them every frame?! idk man...... it works..
            self.entities.sort_by(|e1, e2| e1.kind().cmp(&e2.kind()));

            self.camera.update(deltatime, &self.player, &self.level);
            self.particles.update(&self.camera);
        }

        self.level.update_bumped_tiles(deltatime);
        self.level.update_if_should(resources);
    }

    pub fn draw(&self, world_level: Option<(usize, usize)>, chips: usize, lives: usize, resources: &Resources, debug: bool) {
        let camera_pos = self.camera.pos();
        let view_rect = Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y);
        draw_rect(view_rect, self.level.bg_col());
        self.level.render_bg(camera_pos, resources);
        self.level.render_below(camera_pos, resources);

        for e in &self.entities {
            e.draw(&self.player, camera_pos, resources);
        }
        self.player.draw(self.completed, camera_pos, resources, debug);

        self.level.render_above(camera_pos, resources, debug);
        self.level.render_bumped_tiles(camera_pos, resources);
        self.particles.draw(camera_pos, resources);

        // X-Ray goggles overlay
        if self.player.head_powerup().is_some_and(|p| p == HeadPowerup::XrayGoggles) {
            draw_rect(view_rect, Color::from_rgba(82, 195, 129, 20));
        }

        // Draw the UI
        // Lives
        render_text("- fox -",Color::from_hex(0xf77622), vec2( 40.0,  8.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text("*", WHITE,  vec2( 40.0, 24.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text(&format!("{lives}"), WHITE,  vec2( 60.0, 24.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        resources.draw_rect(vec2(13.0, 16.0), Rect::new(192.0, 16.0, 16.0, 15.0), false, false, WHITE, resources.entity_atlas());
        
        // Powerups
        let render_powerup_text = |kind: PowerupKind, y: f32| {
            render_text(kind.name(), kind.text_color(), vec2(VIEW_SIZE.x / 2.0, y), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        };

        let (head_y, feet_y) = match (self.player.head_powerup(), self.player.feet_powerup()) {
            (Some(_), Some(_)) => (10.0, 22.0),
            _ => (17.0, 17.0),
        };
        if let Some(powerup) = self.player.head_powerup() {
            render_powerup_text(PowerupKind::Head(powerup), head_y);
        }
        if let Some(powerup) = self.player.feet_powerup() {
            render_powerup_text(PowerupKind::Feet(powerup), feet_y);
        }

        // World/level and points
        if let Some((world, level)) = world_level {
            render_text(&format!("{:>2} - {:<2}", world, level), WHITE, vec2(303.0, 8.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        }
        resources.draw_rect(vec2(305.0 - 16.0 - 5.0, 16.0), Rect::new(174.0, 16.0, 18.0, 16.0), false, false, WHITE, resources.entity_atlas());
        render_text(&format!("{:?}", self.player.chips() + chips), GREEN,  vec2(305.0, 19.0), vec2(1.0, 1.0), Align::End, Font::Large, resources);

        if debug {
            render_text(&format!("pos: [{:8.3}, {:8.3}]", self.player.pos().x, self.player.pos().y), RED, vec2(10.0, 50.0), Vec2::ONE, Align::End, Font::Small, resources);
            render_text(&format!("vel: [{:8.3}, {:8.3}]", self.player.vel().x, self.player.vel().y), RED, vec2(10.0, 60.0), Vec2::ONE, Align::End, Font::Small, resources);    
            render_text(&format!("state: {:?}", self.player.state()), RED, vec2(10.0, 70.0), Vec2::ONE, Align::End, Font::Small, resources);    
        }

        self.sign_display.draw(resources);
        self.fader.draw();
        self.camera.draw(debug);
    }
}
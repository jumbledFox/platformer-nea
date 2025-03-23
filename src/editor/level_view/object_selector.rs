// The screen that lets you select a tile/object, opened/closed by pressing space when editing the level

use macroquad::{color::{Color, WHITE}, color_u8, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{game::{entity::{crate_entity::CrateKind, EntityKind}, level::{things::DoorKind, tile::{render_tile, BrickColor, CheckerBlockColor, LockColor, Tile, TileDir, TileRenderLayer}, TileDrawKind, TileRenderData}, player::{FeetPowerup, HeadPowerup, PowerupKind}}, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{button::{Button, ButtonState}, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 150);

const TILES_Y: f32 = 4.0;
const ENTITIES_Y: f32 = 96.0;
const OTHERS_Y: f32 = 180.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Tile(Tile),
    Entity(EntityKind),
    Other(ObjectOtherKind),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectOtherKind {
    Sign,
    Door(DoorKind), // Teleport
    Spawn,
    Finish,
    Checkpoint,
}

pub struct ObjectSelector {
    active: bool,
    object_buttons: Vec<(Object, Button)>
}

impl ObjectSelector {
    pub fn new(resources: &Resources) -> Self {
        let mut object_buttons = vec![];

        // Define the lists of stuff
        let mut tiles = vec![ 
            Tile::Grass,
            Tile::Dirt,
            Tile::Stone,
            Tile::BrightStone,
            Tile::Sand,
            Tile::Cloud,
            Tile::Metal,
            Tile::Bricks(BrickColor::Gray),
            Tile::Bricks(BrickColor::Tan),
            Tile::Bricks(BrickColor::Blue),
            Tile::Bricks(BrickColor::Green),
            Tile::Checker,
            Tile::CheckerBlock(CheckerBlockColor::Cyan),
            Tile::CheckerBlock(CheckerBlockColor::Orange),
            Tile::CheckerBlock(CheckerBlockColor::Purple),
            Tile::WoodenPlatform,
            Tile::MetalPlatform,
            Tile::Bridge,
            Tile::Rope,
            Tile::ShortGrass,
            Tile::TallGrass,
            Tile::DeadShortGrass,
            Tile::DeadTallGrass,
            Tile::Bush,
            Tile::Door,
            Tile::Ladder,
            Tile::Vine,
            Tile::StoneBlock,
            Tile::Glass,
            Tile::Block,
            Tile::Spikes(TileDir::Bottom),
            Tile::Spikes(TileDir::Left),
            Tile::Spikes(TileDir::Top),
            Tile::Spikes(TileDir::Right),
            Tile::Switch(false), Tile::SwitchBlockOff(true), Tile::SwitchBlockOn(false),
            Tile::Lava,
        ];
        let mut entities = vec![
            (EntityKind::Chip(false), "Chip".to_string()),
            (EntityKind::Crate(CrateKind::Chip(false)), "Chip crate".to_string()),
            (EntityKind::Crate(CrateKind::Chip(true)), "Large chip crate".to_string()),
            (EntityKind::Life(false), "Life".to_string()),
            (EntityKind::Crate(CrateKind::Life), "Life crate".to_string()),
            (EntityKind::Powerup(PowerupKind::Head(HeadPowerup::Helmet),      false, false), "Helmet".to_string()),
            (EntityKind::Powerup(PowerupKind::Head(HeadPowerup::XrayGoggles), false, false), "X-Ray Goggles".to_string()),
            (EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::Boots),       false, false), "Boots".to_string()),
            (EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::MoonShoes),   false, false), "Moon Shoes".to_string()),
            (EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::Skirt),       false, false), "Skirt".to_string()),
            (EntityKind::Crate(CrateKind::Powerup(PowerupKind::Head(HeadPowerup::Helmet))),      "Helmet crate".to_string()),
            (EntityKind::Crate(CrateKind::Powerup(PowerupKind::Head(HeadPowerup::XrayGoggles))), "X-Ray Goggles crate".to_string()),
            (EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::Boots))),       "Boots crate".to_string()),
            (EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::MoonShoes))),   "Moon Shoes crate".to_string()),
            (EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::Skirt))),       "Skirt crate".to_string()),
            (EntityKind::Frog(false), "Frog".to_string()),
            (EntityKind::Crate(CrateKind::Frog(false)), "Single-frog crate".to_string()),
            (EntityKind::Crate(CrateKind::Frog(true)), "Multi-frog crate".to_string()),
            (EntityKind::Crate(CrateKind::Explosive), "Explosive crate".to_string()),
            (EntityKind::Goat, "Aerosol Kid".to_string()),
            (EntityKind::Armadillo(false, false), "Armadillo".to_string()),
            (EntityKind::Armadillo(false, true), "Spinning Armadillo".to_string()),
        ];

        for col in LockColor::colors().iter() {
            tiles.push(Tile::Lock(*col));
            tiles.push(Tile::LockBlock(*col));
            entities.push((EntityKind::Key(*col), format!("{:?} key", col)));
        }
        for col in LockColor::colors().iter() {
            entities.push((EntityKind::Crate(CrateKind::Key(*col)), format!("{:?} key crate", col)));
        }

        let others = [
            (ObjectOtherKind::Sign, "Sign"),
            (ObjectOtherKind::Door(DoorKind::Door), "Door"),
            (ObjectOtherKind::Door(DoorKind::Teleporter), "Teleporter"),
            (ObjectOtherKind::Door(DoorKind::SeamlessTeleporter), "Seamless Teleporter"),
            (ObjectOtherKind::Spawn, "Spawn"),
            (ObjectOtherKind::Finish, "Finish"),
            (ObjectOtherKind::Checkpoint, "Checkpoint"),
        ];

        // Add all of the tiles
        let wrap = 17;
        let mut x = 8.0;
        let mut y = TILES_Y + 12.0;
        for (i, t) in tiles.iter().enumerate() {
            let button = Button::new(Rect::new(x, y, 16.0, 16.0), None, Some(resources.tile_data(*t).name().clone()));
            x += 20.0;
            if i % wrap == wrap-1 {
                x = 8.0;
                y += 20.0;
            }
            object_buttons.push((Object::Tile(*t), button));
        }

        // Add all of the entities
        let mut x = 8.0;
        let mut y_index = 0;
        let y_centers = [20.0, 45.0, 70.0];

        for (e, n) in entities {
            let size = e.object_selector_size();
            // Line wrapping
            if x + size.x >= VIEW_SIZE.x - 8.0 {
                x = 8.0;
                y_index += 1;
            }
            let y = ENTITIES_Y + (y_centers[y_index] - size.y / 2.0).floor();
            let button = Button::new(Rect::new(x, y, size.x, size.y), None, Some(n));
            object_buttons.push((Object::Entity(e), button));

            x += size.x + 4.0;
        }

        // Add the others
        let mut x = 8.0;
        let mut y = OTHERS_Y + 12.0;
        for (i, (o, n)) in others.iter().enumerate() {
            let button = Button::new(Rect::new(x, y, 16.0, 16.0), None, Some(n.to_string()));
            x += 20.0;
            if i % wrap == wrap-1 {
                x = 8.0;
                y += 20.0;
            }
            object_buttons.push((Object::Other(*o), button));
        }

        Self { active: false, object_buttons }
    }

    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Object> {
        for (o, b) in &mut self.object_buttons {
            b.update(ui);
            if b.released() {
                return Some(*o);
            }
        }
        None
    }

    pub fn draw(&self, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);
        render_text("Object Selector", WHITE, vec2(VIEW_SIZE.x/2.0, 7.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);

        render_text("Tiles:",    WHITE, vec2(4.0, TILES_Y),    vec2(1.0, 1.0), Align::End, Font::Small, resources);
        render_text("Entities:", WHITE, vec2(4.0, ENTITIES_Y), vec2(1.0, 1.0), Align::End, Font::Small, resources);
        render_text("Others:",   WHITE, vec2(4.0, OTHERS_Y),   vec2(1.0, 1.0), Align::End, Font::Small, resources);
        for (object, b) in self.object_buttons.iter() {
            let pos = b.rect().point() + match b.state() {
                ButtonState::Hovered => -Vec2::ONE,
                _ => Vec2::ZERO,
            };
            match object {
                // If it's a tile, draw it as such
                Object::Tile(t) => render_tile(
                    &TileRenderData { tile: *t, draw_kind: TileDrawKind::Single(0), pos},
                    Vec2::ZERO,
                    TileRenderLayer::Foreground(false),
                    resources,
                ),
                Object::Entity(e) => e.draw_editor(false, false, pos, vec2(0.0, 0.0), resources),
                Object::Other(o) => ObjectSelector::draw_object_other(pos, *o, false, resources),
            }
        }
    }

    pub fn draw_object_other(pos: Vec2, other: ObjectOtherKind, transparent: bool, resources: &Resources) {
        let draw_from_atlas = |rect: Rect| {
            let color = match transparent {
                true  => Color::from_rgba(255, 255, 255, 128),
                false => WHITE,
            };
            draw_texture_ex(resources.entity_atlas(), pos.x, pos.y, color, DrawTextureParams {
                source: Some(rect),
                ..Default::default()
            });
        };
        match other {
            ObjectOtherKind::Door(DoorKind::Door)                => draw_from_atlas(Rect::new(208.0, 32.0, 16.0, 16.0)),
            ObjectOtherKind::Door(DoorKind::Teleporter)          => draw_from_atlas(Rect::new(208.0, 48.0, 16.0, 16.0)),
            ObjectOtherKind::Door(DoorKind::SeamlessTeleporter)  => draw_from_atlas(Rect::new(208.0, 64.0, 16.0, 16.0)),
            ObjectOtherKind::Sign        => draw_from_atlas(Rect::new(240.0,  0.0, 16.0, 16.0)),
            ObjectOtherKind::Spawn       => draw_from_atlas(Rect::new(208.0, 16.0, 16.0, 16.0)),
            ObjectOtherKind::Finish      => draw_from_atlas(Rect::new(240.0, 16.0, 16.0, 16.0)),
            ObjectOtherKind::Checkpoint  => draw_from_atlas(Rect::new(224.0, 16.0, 16.0, 16.0)),
        }
    }
}
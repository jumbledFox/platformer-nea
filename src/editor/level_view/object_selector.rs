// The screen that lets you select a tile/object, opened/closed by pressing space when editing the level

use macroquad::{color::{Color, WHITE}, color_u8, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{game::{entity::EntityKind, level::{tile::{render_tile, CheckerBlockColor, LockColor, Tile, TileRenderLayer}, TileDrawKind, TileRenderData}}, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{Button, ButtonState, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 100);

const TILES_Y: f32 = 14.0;
const ENTITIES_Y: f32 = 86.0;
const OTHERS_Y: f32 = 170.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Tile(Tile),
    Entity(EntityKind),
    Other(ObjectOtherKind),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectOtherKind {
    Sign,
    Door(bool), // Teleport
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
            Tile::Cloud,
            Tile::Metal,
            Tile::Checker,
            Tile::CheckerBlock(CheckerBlockColor::Cyan),
            Tile::CheckerBlock(CheckerBlockColor::Orange),
            Tile::CheckerBlock(CheckerBlockColor::Purple),
            Tile::WoodenPlatform,
            Tile::MetalPlatform,
            Tile::Bridge,
            Tile::Rope,
            Tile::Door,
            Tile::Ladder,
            Tile::Vine,
            Tile::StoneBlock,
            Tile::Glass,
            Tile::Block,
            Tile::Spikes,
            Tile::Switch(false), Tile::SwitchBlockOff(true), Tile::SwitchBlockOn(false),
            Tile::Lava,
        ];
        let mut entities = vec![
            (EntityKind::Chip, "Chip".to_string()),
            (EntityKind::ChipCrate(false), "Chip crate".to_string()),
            (EntityKind::ChipCrate(true), "Large chip crate".to_string()),
            (EntityKind::Life, "Life".to_string()),
            (EntityKind::LifeCrate, "Life crate".to_string()),
            (EntityKind::Frog, "Frog".to_string()),
            (EntityKind::FrogCrate(false), "Single-frog crate".to_string()),
            (EntityKind::FrogCrate(true), "Multi-frog crate".to_string()),
        ];

        for col in LockColor::colors().iter() {
            tiles.push(Tile::Lock(*col));
            tiles.push(Tile::LockBlock(*col));
            entities.push((EntityKind::Key(*col), format!("{:?} key", col)));
        }
        for col in LockColor::colors().iter() {
            entities.push((EntityKind::KeyCrate(*col), format!("{:?} key crate", col)));
        }

        let others = [
            (ObjectOtherKind::Sign, "Sign"),
            (ObjectOtherKind::Door(false), "Door"),
            (ObjectOtherKind::Door(true), "Teleporter"),
            (ObjectOtherKind::Spawn, "Spawn"),
            (ObjectOtherKind::Finish, "Finish"),
            (ObjectOtherKind::Checkpoint, "Checkpoint"),
        ];

        // Add all of the tiles
        let wrap = 17;
        let mut x = 8.0;
        let mut y = TILES_Y + 12.0;
        for (i, t) in tiles.iter().enumerate() {
            let button = Button::new(Rect::new(x, y, 16.0, 16.0), None, Some(resources.tile_data_manager().data(*t).name().clone()));

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
        let y_centers = [20.0, 40.0];

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
                Object::Entity(e) => e.draw_editor(false, pos, vec2(0.0, 0.0), resources),
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
            ObjectOtherKind::Door(false) => draw_from_atlas(Rect::new(208.0, 32.0, 16.0, 16.0)),
            ObjectOtherKind::Door(true)  => draw_from_atlas(Rect::new(208.0, 48.0, 16.0, 16.0)),
            ObjectOtherKind::Sign        => draw_from_atlas(Rect::new(240.0,  0.0, 16.0, 16.0)),
            ObjectOtherKind::Spawn       => draw_from_atlas(Rect::new(208.0, 16.0, 16.0, 16.0)),
            ObjectOtherKind::Finish      => draw_from_atlas(Rect::new(240.0, 16.0, 16.0, 16.0)),
            ObjectOtherKind::Checkpoint  => draw_from_atlas(Rect::new(224.0, 16.0, 16.0, 16.0)),
        }
    }
}
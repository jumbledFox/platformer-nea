// The screen that lets you select a tile/object, opened/closed by pressing space when editing the level

use macroquad::{color::{Color, WHITE}, color_u8, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{game::level::{tile::{render_tile, CheckerBlockColor, LockColor, Tile, TileRenderLayer}, TileDrawKind, TileRenderData}, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{Button, ButtonState, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 100);

#[derive(Clone, Copy)]
pub enum Object {
    Tile(Tile),
    Entity(EntityType)
}

#[derive(Clone, Copy)]
pub enum EntityType {
    PlayerSpawn,
    Frog,
}

pub struct ObjectSelector {
    active: bool,
    object_buttons: Vec<(Object, Button)>
}

impl ObjectSelector {
    pub fn new(resources: &Resources) -> Self {
        let mut object_buttons = vec![];

        let mut tiles = vec![ 
            Tile::Grass,
            Tile::Stone,
            Tile::Cloud,
            Tile::Metal,
            Tile::Checker,
            Tile::CheckerBlock(CheckerBlockColor::Cyan),
            Tile::CheckerBlock(CheckerBlockColor::Orange),
            Tile::CheckerBlock(CheckerBlockColor::Purple),
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
        ];
        for col in LockColor::colors().iter() {
            tiles.push(Tile::Lock(*col));
            tiles.push(Tile::LockBlock(*col));
        }

        // Add all of the tiles
        let wrap = 17;
        let mut x = 8.0;
        let mut y = 16.0;
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
        // TODO: ....

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

        render_text("Tiles:",    WHITE, vec2(4.0,   4.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
        render_text("Entities:", WHITE, vec2(4.0,  80.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
        render_text("Others:",   WHITE, vec2(4.0, 160.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
        for (o, b) in self.object_buttons.iter() {
            let offset = match b.state() {
                ButtonState::Hovered => -Vec2::ONE,
                _ => Vec2::ZERO,
            };

            match o {
                // If it's a tile, draw it as such
                Object::Tile(t) => render_tile(
                    &TileRenderData { tile: *t, draw_kind: TileDrawKind::Single(0), pos: b.rect().point() + offset},
                    Vec2::ZERO,
                    TileRenderLayer::Foreground(false),
                    resources,
                ),
                _ => {}
            }
            
        }
    }
}
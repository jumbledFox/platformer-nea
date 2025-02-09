// The screen that lets you select a tile/object, opened/closed by pressing space when editing the level

use macroquad::{color::{Color, WHITE}, color_u8, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{game::level::{tile::{render_tile, CheckerBlockColor, LockColor, Tile, TileRenderLayer}, TileDrawKind, TileRenderData}, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{Button, ButtonState, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 100);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Tile(Tile),
    Entity(EntityKind),
    Other(OtherKind),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    PlayerSpawn,
    Frog,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OtherKind {
    Door,
    Sign,
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
            Tile::Dirt,
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

        // Add the others
        let others = vec![OtherKind::Sign, OtherKind::Door];
        let mut x = 8.0;
        let mut y = 172.0;
        for (i, o) in others.iter().enumerate() {
            let button = Button::new(Rect::new(x, y, 16.0, 16.0), None, Some(format!("{:?}", o)));
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

        render_text("Tiles:",    WHITE, vec2(4.0,   4.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
        render_text("Entities:", WHITE, vec2(4.0,  80.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
        render_text("Others:",   WHITE, vec2(4.0, 160.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
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
                Object::Other(o) => ObjectSelector::draw_object_other(pos, *o, false, resources),
                _ => {}
            }
        }
    }

    pub fn draw_object_other(pos: Vec2, other: OtherKind, transparent: bool, resources: &Resources) {
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
            OtherKind::Door => draw_from_atlas(Rect::new(192.0, 0.0, 16.0, 16.0)),
            OtherKind::Sign => draw_from_atlas(Rect::new(240.0, 0.0, 16.0, 16.0)),
        }
    }
}
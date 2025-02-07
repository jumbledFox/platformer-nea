// The screen that lets you select a tile/object, opened/closed by pressing space when editing the level

use macroquad::{color::Color, color_u8, math::{Rect, Vec2}, shapes::draw_rectangle};

use crate::{game::level::{tile::{render_tile, CheckerBlockColor, LockColor, Tile}, TileRenderData}, resources::Resources, ui::{Button, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 100);

// TODO: Maybe not make this constant and add it programmatically
pub const ALLOWED_TILES: &[Tile] = &[
    Tile::Empty, 
    Tile::Door,
    Tile::Grass,
    Tile::Metal,
    Tile::Checker,
    Tile::Cloud,
    Tile::CheckerBlock(CheckerBlockColor::Cyan),
    Tile::CheckerBlock(CheckerBlockColor::Orange),
    Tile::CheckerBlock(CheckerBlockColor::Purple),
    Tile::Bridge,
    Tile::Rope,
    Tile::Ladder,
    Tile::Vine,
    Tile::StoneBlock,
    Tile::Glass,
    Tile::Block,
    Tile::Spikes,
    Tile::Switch(false), Tile::SwitchBlockOff(true), Tile::SwitchBlockOn(false),
    Tile::Lock(LockColor::Red),     Tile::LockBlock(LockColor::Red),
    Tile::Lock(LockColor::Green),   Tile::LockBlock(LockColor::Green),
    Tile::Lock(LockColor::Blue),    Tile::LockBlock(LockColor::Blue),
    Tile::Lock(LockColor::Yellow),  Tile::LockBlock(LockColor::Yellow),
    Tile::Lock(LockColor::White),   Tile::LockBlock(LockColor::White),
    Tile::Lock(LockColor::Black),   Tile::LockBlock(LockColor::Black),
    Tile::Lock(LockColor::Rainbow), Tile::LockBlock(LockColor::Rainbow),
];

pub struct ObjectSelector {
    active: bool,
    object_buttons: Vec<(usize, Button)>
}

impl ObjectSelector {
    pub fn new(resources: &Resources) -> Self {
        let mut object_buttons = vec![];

        let wrap = 17;
        let mut x = 8.0;
        let mut y = 8.0;
        for (i, t) in ALLOWED_TILES.iter().enumerate() {
            let button = Button::new(Rect::new(x, y, 16.0, 16.0), Some(resources.tile_data_manager().data(*t).name().clone()));

            x += 20.0;
            if i % wrap == wrap-1 {
                x = 8.0;
                y += 20.0;
            }
            object_buttons.push((i, button));
        }

        Self { active: false, object_buttons }
    }

    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<Tile> {
        
        for (i, b) in &mut self.object_buttons {
            b.update(ui);
            if b.released() {
                return Some(ALLOWED_TILES[*i]);
            }
        }
        
        None
    }

    pub fn draw(&self, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);

        for (i, b) in self.object_buttons.iter() {
            b.draw();
            render_tile(
                &TileRenderData {
                    tile: ALLOWED_TILES[*i],
                    draw_kind: crate::game::level::TileDrawKind::Single(0),
                    pos: b.rect().point()
                },
                Vec2::ZERO,
                resources
            );
        }
    }
}
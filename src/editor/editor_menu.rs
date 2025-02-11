// The menu for info about the current level, changing the level, and help

use macroquad::{color::{Color, WHITE}, color_u8, input::{is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, ui::{Button, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 100);

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HelpKind {
    Editor,
    LevelPacks,
    Level,
    Resizing,
    ObjectSelector,
    Tiles, BackgroundTiles,
    Entities, Signs, Doors, Teles, SpawnFinish, Checkpoints,

    // Not an actual help page, just so i know how large the enum is without
    // the nightly feature 'std::mem::variant_count'.
    Last,
}

impl From<HelpKind> for u8 {
    fn from(value: HelpKind) -> Self {
        value as u8
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum HelpScreen {
    Closed,
    OpenFromMenu,
    OpenFromKeybind,
}

pub struct EditorMenu {
    active: bool,

    // The button to open the help screen
    help_button: Button,

    // The help screen
    help_page:   u8,
    help_screen: HelpScreen,
    help_button_prev:  Button,
    help_button_next:  Button,
    help_button_close: Button,
}

impl Default for EditorMenu {
    fn default() -> Self {
        Self {
            active: true,

            help_button: Button::new(Rect::new(5.0, 5.0, 54.0, 12.0), Some(String::from("Help!")), None),

            help_page: 0,
            help_screen: HelpScreen::OpenFromKeybind,
            help_button_prev:  Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 40.0 - 12.0, 190.0, 12.0, 12.0), Some(String::from("🮤")), None),
            help_button_next:  Button::new(Rect::new(VIEW_SIZE.x / 2.0 + 40.0,        190.0, 12.0, 12.0), Some(String::from("🮥")), None),
            help_button_close: Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 54.0 / 2.0,  190.0, 54.0, 12.0), Some(String::from("Close")), None),
        }
    }
}

impl EditorMenu {
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, active: bool) {
        if active {
            self.help_screen = HelpScreen::Closed;
        }
        self.active = active
    }

    pub fn open_help_menu(&mut self, help_kind: HelpKind) {
        self.active = true;
        self.help_page = help_kind as u8;
        self.help_screen = HelpScreen::OpenFromKeybind;
    }

    fn update_help_screen(&mut self, ui: &mut Ui) {
        // Make it so the next/prev buttons are disabled if you're at either end
        self.help_button_prev.set_disabled(self.help_page == 0);
        self.help_button_next.set_disabled(self.help_page == HelpKind::Last as u8 - 1);

        // Update the buttons
        self.help_button_prev.update(ui);
        self.help_button_next.update(ui);
        self.help_button_close.update(ui);

        // Handle them being pressed accordingly...
        if self.help_button_prev.released() {
            self.help_page -= 1;
        }
        if self.help_button_next.released() {
            self.help_page += 1;
        }
        // Close the menu if the close button OR the 'h' key is pressed
        if self.help_button_close.released() || is_key_pressed(KeyCode::H) {
            // If we've opened the help screen from a keybind, rather than from the menu
            // When closing it we want to also close the menu
            if self.help_screen == HelpScreen::OpenFromKeybind {
                self.active = false;
            }
            self.help_screen = HelpScreen::Closed;
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        // Update the help screen, and don't update anything else if it's still open
        if self.help_screen != HelpScreen::Closed {
            self.update_help_screen(ui);

            if self.help_screen != HelpScreen::Closed {
                return;
            }
        }
        // If we're no-longer active, don't update anything
        // IDK man this could cause a frame delay because of closing the help menu blllaaaahhhrhrhghhh
        if !self.active {
            return;
        }

        self.help_button.update(ui);

        if self.help_button.released() {
            self.help_screen = HelpScreen::OpenFromMenu;
        }
    }

    fn draw_help_screen(&self, resources: &Resources) {
        self.help_button_prev.draw(resources);
        self.help_button_next.draw(resources);
        self.help_button_close.draw(resources);

        /*
        Editor,
        LevelPacks,
        Level,
        Resizing,
        ObjectSelector,
        Tiles, BackgroundTiles,
        Entities, Signs, Doors, SpawnFinish, Checkpoints,
        */
        let (title, lines): (&str, &[&str]) = match self.help_page {
            p if p == HelpKind::Editor as u8 => ("The editor", &[
                //-----------------------------------//
                "Welcome to the editor! It has many",
                "features, so I strongly encourage you",
                "to read the help menu.",
                "",
                "Press 'h' when editing the level to",
                "open the help menu, the page will",
                "correspond to the currently selected",
                "object!",
                "",
                "Happy editing! :3"
            ]),
            p if p == HelpKind::LevelPacks as u8 => ("Level packs", &[
                //-----------------------------------//
                "The editor lets you to create a pack",
                "of levels.",
                "",
                "The menu (brought up by pressing",
                "'ESCAPE') allows you to:",
                " - Add / remove levels.",
                " - Reorder levels in the pack.",
                " - Change the level you're currently",
                "   editing.",
                " - Access this help menu.",
            ]),
            p if p == HelpKind::Level as u8 => ("Level", &[
                //-----------------------------------//
                "To move the camera around the level,",
                "hold the middle mouse button and drag",
                "or use the WASD keys.",
                "",
                "The level can be resized (mentioned",
                "on the next help page)."
            ]),
            p if p == HelpKind::Resizing as u8 => ("Resizing", &[
                //-----------------------------------//
                "The level can be resized by pressing",
                "the buttons along the borders of the",
                "level.",
                "",
                "There's a minimum and maximum level",
                "size, going from 1 screen of tiles to",
                "255*255 tiles.",
                "",
                "When resizing the level, all objects",
                "(tiles/entities/others) that become",
                "out of bounds will be deleted (with a",
                "few exceptions mentioned elsewhere).",
            ]),
            p if p == HelpKind::ObjectSelector as u8 => ("Object selector", &[
                //-----------------------------------//
                "Press 'space' when editing the level",
                "to open the object selector.",
                "",
                "You'll be greeted by a bunch of",
                "objects arranged neatly, click on one",
                "to select it.",
                "",
                "Objects can be tiles, entities, or",
                "other special kinds like doors, signs",
                "and checkpoints."
            ]),
            p if p == HelpKind::Tiles as u8 => ("Tiles", &[
                //-----------------------------------//
                
            ]),
            p if p == HelpKind::BackgroundTiles as u8 => ("Background tiles", &[
                //-----------------------------------//
                
            ]),
            p if p == HelpKind::Entities as u8 => ("Entities", &[
                //-----------------------------------//

            ]),
            p if p == HelpKind::Signs as u8 => ("Signs", &[
                //-----------------------------------//
                "With the sign object selected:",
                "", "",
                "Left click to place down a sign, a",
                "menu will open.",
                "",
                "Right click a sign to delete it.",
                "",
                "Hover over a sign and press 'C' to",
                "'copy' it or 'X' to 'cut' it, the",
                "tool outline will change to green",
                "or purple, showing copy/cut status."
            ]),
            p if p == HelpKind::Doors as u8 => ("Doors", &[
                //-----------------------------------//
                "With the door object selected:",
                "", "",
                "Left click to place the door's start",
                "position. Right click to cancel.",
                "Left click again to finish the door",
                "and add it to the world.",
                "",
                "When not making a door, right click a",
                "door's start position to delete it.",
                "",
                "Remember: doors can't be used in air!"
            ]),
            p if p == HelpKind::Teles as u8 => ("Teleporters", &[
                //-----------------------------------//,
                "The teleporter object works in the",
                "exact same way as the door object",
                "(previous page).",
                "", "",
                "The only gameplay differences are",
                "that teleporters work instantly (not",
                "requiring any input from the player)",
                "and they don't require the player to",
                "be stood on ground to work.",
            ]),
            p if p == HelpKind::SpawnFinish as u8 => ("Spawn/Finish", &[
                //-----------------------------------//
                "The spawn and finish are the points",
                "where the player first spawns in,",
                "and where they complete the level.",
                "", "", "",
                "They always exist and cannot be",
                "removed. If the level resizes and they",
                "are on the edge, they are pushed back",
                "to being in the bounds of the level.",
            ]),
            p if p == HelpKind::Checkpoints as u8 => ("Checkpoints", &[
                //-----------------------------------//
                
            ]),

            _ => ("help screen", &["page error!! ???"])
        };

        render_text("Help menu", WHITE, vec2(VIEW_SIZE.x/2.0, 12.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        render_text(&title, Color::from_rgba(250, 135, 0, 255), vec2(VIEW_SIZE.x/2.0, 42.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
        for (i, line) in lines.iter().enumerate() {
            render_text(&line, WHITE, vec2(24.0, 62.0 + i as f32 * 10.0), Vec2::ONE, Align::End, Font::Small, resources);
        }
    }

    pub fn draw(&self, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);

        if self.help_screen != HelpScreen::Closed {
            self.draw_help_screen(resources);
            return;
        }
        self.help_button.draw(resources);
    }
}
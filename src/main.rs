use std::{thread::sleep, time::Duration};

use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{BLACK, WHITE}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, render_target, set_default_filter_mode, DrawTextureParams, FilterMode}, time::get_frame_time, window::{clear_background, next_frame, Conf}};
use menu::Menu;
use resources::Resources;
use ui::Ui;

pub mod util;
pub mod resources;
pub mod text_renderer;
pub mod ui;
pub mod level_pack_data;

// The different game states
pub mod menu;
pub mod game;
pub mod editor;

// How many tiles the screen should be
pub const VIEW_WIDTH:  usize = 22;
pub const VIEW_HEIGHT: usize = 14;
pub const VIEW_SIZE: Vec2 = vec2((VIEW_WIDTH * 16) as f32, (VIEW_HEIGHT * 16) as f32);

fn window_conf()-> Conf {
    let scale_factor = 4;
    Conf { 
        window_title: String::from("Fox Game !! :3"),
        window_width:  VIEW_WIDTH  as i32 * 16 * scale_factor,
        window_height: VIEW_HEIGHT as i32 * 16 * scale_factor,
        ..Default::default()
    }
}

pub trait GameState {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources, next_state: &mut Option<Box<dyn GameState>>);
    fn draw(&self, ui: &Ui, resources: &Resources, debug: bool);
}

#[macroquad::main(window_conf())]
async fn main() {
    // Seed the randomness
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);

    set_default_filter_mode(FilterMode::Nearest);
    let mut resources = Resources::default();

    let render_target = render_target(VIEW_WIDTH as u32 * 16, VIEW_HEIGHT as u32 * 16);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut ui = Ui::new();

    let mut world_cam = Camera2D::from_display_rect(
        Rect::new(
            0.0,
            0.0,
            VIEW_WIDTH  as f32 * 16.0,
            VIEW_HEIGHT as f32 * 16.0,
        ),
    );
    world_cam.render_target = Some(render_target.clone());

    let mut debug = false;
    // let mut game_state: Box<dyn GameState> = Box::new(Editor::new(&resources));
    let mut game_state: Box<dyn GameState> = Box::new(Menu::new(None));
    let mut next_state: Option<Box<dyn GameState>> = None;

    loop {
        ui.begin_frame();

        // Toggling debug mode
        if is_key_pressed(macroquad::input::KeyCode::Key0) {
            debug = !debug;
        }

        // Update the game state
        let deltatime = get_frame_time();
        resources.update_tile_animation_timer(deltatime);
        game_state.update(deltatime, &mut ui, &mut resources, &mut next_state);

        if let Some(state) = next_state.take() {
            game_state = state;
            game_state.update(deltatime, &mut ui, &mut resources, &mut next_state);
        }

        // Draw to the render target
        set_camera(&world_cam);
        game_state.draw(&ui, &resources, debug);
        ui.draw(&resources);

        // Draw the render target
        let r = Ui::render_target_rect();
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(&render_target.texture, r.x, r.y, WHITE, DrawTextureParams {
            dest_size: Some(r.size()),
            flip_y: true,
            ..Default::default()
        });

        // Wait for the next frame
        // We sleep here to stop macroquad from going over ~60 fps, which would be pointless and hog the CPU
        // TODO:... remove this
        if !is_key_down(KeyCode::F) {
            // sleep(Duration::from_millis(6));
        } else {
            sleep(Duration::from_millis(100));
        }
        next_frame().await
    }
}

use std::{thread::sleep, time::Duration};

use editor::Editor;
use game::Game;
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{Color, BLACK, WHITE}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, render_target, set_default_filter_mode, DrawTextureParams, FilterMode}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use resources::Resources;
use ui::Ui;

pub mod util;
pub mod resources;
pub mod text_renderer;
pub mod ui;
pub mod game;
pub mod editor;

// How many tiles the screen should be
pub const VIEW_WIDTH:  usize = 22;
pub const VIEW_HEIGHT: usize = 14;
pub const VIEW_SIZE: Vec2 = vec2((VIEW_WIDTH * 16) as f32, (VIEW_HEIGHT * 16) as f32);

fn window_conf()-> Conf {
    let scale_factor = 4;
    Conf { 
        window_title: String::from("Fox :3"),
        window_width:  VIEW_WIDTH  as i32 * 16 * scale_factor,
        window_height: VIEW_HEIGHT as i32 * 16 * scale_factor,
        ..Default::default()
    }
}

pub trait GameState {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &Resources);
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

    let mut debug = true;
    let mut game_state: Box<dyn GameState> = Box::new(Editor::new(&resources));

    loop {
        ui.begin_frame();

        if is_key_pressed(macroquad::input::KeyCode::Key0) {
            debug = !debug;
        }

        // Update the game state
        let deltatime = get_frame_time();
        resources.update_tile_animation_timer(deltatime);
        game_state.update(deltatime, &mut ui, &resources);

        // Draw to the render target
        set_camera(&world_cam);
        clear_background(Color::from_hex(0x6dcaff));

        game_state.draw(&ui, &resources, debug);
        ui.draw(&resources);

        // Draw the render target
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(&render_target.texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            flip_y: true,
            ..Default::default()
        });

        // Wait for the next frame
        // We sleep here to stop macroquad from going over ~60 fps, which would be pointless and hog the CPU
        if !is_key_down(KeyCode::F) {
            // sleep(Duration::from_millis(14));
        } else {
            sleep(Duration::from_millis(100));
        }
        next_frame().await
    }
}

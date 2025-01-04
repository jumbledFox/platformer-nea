use std::{thread::sleep, time::Duration};

use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{Color, BLACK, WHITE}, math::{vec2, Rect}, texture::{draw_texture_ex, render_target, set_default_filter_mode, DrawTextureParams, FilterMode}, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width}};
use resources::Resources;
use scene::Scene;

pub mod util;
pub mod resources;
pub mod text_renderer;
pub mod level;
pub mod game;
pub mod scene;

// How many tiles the screen should be
pub const VIEW_WIDTH:  usize = 22;
pub const VIEW_HEIGHT: usize = 14;

#[macroquad::main("Fox")]
async fn main() {
    // Seed the randomness
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);

    set_default_filter_mode(FilterMode::Nearest);
    let mut resources = Resources::default();

    let render_target = render_target(VIEW_WIDTH as u32 * 16, VIEW_HEIGHT as u32 * 16);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut world_cam = Camera2D::from_display_rect(
        Rect::new(
            0.0,
            0.0,
            VIEW_WIDTH  as f32 * 16.0,
            VIEW_HEIGHT as f32 * 16.0,
        ),
    );
    world_cam.render_target = Some(render_target.clone());

    let mut test_scene = Scene::default();
    test_scene.foo();

    loop {
        let deltatime = get_frame_time();
        resources.update_tile_animation_timer(deltatime);
        test_scene.update(deltatime);

        // Draw to the render target
        set_camera(&world_cam);
        clear_background(Color::from_hex(0x6dcaff));

        test_scene.draw(4, &resources);

        // Draw render target
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(&render_target.texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            flip_y: true,
            ..Default::default()
        });

        // Wait for the next frame
        // We sleep here to stop macroquad from going over ~60 fps, which would be pointless and hog the CPU
        sleep(Duration::from_millis(14));
        next_frame().await
    }
}

use std::{thread::sleep, time::Duration};

use level::Level;
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{BLACK, BLUE, RED, WHITE}, math::{vec2, Rect}, texture::{draw_texture_ex, render_target, set_default_filter_mode, DrawTextureParams, FilterMode}, window::{clear_background, next_frame, screen_height, screen_width}};
use resources::Resources;
use text_renderer::{render_text, Align};

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
    set_default_filter_mode(FilterMode::Nearest);
    let resources = Resources::default();

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

    let mut test_level = Level::default();
    test_level.prepare_tiles();

    loop {
        // Draw to the render target
        set_camera(&world_cam);
        clear_background(BLUE);

        Level::render_tiles(test_level.tiles_below(), resources.tiles_atlas());

        render_text("text renderer is WORKING! :3", RED, vec2(190.0, 40.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());

        // Draw render target
        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(&render_target.texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            flip_y: true,
            ..Default::default()
        });

        // Wait for the next frame
        sleep(Duration::from_millis(14));
        next_frame().await
    }
}

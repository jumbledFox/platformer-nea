use std::{thread::sleep, time::Duration};

use level::Level;
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{BLACK, BLUE, GREEN, ORANGE, RED, WHITE}, math::{vec2, Rect}, texture::{draw_texture_ex, render_target, set_default_filter_mode, DrawTextureParams, FilterMode}, window::{clear_background, next_frame, screen_height, screen_width}};
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
    test_level.update_tile_render_data();

    loop {
        // Draw to the render target
        set_camera(&world_cam);
        clear_background(BLUE);

        Level::render_tiles(test_level.tiles_below(), resources.tiles_atlas());

        render_text("- fox -", ORANGE, vec2(40.0, 8.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("  * 3", WHITE, vec2(40.0, 24.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("BOOTS", WHITE, vec2(176.0, 10.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("HELMET", WHITE, vec2(176.0, 22.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("420", WHITE, vec2(305.0, 3.0), vec2(1.0, 1.0), Align::End, resources.font_atlas());
        render_text("69", GREEN, vec2(305.0, 19.0), vec2(1.0, 1.0), Align::End, resources.font_atlas());

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

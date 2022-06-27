#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod player;
mod debug;
mod tile_sheet;
mod tile_map;
use player::PlayerPlugin;
use debug::DebugPlugin;
use tile_sheet::TileSheetPlugin;
use tile_map::TileMapPlugin;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))//clear color
        .insert_resource(WindowDescriptor {//basic window properties
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        //.add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileSheetPlugin)
        .add_plugin(TileMapPlugin)
        .add_system(toggle_inspector)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;//so pixel look stays

    commands.spawn_bundle(camera);
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}

#[allow(dead_code)]
fn input_line(buffer: &mut [u16]){
    let mut text = String::new();
    std::io::stdin()
    .read_line(&mut text)
    .expect("Cannot read line.");
    let text_size = text.len().min(buffer.len());//whatever is smaller text or buffer
    for (i, word) in buffer.iter_mut().enumerate().take(text_size)//until elements of "text_size" are no more
    {
        *word = text.as_bytes()[i].into();//assign to each place in the buffer text as_bytes that are transformed/casted into ascii numerical value
    }

    for word in buffer.iter_mut().skip(text.len())
    {
        *word = 0;//set every value that is bigger than text's length to 0 (in case buffer space in bigger than text's)
    }

}

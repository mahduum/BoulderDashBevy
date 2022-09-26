#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod prelude
{
    //put extern crates here
    use bevy_ecs_tilemap::prelude::*;

	pub const SCREEN_WIDTH: i32 = 80;
	pub const SCREEN_HEIGHT: i32 = 50;
	pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
	pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use animate_sprites::AnimateSpritesPlugin;
use bevy::{prelude::*, render::{texture::ImageSettings}, window::PresentMode};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod components;
mod diamond;
mod animate_sprites;
mod player;
mod debug;
mod tile_sheet;
mod tile_map;
mod plugins;

use player::PlayerPlugin;
use plugins::{player_input::PlayerInputPlugin, movement::MovementPlugin};
use debug::DebugPlugin;
use tile_sheet::TileSheetPlugin;
use tile_map::TileMapPlugin;
use bevy_ecs_tilemap::TilemapPlugin;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_SCALE: f32 = 0.01;
pub const TILE_SIZE_SCALED: f32 = 16.0 * 0.01;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum MyLabel {
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    /// everything that moves things (works with transforms)
    Movement,
    /// systems that update the world map
    Camera,
}

//todo: is there a function in plugin that remeber what is on what tile?
fn main() {
    App::new()
        //.insert_resource(ClearColor(CLEAR))//clear color
        .insert_resource(WindowDescriptor {//basic window properties
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            // present_mode: PresentMode::Fifo,
            // resizable: false,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        //.add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileSheetPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(TilemapPlugin)
        .add_system(toggle_inspector)
        .add_plugin(AnimateSpritesPlugin)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(MovementPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {

    let projection = OrthographicProjection {
        left: -1.0 * RESOLUTION,
        right: 1.0 * RESOLUTION,
        bottom: -1.0,
        top: 1.0,
        far: 1000.0,
        ..Default::default()};

    commands.spawn_bundle(Camera2dBundle{
        projection,
        ..Default::default()
    });
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

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn_bundle(Camera2dBundle::default());
//     commands.spawn_bundle(SpriteBundle {
//         texture: asset_server.load("branding/icon.png"),
//         ..default()
//     });
// }

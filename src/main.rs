#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
//#![feature(box_into_inner)]

mod prelude
{
    pub use dyn_clone::DynClone;
    pub use std::borrow::BorrowMut;
    use bevy::prelude::Color;
    //put extern crates here
    pub use bevy_ecs_tilemap::prelude::*;
    pub use crate::components::*;
    pub use crate::resources::*;

	pub const SCREEN_WIDTH: i32 = 80;
	pub const SCREEN_HEIGHT: i32 = 50;
	pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
	pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
    pub const HEIGHT: f32 = 1024.0;
    pub const RESOLUTION: f32 = 16.0 / 9.0;
    pub const TILE_SIZE: f32 = 32.0;
    pub const TILE_SCALE: f32 = 0.005;
    pub const TILE_SIZE_SCALED: f32 = 32.0 * 0.005;//final scale is 0.0025
}

use std::time::Duration;
use relocate_components::RelocateComponentsPlugin;
use bevy::{prelude::*, window::PresentMode};
use bevy::render::camera::ScalingMode;
use bevy::sprite::Material2dPlugin;
use bevy::utils::define_label;
use bevy::window::{ExitCondition, WindowResolution};


//use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod components;
mod relocate_components;
mod debug;
mod tile_sheet;
mod tile_map;
mod plugins;
mod camera_follow;
mod resources;

use plugins::{player_input::PlayerInputPlugin, movement::MovementPlugin};
use debug::DebugPlugin;
use tile_sheet::TileSheetPlugin;
use tile_map::TileMapPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use crate::camera_follow::CameraFollowPlugin;
use prelude::*;
use crate::plugins::dig_tunnel::DigTunnelPlugin;
use crate::plugins::player_input::{InputDelayTimer};
use crate::plugins::sprite_animation::SpriteAnimationPlugin;
use crate::resources::sprite_sequences_resource::SpriteAnimationSequences;
use crate::plugins::animation_state::AnimationStatePlugin;
use crate::TimerMode::Repeating;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
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


#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
#[system_set(base)]
pub enum MovementStage{//todo not yet implemented???
    Moving,//before
    Digging//after
}

//todo: is there a function in plugin that remeber what is on what tile?
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Option::from(Window {
                        resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                        position: WindowPosition::Centered(MonitorSelection::Index(0)),
                        ..default()
                    }),
                    close_when_requested: true,
                    exit_condition: ExitCondition::OnPrimaryClosed
                }))
                    //..default()))
        // .insert_resource(WorldInspectorParams {//todo update for bevy 0_9
        //     enabled: false,
        //     ..Default::default()
        // })
        //.add_plugin(WorldInspectorPlugin::new())
        //.add_startup_system(spawn_camera)//move to system post process
        .add_plugin(Material2dPlugin::<plugins::post_process::EightBitPostProcessingMaterial>::default())
        .add_plugin(plugins::post_process::PostProcessPlugin)
        .add_plugin(CameraFollowPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileSheetPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(TilemapPlugin)
            //.add_system(toggle_inspector)//todo not available in bevy 0.9
        .add_plugin(RelocateComponentsPlugin)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(DigTunnelPlugin)
        .add_plugin(SpriteAnimationPlugin)
        .add_plugin(AnimationStatePlugin)
        .init_resource::<SpriteAnimationSequences>()
        .insert_resource(InputDelayTimer(Timer::from_seconds(0.2, Repeating)))
            // .configure_set
            // (
            //     MovementStage::Digging
            //             .after(CoreSet::UpdateFlush)
            //             .before(CoreSet::PostUpdate)
            // )
            // .configure_set
            // (
            //     MovementStage::Moving
            //             .after(CoreSet::PreUpdateFlush)
            //             .before(CoreSet::Update)
            // )
            .configure_sets
            ((
                    CoreSet::PreUpdateFlush,
                    MovementStage::Moving,
                    CoreSet::PreUpdateFlush,
                    CoreSet::UpdateFlush,
                    MovementStage::Digging,
                    CoreSet::PostUpdate
            ))//.chain().add_system(some_system.in_base_set(MovementStage::Digging));
        //.add_stage_before(CoreSet::Update, MovementStage::Moving, SystemStage::parallel())//todo require configure_set
        //.add_stage_after(CoreSet::Update, MovementStage::Digging, SystemStage::parallel())//todo require configure_set
        .run();
}

fn spawn_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection{
            scaling_mode: ScalingMode::FixedVertical(RESOLUTION),
            ..Default::default()
        },
        ..Default::default()
    });
}
// todo: not compiling in Bevy 0.9
// fn toggle_inspector(
//     input: ResMut<Input<KeyCode>>,
//     mut window_params: ResMut<WorldInspectorParams>,
// ) {
//     if input.just_pressed(KeyCode::Grave) {
//         window_params.enabled = !window_params.enabled
//     }
// }

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


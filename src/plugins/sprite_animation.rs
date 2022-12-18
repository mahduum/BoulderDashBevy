use std::ops::{Deref, RemAssign};
use std::thread::current;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use crate::{App, CoreStage, SpriteAnimationSequence, SpriteAnimationSequences};

#[derive(Default)]
pub struct SpriteAnimationPlugin;


impl Plugin for SpriteAnimationPlugin{
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::PostUpdate, sprite_animation_player);
	}
}

pub fn sprite_animation_player(
	sequences: Res<SpriteAnimationSequences>,//todo change sequences to be taken fom files
	time: Res<Time>,
	mut sprite_animation_players: Query<(Entity, &mut SpriteAnimationPlayer)>,//player will
	mut texture_indices: Query<&mut TileTextureIndex>
){
	//todo do the query both styles: set of components, and just anim players and then tileIndexQuery.get_mut(entity) to Ok(result)
	const FRAMES_PER_SECOND: f32 = 12.;
	//it is useful if we need for example components or data from the children od that entity
	for (entity, mut anim_player) in &mut sprite_animation_players{
		//if elapsed has elapsed enough given the speed then advance the index
		if let Some(sequence) = sequences.sequences.get(&anim_player.sequence_name){
			if anim_player.paused && !anim_player.is_changed() {
				continue;
			}

			//todo: with elapsed time it will run and refresh every frame, but instead a global timer can be set
			if !anim_player.paused {
				anim_player.elapsed += time.delta_seconds() * anim_player.speed;
			}

			let mut elapsed = anim_player.elapsed;
			let sequence_duration = sequence.len() as f32/FRAMES_PER_SECOND;
			if anim_player.repeat {
				elapsed %= sequence_duration;
			}
			if elapsed < 0.0{
				elapsed += sequence_duration;
			}
			assert!(elapsed < sequence_duration);

			let percentage = (elapsed/sequence_duration);//percentage from 0 to 10
			//from 0 to 8 by percentage
			let remap = sequence.len() as f32 * percentage;
			let mut current_frame = remap as usize;
			current_frame = if current_frame < 0 {0} else {current_frame};

			let current_index = if (current_frame) >= sequence.len()
			{
				sequence[0]
			}else{
				sequence[current_frame]
			};

			if let Ok(mut texture_index) = texture_indices.get_mut(entity){
				if texture_index.0 != current_index {
					*texture_index = TileTextureIndex(current_index as u32);
				}
			}

			//TODO OR:
			//simply add 1 frame in a fixed timer update
		}
	}
}

/// Animation controls
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpriteAnimationPlayer {
	paused: bool,
	repeat: bool,
	speed: f32,
	elapsed: f32,
	current_index: Option<u32>,
	sequence_name: Name
}

impl Default for SpriteAnimationPlayer {
	fn default() -> Self {
		Self {
			paused: false,
			repeat: true,
			speed: 1.0,
			elapsed: 0.0,
			current_index: None,//todo first in array
			sequence_name: Default::default(),//it works that way that system scans for all players, and works only on the ones with Some as sequence
		}
	}
}

impl Clone for SpriteAnimationPlayer {
	fn clone(&self) -> Self {
		let str = self.sequence_name.deref().to_owned();
		SpriteAnimationPlayer{
			paused: self.paused,
			repeat: self.repeat,
			speed: self.speed,
			elapsed: self.elapsed,
			current_index: self.current_index,
			sequence_name: Name::new(str),
		}
	}
}

impl SpriteAnimationPlayer {
	pub fn new(sequence_name: Name) -> Self{
		SpriteAnimationPlayer{
			sequence_name,
			..Default::default()
		}
	}

	/// Start playing an animation, resetting state of the player
	pub fn start(&mut self, sequence_name: Name) -> &mut Self {
		*self = Self {
			sequence_name,
			..Default::default()
		};
		self
	}

	/// Start playing an animation, resetting state of the player, unless the requested animation is already playing.
	pub fn play(&mut self, sequence: Name) -> &mut Self {
		if self.sequence_name != sequence || self.is_paused() {
			self.start(sequence);
		}
		self
	}

	/// Set the animation to repeat
	pub fn repeat(&mut self) -> &mut Self {
		self.repeat = true;
		self
	}

	/// Stop the animation from repeating
	pub fn stop_repeating(&mut self) -> &mut Self {
		self.repeat = false;
		self
	}

	/// Pause the animation
	pub fn pause(&mut self) {
		self.paused = true;
	}

	/// Unpause the animation
	pub fn resume(&mut self) {
		self.paused = false;
	}

	/// Is the animation paused
	pub fn is_paused(&self) -> bool {
		self.paused
	}

	/// Speed of the animation playback
	pub fn speed(&self) -> f32 {
		self.speed
	}

	/// Set the speed of the animation playback
	pub fn set_speed(&mut self, speed: f32) -> &mut Self {
		self.speed = speed;
		self
	}

	/// Time elapsed playing the animation
	pub fn elapsed(&self) -> f32 {
		self.elapsed
	}

	/// Seek to a specific time in the animation
	pub fn set_elapsed(&mut self, elapsed: f32) -> &mut Self {
		self.elapsed = elapsed;
		self
	}
}


//todo implementation template for assets:
// impl Plugin for AnimationPlugin {
// 	fn build(&self, app: &mut App) {
// 		app.add_asset::<AnimationClip>()
// 		   .register_asset_reflect::<AnimationClip>()
// 		   .register_type::<SpriteAnimationPlayer>()
// 		   .add_system_to_stage(
// 			   CoreStage::PostUpdate,
// 			   animation_player.before(TransformSystem::TransformPropagate),
// 		   );
// 	}
// }

//from animated_fox init assets template:
// #[derive(Resource)]
// struct Animations(Vec<Handle<AnimationClip>>);
//
// fn setup(
// 	mut commands: Commands,
// 	asset_server: Res<AssetServer>,
// 	mut meshes: ResMut<Assets<Mesh>>,
// 	mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
// 	// Insert a resource with the current scene information
// 	commands.insert_resource(Animations(vec![
// 		asset_server.load("models/animated/Fox.glb#Animation2"),
// 		asset_server.load("models/animated/Fox.glb#Animation1"),
// 		asset_server.load("models/animated/Fox.glb#Animation0"),
// 	]));
// }
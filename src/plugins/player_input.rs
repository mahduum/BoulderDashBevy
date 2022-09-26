use std::io::stderr;
use crate::{
	tile_map::TileCollider,
	tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
	TILE_SCALE,
	//tilemap::TileCollider,
	TILE_SIZE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_inspector_egui::Inspectable;
use crate::components::Delta;
use crate::player::Player;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin{
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::First, keyboard_input);
	}
}

//just sending an intent where to move and if to move
//if there is an obstacle then accumulate move intents up to a number, or a time in frames/seconds the button is being held
//use just pressed to receive current input for constant movement
//movement speed will be how often the message is sent
//tile position every entity that has it and is movable, in this case player
//add wants to move to player entity (entity with player, tilepos,
//movement adds to entity component Delta + TilePos = destination
fn keyboard_input(
	mut player_query: Query<Entity, (With<Player>, With<TilePos>)>,
	//wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>, //with constraint on the collider because we are not using data on it
	mut keyboard: ResMut<Input<KeyCode>>,
	time: Res<Time>,
	mut commands: Commands
) {
	//add tick frequency:
	//...

	//first register released
	//allow only one can be pressed

	//check first horizontal then verical
	//keyboard.get_just_pressed(). and order according to ifs
	if keyboard.get_just_released().len() > 0 {
		println!("Clearing input.");
		keyboard.clear();
	}

	if keyboard.get_just_pressed().len() > 0 {
		println!("Just pressed count: {}", keyboard.get_just_pressed().len())
	}

	let mut delta = Delta::zero();

	if keyboard.just_pressed(KeyCode::A) {
		println!("Pressed A");
		delta.x = -1;
	}

	if delta == Delta::zero() && keyboard.just_pressed(KeyCode::D) {
		println!("Pressed D");
		delta.x = 1;
	}

	if delta == Delta::zero() && keyboard.just_pressed(KeyCode::W) {
		println!("Pressed W");
		delta.y = 1;
	}

	if delta == Delta::zero() && keyboard.just_pressed(KeyCode::S) {
		println!("Pressed S");
		delta.y = -1;
	}

	if delta == Delta::zero(){
		return;
	}

	match player_query.get_single_mut(){
		Ok(entity) => {
			commands
					.entity(entity)
					.insert(delta);
		}
		Err(msg) => eprintln!("Couldn't find player entity: {}", msg),
	}
}
use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Query};// ParallelSystemDescriptorCoercion};
use bevy::time::{Timer, TimerMode};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy::prelude::IntoSystemDescriptor;
use crate::animate_sprites::{Animatable, AnimatableGeneric};
use crate::{animate_sprites, Commands, CoreStage, MyLabel};
use crate::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin{
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::Update, movement.label("movement"));
	}
}

//process movement for all movable objects
fn movement(
	//query tiles because we are modifying what they are displaying
	mut delta_query: Query<(Entity, &Delta, &TilePos, Option<&mut Player>)>,
	tile_storage: Query<(&TileStorage)>,
	mut commands: Commands
){

	//tile replacement - which ever tile has the player component will display its animation if it can be entered, and its data modified
	//first calculate destination from delta
	//check if delta entity has player

	//player can be a separate tile layer or bundle layed on top of the map
	//player can be a more complex component with it's own animatable so we remove and add a component with given settings
	//resource keep data about player actual and planned whereabouts, animatable will have reassigned index and provider
	//player component is only for the camera to follow

	//make query for player and another for tile storage

	let mut destination_info: Option<(Entity, TilePos, TilePos)> = None;
	delta_query.iter_mut().for_each(|(e, delta, pos, player)|{
		if let Some(mut found_player) = player {
			//if tile position is the player position then change its display only or its enum type
			//entity can be accessed from tile storage "get" by tile by using helpers "pos_2d_to_index"
			//(d, _) (d, p)
			//(d, p) adds another delta when coming back, because the previous wasn't removed, that is why there was two entities
			destination_info =  Some((e, *pos, TilePos::new((pos.x as i32 + delta.x) as u32, (pos.y as i32 + delta.y) as u32)));
		}
	});

	match destination_info{
		Some(info) => {
			let storage = tile_storage.single();
			if let Some(move_to_entity) = storage.get(&info.2){
				//todo how to split animation update from movement??? sieve through components with this type of animation and update them depending on whether they still have the owning animation component
				//todo if we have many entities of the same type (enemy) how we know whose animation timer and indexes are to be passed?
				//example: the owning component (enemy) was transferred to a different entity, so then we find some anim component: how do we know to which entity we pass over its data?
				//we must create transfer messages on movement complete, tuples (from entity, to entity) and then arrange for the passing of the infos in a separate system???
				//add component DataTransfer from entity to entity and each transferable system that has need of transferring its data will be able to do it on its own
				//we move from a to b, we
				//add system for clearing passage, change entity type wherever rockford was
				commands.entity(info.0).remove::<Player>().remove::<Delta>().insert(DataTransfer::move_to(move_to_entity));
				commands.entity(move_to_entity).insert(Player::new())
						.insert(animate_sprites::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
						// .insert(animate_sprites::AnimatableGeneric {
						// 	current_index: 0,
						// 	sprite_index_provider: Box::new(
						// 		RockfordAnimation {
						// 			timer: animate_sprites::AnimationTimer(Timer::from_seconds(0.1, true))
						// 		})
						// });
				//continue with adding animatables
			}
		}
		_ => {}
	}

}
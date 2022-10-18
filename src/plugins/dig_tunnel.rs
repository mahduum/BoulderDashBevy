use bevy::app::Plugin;
use bevy::prelude::{Query, RemovedComponents};
use crate::{App, CoreStage, ParallelSystemDescriptorCoercion};
use crate::tile_map::TileType;
use crate::prelude::*;

pub struct DigTunnelPlugin;

impl Plugin for DigTunnelPlugin{
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::PreUpdate, dig);
	}
}

//Todo: this should be updated before player enters the undug tile
fn dig(removed: RemovedComponents<Player>, mut query: Query<(&mut TileType)>){
	removed.iter().for_each(
		|e|
		{
			match query.get_mut(e) {
				Ok(mut tile_type) => *tile_type = TileType::Tunnel,
				_ => {}
			}
		}
	);
}
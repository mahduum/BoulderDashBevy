use bevy::app::Plugin;
use bevy::prelude::*;
use crate::{App, MovementStage};//ParallelSystemDescriptorCoercion};
use crate::tile_map::TileType;
use crate::prelude::*;

pub struct DigTunnelPlugin;

impl Plugin for DigTunnelPlugin{
	fn build(&self, app: &mut App) {
		app.add_system( dig.in_base_set(CoreSet::Update));
	}
}
//send want_to_dig message:
fn dig(mut delta_query: Query<(Entity, &MakeWay, &mut TileType)>,
	   tile_storage: Query<(&TileStorage)>,
	   mut commands: Commands)
{

	delta_query.iter_mut().for_each(|(e, make_way, mut tile_type)|{
		*tile_type = TileType::Tunnel;
		commands.entity(e).remove::<MakeWay>();
		}
	);
}

//Todo: this should be updated before player enters the undug tile
fn dig_after_player_relocated(mut removed: RemovedComponents<Player>, mut query: Query<(&mut TileType)>){
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
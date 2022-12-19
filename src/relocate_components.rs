use std::any::{Any, TypeId};
use std::iter::repeat;
use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::prelude::*;
use crate::tile_map::TileType;
use crate::{tile_map::SpriteIndex, DataTransfer};
use crate::plugins::animation_state::RockfordAnimationState;
use crate::plugins::sprite_animation::SpriteAnimationPlayer;

pub struct RelocateComponentsPlugin;

impl Plugin for RelocateComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, relocate_components.label("relocate"));
    }
}

fn relocate_components(
    mut query: Query<(Entity, &DataTransfer, &SpriteAnimationPlayer, &TileType, &mut TileTextureIndex, &RockfordAnimationState)>,
    mut commands: Commands,
) {
    //what to add after it was removed (todo later do this on layers)
    for (entity, mut data, mut anim, tile, mut tile_index, anim_state) in query.iter_mut() {
        //clear tile (todo maybe should be done in a separate system?):
        *tile_index = TileTextureIndex(tile.get_sprite_index());
        commands.entity(data.to).insert(anim.clone()).insert(anim_state.clone());//todo can be cloned? or store its data in resources somewhere???
        commands
            .entity(entity)
            .remove::<DataTransfer>()
            .remove::<SpriteAnimationPlayer>()
            .remove::<RockfordAnimationState>();
    }
}

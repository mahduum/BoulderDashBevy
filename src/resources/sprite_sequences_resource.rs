use std::collections::HashMap;
use bevy::prelude::Name;
use crate::{FromWorld, World, Resource};

#[derive(Resource)]
pub struct SpriteAnimationSequences
{
	pub sequences: HashMap<Name, Vec<u32>>//todo make a new type from sequence and add duration
}

impl FromWorld for SpriteAnimationSequences{
	fn from_world(world: &mut World) -> Self {
		//with asset loader it can be set elsewhere
		//let mut x = world.get_resource_mut::<MyOtherResource>().unwrap();
		let sequences = HashMap::from(
			[
				(
					Name::new("RockfordStanding"),
					vec![0, 1, 2, 3, 4, 5, 6]
				),
				(
					Name::new("DiamondShining"),
					vec![40, 50, 60, 70, 41, 51, 61, 71]
				)
			]);

		SpriteAnimationSequences { sequences }
	}
}
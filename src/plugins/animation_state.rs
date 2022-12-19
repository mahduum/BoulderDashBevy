use bevy::prelude::*;
use crate::components::{Delta};
use crate::plugins::sprite_animation::SpriteAnimationPlayer;
use crate::{PreviousDelta};

#[derive(Default)]
pub struct AnimationStatePlugin;

impl Plugin for AnimationStatePlugin{
	fn build(&self, app: &mut App) {
		app.add_system_to_stage(CoreStage::Update, update_animation_state);
	}
}

//todo maybe each entity could have animation state? and some be marked as active etc.
fn update_animation_state(
	mut query: Query<(Entity, &Delta, &mut RockfordAnimationState, &mut SpriteAnimationPlayer, Option<&PreviousDelta>)>
){
	for (entity, delta, mut animation_state, mut animation_player, previous_delta) in query.iter_mut(){
		if let Some(new_state) = update_motion_state_option( &mut animation_state, delta, previous_delta){
			let sequence_name = match new_state{
				RockfordAnimationState::Idle(_) => Name::new("RockfordStanding"),
				RockfordAnimationState::MovingRight(_) => Name::new("RockfordMovingRight"),
				RockfordAnimationState::MovingLeft(_) => Name::new("RockfordMovingLeft"),
			};

			*animation_state = new_state;

			animation_player.play(sequence_name);
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum RockfordAnimationState {
	Idle(Delta),
	MovingLeft(Delta),
	MovingRight(Delta),
}

//todo more complicated for fun, but can be made much simpler, take out delta from enum
fn update_motion_state_option(last_state: &mut RockfordAnimationState, delta: &Delta, previous_delta: Option<&PreviousDelta>) -> Option<RockfordAnimationState>
{
	if let Some(prev_delta) = previous_delta
	{
		if prev_delta.0 == *delta {
			return None;
		}
	};

	if delta.x == 0 && delta.y == 0 {
		return match last_state {
			RockfordAnimationState::Idle(_)  => None, //if last state was idle do nothing
			RockfordAnimationState::MovingRight(delta) | RockfordAnimationState::MovingLeft(delta) =>
			{
				Some(RockfordAnimationState::Idle(*delta)) //if last state was not idle but delta was 0 make it idle and remember last direction
			}
		}
	} else {
		let delta_x = if delta.x != 0 {delta.x}
		else {
			match last_state {
				RockfordAnimationState::MovingRight(d) | RockfordAnimationState::MovingLeft(d) | RockfordAnimationState::Idle(d) =>
					{ let x = d.x;
						x
					}
			}
		};

		return match delta_x {
			1 => Some(RockfordAnimationState::MovingRight(Delta::from_tuple((1, 0)))),
			_=> Some(RockfordAnimationState::MovingLeft(Delta::from_tuple((-1, 0)))),
			}
		};
	None
}

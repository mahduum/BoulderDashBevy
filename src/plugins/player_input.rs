use crate::{
    tile_map::TileCollider,
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_tilemap::prelude::TilePos;
use std::borrow::Borrow;
use std::io::stderr;
use std::mem::replace;
//use bevy_inspector_egui::Inspectable;
use crate::prelude::*;

pub struct PlayerInputPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RockfordMotionState {
    Idle {
        last_direction: Box<RockfordMotionState>,
    },
    MovingLeft,
    MovingRight,
}

impl RockfordMotionState {
    pub fn update_motion_state(&mut self, delta: &Delta) {
        if delta.x == 0 && delta.y == 0 {
            match self {
                RockfordMotionState::Idle { last_direction } => {} //if last state was idle do nothing
                _ => {
                    *self = RockfordMotionState::Idle {
                        last_direction: Box::new(self.clone()),
                    } //if last state was not idle but delta was 0 make it idle and remember last direction
                }
            }
        } else if delta.x != 0 {
            *self = match delta.x {
                1 => RockfordMotionState::MovingRight,
                _ => RockfordMotionState::MovingLeft,
            }
        } else if let RockfordMotionState::Idle { last_direction } = self {
            //retrieve last direction to position sprite as it was before when moving vertically, steal it from the box
            *self = *last_direction.to_owned();
        }
        //otherwise we are ok
    }

    pub fn update_motion_state_option(&self, delta: &Delta) -> Option<RockfordMotionState> {
        if delta.x == 0 && delta.y == 0 {
            return match self {
                RockfordMotionState::Idle { last_direction } => None, //if last state was idle do nothing
                _ => {
                    Some(RockfordMotionState::Idle {
                        last_direction: Box::new(self.clone()),
                    }) //if last state was not idle but delta was 0 make it idle and remember last direction
                }
            }
        } else if delta.x != 0 {
            return match delta.x {
                1 => Some(RockfordMotionState::MovingRight),
                _ => Some(RockfordMotionState::MovingLeft),
            }
        } else if let RockfordMotionState::Idle { last_direction } = self {
            //retrieve last direction to position sprite as it was before when moving vertically, steal it from the box
            return Some(*last_direction.to_owned());
        }

        None
    }

    pub fn get_first_frame_index(&self) -> u32 {
        match self {
            RockfordMotionState::Idle { .. } => 0,
            RockfordMotionState::MovingLeft => 10,
            RockfordMotionState::MovingRight => 20,
        }
    }
}

impl Plugin for PlayerInputPlugin {
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
    mut motion_state: ResMut<State<RockfordMotionState>>,
    mut commands: Commands,
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

    // let mut state = motion_state.current().clone();
    // state.update_motion_state(&delta);
    // if *motion_state.current() != state && motion_state.overwrite_replace(state).is_ok() == false{
    // 	eprintln!("Set state unsuccessful!");
    // 	return;
    // }

    // motion_state.pop();

    if let Some(new_state) = motion_state.current().update_motion_state_option(&delta) {
        if let Err(error) = motion_state.overwrite_replace(new_state) {
            eprintln!("Overwrite state error: {}", error);
        }
        //todo as long as the same input is being received make a delay to first clean up/dig the dirt tile and only after that move on its place
        //first check if the tunnel can be dug, then when it is dug out, allow to move on its position
        println!("Set state successful!");
    }

    if delta == Delta::zero() {
        return;
    }

    match player_query.get_single_mut() {
        Ok(entity) => {
            commands.entity(entity).insert(delta);
        }
        Err(msg) => eprintln!("Couldn't find player entity: {}", msg),
    }
}

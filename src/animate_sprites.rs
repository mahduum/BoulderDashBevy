use std::time::Duration;

use bevy::prelude::*;
//use bevy_ecs_tilemap::tiles::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::plugins::player_input::RockfordMotionState;
use crate::prelude::*;
use crate::tile_map::TileType;
use crate::{components::SpriteIndexRuntime, tile_map::SpriteIndex, DataTransfer};
use crate::plugins::sprite_animation::SpriteAnimationPlayer;

pub struct AnimateSpritesPlugin;

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
//todo can be change to simple (pub fn(usize) -> usize)
pub struct Animatable
//receives as arg animatable struct or trait or func, so we know what to use
{
    pub current_index: u32,
    pub sprite_index_provider: fn(u32) -> u32,
}

/// Non generic trait for simple linear animations
impl Animatable {
    pub fn next_index(&mut self) -> u32 {
        let next_index = (self.sprite_index_provider)(self.current_index);
        self.current_index = next_index;
        self.current_index
    }
}

#[derive(Component)] //todo can it simply extend normal Animatable? (to reuse the function
pub struct AnimatableGeneric {
    pub current_index: u32,
    pub current_state: RockfordMotionState,
    pub sprite_index_provider: Box<dyn SpriteIndexRuntime + Send + Sync>,
}

/// Provides a way for implementing more complex custom animations for dissimilar types
impl AnimatableGeneric {
    pub fn get_index(&mut self) -> u32 {
        // if state changed then detect the change and set current index to invalid? so get sprite index could now that is must start "fresh"
        // state or AnimatableGeneric must have logic to provide first index and to detect state change if there is a state in the option
        // TODO: update current index (todo should be done in a separate system, each animatable generic that can accept state as arg, like AnimatableGeneric<RockfordMotionState>)
        // for now short cut in update state:
        let next_index =
            (*self.sprite_index_provider).get_sprite_index(self.current_index, &self.current_state); //current index is already updated but we need to pass state as well
        self.current_index = next_index;
        self.current_index
    }

    //todo UNUSED!
    pub fn update_state(&mut self, state: RockfordMotionState) {
        if self.current_state != state {
            self.current_state = state.clone();
            self.current_index = state.get_first_frame_index();
        }
    }
}

impl Clone for AnimatableGeneric {
    fn clone(&self) -> Self {
        AnimatableGeneric {
            current_index: self.current_index,
            current_state: self.current_state.clone(),
            sprite_index_provider: dyn_clone::clone_box(&*self.sprite_index_provider), //todo try in the future substitute with Rc<Cell>
        }
    }
}

impl Plugin for AnimateSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, relocate_components.label("relocate"))
            .add_system(animate_sprites);
    }
}

//this can be split to different animatable components, but the component can get the way (function delegate) to provide the right index
fn animate_sprites(
    time: Res<Time>,
    rockford_motion_state: Res<State<RockfordMotionState>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    //making set of queries because they access simultaneously common components and can have mutability conflicts over the same data
    //optionally both animation components could be wrapped in option and then check whichever is present
    mut set: ParamSet<(
        Query<(
            &mut AnimationTimer,
            &mut Animatable,
            &mut TileTextureIndex, //tile texture to have its index changed
        )>,
        Query<(
            &mut AnimationTimer,
            &mut AnimatableGeneric,
            &mut TileTextureIndex,
        )>,
    )>,
    mut static_tiles_query: Query<
        (&mut TileTextureIndex, &TileType),
        (Without<AnimatableGeneric>, Without<Animatable>),
    >,
) {
    for (mut timer, mut animatable, mut tile_texture) in set.p0().iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            *tile_texture = TileTextureIndex(animatable.next_index());
        }
    }

    for (mut timer, mut animatable_generic, mut tile_texture) in set.p1().iter_mut() {
        animatable_generic.update_state(rockford_motion_state.current().clone());
        timer.tick(time.delta());
        if timer.just_finished() {
            let next_index = animatable_generic.get_index();
            *tile_texture = TileTextureIndex(next_index);
        }
    }

    //todo is it necessary to continuously animate the same image on every static tile?
    static_tiles_query
        .iter_mut()
        .for_each(|(mut tile_tex, tile_type)| {
            *tile_tex = TileTextureIndex(tile_type.get_sprite_index());
        })
}

//todo figure out
fn relocate_components(
    mut query: Query<(Entity, &DataTransfer, &SpriteAnimationPlayer, &mut TileType)>,
    mut commands: Commands,
) {
    //what to add after it was removed (todo later do this on layers)
    for (entity, mut data, mut anim, mut tile) in query.iter_mut() {
        //clone animatable generic with its data:
        commands.entity(data.to).insert(anim.clone());//todo can be cloned? or store its data in resources somewhere???
        commands
            .entity(entity)
            .remove::<DataTransfer>()
            .remove::<SpriteAnimationPlayer>();
    }
}

#[derive(Component)]
struct PauseAnimation(bool);

#[allow(dead_code)]
pub fn get_index_rockford_standing(current_index: u32, mut timer: Mut<AnimationTimer>) -> u32 {
    let next_index = (current_index + 1) % 7;
    if next_index == 0 && timer.duration().as_secs_f32() < 1.0 {
        timer.set_duration(Duration::from_secs(3));
    } else if timer.duration().as_secs_f32() >= 1.0 {
        timer.set_duration(Duration::from_millis(150))
    }

    next_index
}

#[allow(dead_code)]
pub fn get_index_rockford_walk_left(current_index: u32) -> u32 {
    (current_index + 1) % 7 + 10
}

#[allow(dead_code)]
pub fn get_index_rockford_walk_right(current_index: usize) -> usize {
    (current_index + 1) % 7 + 20
}

// fn get_index_for_butterfly(current_index: usize) -> usize
// {
//     let start_offset = 4 * 10 + 6; //46
//     get_index_vertical_animation(current_index, start_offset)
// }

// fn get_index_for_ghost(current_index: usize) -> usize
// {
//     let start_offset = 4 * 10 + 5; //45
//     get_index_vertical_animation(current_index, start_offset)
// }

pub fn get_index_for_diamond(current_index: u32) -> u32 {
    let start_offset = 4 * 10;
    let next_index = get_index_vertical_animation(current_index, start_offset);
    if next_index % 2 == 0 {
        if (next_index - start_offset) == 0 {
            return next_index + 1;
        }
    } else {
        if (next_index - start_offset - 1) == 0 {
            return next_index - 1;
        }
    }

    println!("diamond index: {}", next_index);
    next_index
}

fn get_index_vertical_animation(current_index: u32, start_offset: u32) -> u32 {
    if current_index <= start_offset {
        return start_offset;
    }

    ((current_index - start_offset + 10) % 40) + start_offset
}

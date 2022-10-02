use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_inspector_egui::Inspectable;

use crate::animate_sprites::AnimationTimer;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    pub active: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            speed: 3.0,
            active: true,
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Diamond{

}

pub trait SpriteIndexRuntime {
    fn get_sprite_index(&mut self, current_index: u32) -> u32;
}

#[derive(Component)]
pub struct RockfordAnimation {
    //todo this should be a component in order to access timer ecs way, it cannot have a direct reference, this is for testing only, but maybe it could be simply made a component?
    pub timer: AnimationTimer, //todo this particular animation can also have its own timer outside in resources which it could access and modify but in a less readable way,
                               // it would be a separate Query<(&mut RockfordAnimation, &mut RockfordAnimationTimer), (With<Player>)>
}

/// Internal custom index getters for animations of Rockford. Can be rendered more complex and reactive to Rockford's stated by adding more functions, or accepting more arguments like state.
impl<'a> RockfordAnimation {
    pub fn get_index_rockford_standing(&'a mut self, current_index: u32) -> u32 {
        let next_index = (current_index + 1) % 7;
        if next_index == 0 && self.timer.duration().as_secs_f32() < 1.0 {
            self.timer.set_duration(Duration::from_secs(3));
        } else if self.timer.duration().as_secs_f32() >= 1.0 {
            self.timer.set_duration(Duration::from_millis(150))
        }

        next_index
    }
}

//'a : 'b means "a outlives b"
/// Implementation of runtime animation index that allows using internal Rockford animation functions in a generic context and in a way that details are hidden.
impl SpriteIndexRuntime for RockfordAnimation {
    fn get_sprite_index(&mut self, current_index: u32) -> u32 {
        self.get_index_rockford_standing(current_index)
    }
}

//whatever has this component is intended to move (instead of wants to move try using only this one)
#[derive(Component, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Delta {
    pub x: i32,
    pub y: i32,
}

impl Delta {
    /// Create a new point from an x/y coordinate.
    #[inline]
    #[must_use]
    pub fn new<T>(x: T, y: T) -> Delta
    where
        T: TryInto<i32>,
    {
        Delta {
            x: x.try_into().ok().unwrap_or(0),
            y: y.try_into().ok().unwrap_or(0),
        }
    }

    /// Create a new point from i32, this can be constant
    pub const fn constant(x: i32, y: i32) -> Self {
        Delta { x, y }
    }

    // Create a zero point
    #[inline]
    pub fn zero() -> Self {
        Delta { x: 0, y: 0 }
    }

    #[inline]
    // Create a point from a tuple of two i32s
    pub fn from_tuple<T>(t: (T, T)) -> Self
    where
        T: TryInto<i32>,
    {
        Delta::new(t.0, t.1)
    }

    #[inline]
    /// Helper for map index conversion
    pub fn to_index<T>(self, width: T) -> usize
    where
        T: TryInto<usize>,
    {
        let x: usize = self.x.try_into().ok().unwrap();
        let y: usize = self.y.try_into().ok().unwrap();
        let w: usize = width.try_into().ok().unwrap();
        (y * w) + x
    }

    /// Converts the point to an i32 tuple
    pub fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Converts the point to a usize tuple
    pub fn to_unsigned_tuple(self) -> (usize, usize) {
        (
            self.x.try_into().ok().unwrap(),
            self.y.try_into().ok().unwrap(),
        )
    }

    /// Converts the point to an UltraViolet vec2
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    /*
    // This doesn't seem to exist anymore?
    /// Converts the point to an UltraViolet vec2i
    pub fn to_vec2i(self) -> Vec2i {
        Vec2i::new(self.x, self.y)
    }
    */

    /// Creates a point from an UltraViolet vec2
    pub fn from_vec2(v: Vec2) -> Self {
        Self::new(v.x as i32, v.y as i32)
    }

    /*
    /// Creates a point from an UltraViolet vec2i
    pub fn from_vec2i(v: Vec2i) -> Self {
        Self::new(v.x, v.y)
    }
    */
}

impl From<(i32, i32)> for Delta {
    fn from(item: (i32, i32)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl From<(f32, f32)> for Delta {
    fn from(item: (f32, f32)) -> Self {
        Self {
            x: item.0 as i32,
            y: item.1 as i32,
        }
    }
}

impl From<Vec2> for Delta {
    fn from(item: Vec2) -> Self {
        Self {
            x: item.x as i32,
            y: item.y as i32,
        }
    }
}

#[derive(Component)]
pub struct WantsToMove {
    entity: Entity,
    destination: Delta,
}

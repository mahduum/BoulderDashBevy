use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use std::{marker::PhantomData, time::Duration};
//use bevy_inspector_egui::Inspectable;
use crate::plugins::player_input::*;
use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
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

#[derive(Component)]
pub struct Diamond {}

#[derive(Component)]
pub struct DataTransfer {
    pub to: Entity,
}

impl DataTransfer {
    // for transferring the player with its data onto another tile
    pub fn move_to(to: Entity) -> Self {
        DataTransfer { to }
    }
}

//todo make this a file with parsed sequence:
#[derive(Clone, Debug)]
pub struct SpriteAnimationSequence<'a>{
    pub name: Name,
    pub sequence: &'a Vec<u32>,//todo just to play with reference but change it to clone
    pub duration: u32
}

impl<'a> SpriteAnimationSequence<'a>{
    pub fn new(name: Name, sequence: &'a Vec<u32>) -> Self {
        SpriteAnimationSequence
        {
            name,
            sequence,
            duration: (*sequence).len() as u32
        }
    }
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct MakeWay;

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

#[derive(Component, Deref, DerefMut, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct PreviousDelta(pub Delta);

#[derive(Component)]
pub struct WantsToMove {
    entity: Entity,
    destination: Delta,
}

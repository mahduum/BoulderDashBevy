use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use crate::{
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
    //tilemap::TileCollider,
    TILE_SIZE, tile_map::TileCollider, TILE_SCALE,
};

pub struct DiamondPlugin;

#[derive(Component, Inspectable)]
pub struct Diamond{

}

//implement animatable trait for all animatable objects, make system that animates everything and components implement get index etc.
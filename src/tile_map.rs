use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;
use bevy_ecs_tilemap::TileBundleTrait;
use crate::prelude::*;

use crate::{
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
    TILE_SIZE,
};

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
    Entrance,
}

#[derive(Copy, Clone, PartialEq)]
pub struct BDTile;
//TODO how to build a map with tiles???

// impl TileBundleTrait for BDTile
// {
//     fn get_tile_pos_mut(&mut self) -> &mut bevy_ecs_tilemap::TilePos {
//         todo!()
//     }

//     fn get_tile_parent(&mut self) -> &mut bevy_ecs_tilemap::TileParent {
//         todo!()
//     }
// }

// impl Bundle for BDTile{

//     fn component_ids(components: &mut bevy::ecs::component::Components, storages: &mut bevy::ecs::storage::Storages) -> Vec<bevy::ecs::component::ComponentId> {
//         todo!()
//     }

//     unsafe fn from_components(func: impl FnMut() -> *mut u8) -> Self
//     where
//         Self: Sized {
//         todo!()
//     }

//     fn get_components(self, func: impl FnMut(*mut u8)) {
//         todo!()
//     }
// }


fn create_simple_map(mut commands: Commands, sheet: Res<TileSheet>) {
    let file = File::open("assets/maps/start_map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {//y is line row here
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_sprite_from_tile_sheet(
                    &mut commands,
                    &sheet,
                    match char {
                        '#' => 32,
                        '.' => 33,
                        _ => 36,
                    },
                    Default::default(),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())//propagate transforms down in respect to the parent
        .push_children(&tiles);
}
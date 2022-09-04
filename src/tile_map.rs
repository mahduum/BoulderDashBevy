use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};
use rand::{thread_rng, Rng};

use bevy::{prelude::*, reflect::GetTypeRegistration};
use bevy::reflect;
use bevy::render::texture::ImageSettings;

use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::prelude::*;

use crate::{prelude::*, tile_sheet::sprite_sheet_bundle, RESOLUTION};

use crate::{
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
    TILE_SIZE,
};

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(pre_startup);
        //app.add_startup_system(create_simple_map);
        //app.add_startup_system_to_stage(StartupStage::PostStartup, create_simple_map);
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
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 800.0),
                );

                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }

                tiles.push(tile);
            }
        }
    }

    commands
        .spawn_bundle(SpatialBundle::default())
        .push_children(&tiles);
}

fn get_texture_atlas_indices() -> HashMap<(u32,u32), u32>{
    let file = File::open("assets/maps/start_map.txt").expect("No map file found");
    let mut tile_sprite_indices = HashMap::<(u32, u32), u32>::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {//y is line row here
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate()
            {
                tile_sprite_indices.insert((y as u32, x as u32), match char {
                    '#' => 32,
                    '.' => 33,
                    _ => 36,
                });
            }
        }
    }

    tile_sprite_indices
}

//or make this prestartup and then set tile indexes??? by positions
fn pre_startup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let texture_handle: Handle<Image> = asset_server.load("textures/chars/boulder_dash.png");//we already have tiles... use texture atlas instead of image?

    let tilemap_size = TilemapSize { x: 18, y: 8 };
    let mut tile_storage = TileStorage::empty(tilemap_size);
    // Create map entity and component:
    let tilemap_entity = commands.spawn().id();//empty entity
    let tile_sprite_indices = get_texture_atlas_indices();

    //todo how to read file buffer with indexes???
    for x in 0..18u32 {
        for y in 0..8u32 {
            let tile_pos = TilePos { x, y };
            let key = (7 - y, x);
            let index = if let Some(value) = tile_sprite_indices.get(&key){
                *value
            }else{
                0
            };
            let tile_texture = TileTexture(index);//get the tile texture index from file buffer
            //let tile_texture = TileTexture(45);//get the tile texture index from file buffer
            let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        texture: tile_texture,
                        tilemap_id: TilemapId(tilemap_entity),
                        visible: TileVisible(true),
                        ..Default::default()
                    })
                    //.insert(LastUpdate::default())
                    .id();

            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    //let tile_map_texture = TilemapTexture(texture_handle);

    commands
            .entity(tilemap_entity)
            .insert_bundle(TilemapBundle {
                grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
                size: tilemap_size,
                storage: tile_storage,
                texture: TilemapTexture(texture_handle),
                tile_size,
                transform: //get_centered_transform_2d(&tilemap_size, &tile_size, 100.0),//.with_scale(Vec3 { x: 0.1, y: 0.1, z: 0.1 }),
                Transform::from_xyz(
                    0.1,
                    0.1,
                    899.0,
                ).with_scale(Vec3 { x: 0.01, y: 0.01, z: 1.0 }),//get_centered_transform_2d(&tilemap_size, &tile_size, 0.0),*/
                ..Default::default()
            });
}

fn random_index(seed: u32) -> u32 {
    if seed % 2u32 == 0 {
        return 0u32;
    }
    else {
        let mut rng = rand::thread_rng();
        let num: f64 = rng.gen();
        ((num * 10.0) as u32) % 5
    }
}

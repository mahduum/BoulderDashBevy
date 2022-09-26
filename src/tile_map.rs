use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap, sync::Arc,
};
use bevy_inspector_egui::Inspectable;
use rand::{thread_rng, Rng};

use bevy::{prelude::*, reflect::GetTypeRegistration};
use bevy::reflect;
use bevy::render::texture::ImageSettings;

use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::prelude::*;

use crate::{prelude::*, tile_sheet::sprite_sheet_bundle, RESOLUTION, TILE_SCALE, TILE_SIZE_SCALED, components::{SpriteIndexRuntime, RockfordAnimation}};

use crate::{
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
    TILE_SIZE, player, diamond, animate_sprites
};

mod test_module;
//use player;

//todo how to move from tile to tile? gradualy or in an instance
#[derive(Component, Inspectable)]
pub struct Tile{
  pub spriteAtlasIndex: u32,
}

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup.label("player_spawn"));
        //app.add_startup_system(create_simple_map);
        //app.add_startup_system_to_stage(StartupStage::PostStartup, create_simple_map);
    }
}

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

//for now make tiles sprites, then make dynamic non tiles for fluent animation.
//on movement first check for hits agains dynamic entitites, then check for static tile type
//maybe only the tile bundle could be moved, transform changed from source tile to destination tile and reassinged to new tile?
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Dirt,
    Tunnel,
    Boulder,
    Diamond,
    Exit,
    Entrance,
    Player,
}

impl SpriteIndex for TileType{
    fn get_sprite_index (&self) -> u32{
        match self {
            TileType::Wall => 32,
            TileType::Dirt => 33,
            TileType::Tunnel => 36,
            TileType::Player => 0,
            TileType::Diamond => 40,
            _ => 33,
        }
    }
}

pub trait SpriteIndex {
    fn get_sprite_index(&self) -> u32;
}

//static tile, dynamic tile


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
                        '_' => 36,
                        _ => 36,
                    },
                    Default::default(),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE * TILE_SCALE, 800.0),
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

fn get_texture_atlas_indices() -> HashMap<(u32,u32), TileType>{
    let file = File::open("assets/maps/start_map.txt").expect("No map file found");
    let mut tile_sprite_indices = HashMap::<(u32, u32), TileType>::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {//y is line row here
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate()
            {
                tile_sprite_indices.insert((y as u32, x as u32), match char {
                    '#' => TileType::Wall,
                    '.' => TileType::Dirt,
                    '_' => TileType::Tunnel,
                    'R' => TileType::Player,//Do it other way, separate dynamic from static?//spawn on a different layer???
                    '*' => TileType::Diamond,
                    _ => TileType::Dirt,
                });
            }
        }
    }

    tile_sprite_indices
}

//or make this prestartup and then set tile indexes??? by positions
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let texture_handle: Handle<Image> = asset_server.load("textures/chars/boulder_dash.png");//we already have tiles... use texture atlas instead of image?

    let tilemap_size = TilemapSize { x: 18, y: 8 };
    let mut tile_storage = TileStorage::empty(tilemap_size);
    // Create map entity and component:
    let tilemap_entity = commands.spawn().id();//empty entity
    let tile_sprite_indices = get_texture_atlas_indices();

    //todo keep a map of tiles as resource -> what kind of tile is on which position -> this is already taken care of by tiles plugin? use example from game of life
    for x in 0..18u32 {
        for y in 0..8u32 {
            let tile_pos = TilePos { x, y };
            let key = (7 - y, x);
            let index: TileType;
            if let Some(value) = tile_sprite_indices.get(&key){
                index = *value
            }
            else{
                continue;
            };

            let tile_texture = TileTexture(index.get_sprite_index());//get the tile texture index from file buffer
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
                    //.insert(LastUpdate::default())//todo insert different components to tiles
                    .id();

            if index == TileType::Player {
                commands
                    .entity(tile_entity)
                    .insert(Name::new("Player"))
                    .insert(player::Player::new())
                    //.insert(AnimatedTile{start: 0, end: 7, speed: 0.7})
                    .insert(animate_sprites::AnimationTimer(Timer::from_seconds(0.1, true)))
                    .insert(animate_sprites::AnimatableGeneric{
                        current_index: tile_texture.0,
                        sprite_index_provider: Box::new(
                            RockfordAnimation{timer: animate_sprites::AnimationTimer(Timer::from_seconds(0.1, true))
                        })
                    });
            }
            else if index == TileType::Diamond {
                commands.entity(tile_entity)
                .insert(Name::new("Diamond"))
                .insert(diamond::Diamond{})
                // .insert(Transform {
                .insert(animate_sprites::AnimationTimer(Timer::from_seconds(0.1, true)))
                .insert(animate_sprites::Animatable{
                    current_index: tile_texture.0,
                    sprite_index_provider: animate_sprites::get_index_for_diamond
                });
            }

            commands.entity(tile_entity)
                    .insert(Transform {
                        //transform has to be overwritten later what the position can be known  (or transform can be added later)
                        //transform of a tile can be calculated by the offset from the map center
                        translation: Vec3::new(x as f32 * TILE_SIZE_SCALED, y as f32 * TILE_SIZE_SCALED, 900.0),
                        ..Default::default()});

            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    //let tile_map_texture = TilemapTexture(texture_handle);

    // todo
    // let transform = Transform::from_translation(Vec3::new(
    //     map_pos_x * 18 as f32 * 16.0,
    //     map_pos_y as f32 * 8 as f32 * 16.0,
    //     0.0,
    // ));
    
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
                    0.0,
                    0.0,
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

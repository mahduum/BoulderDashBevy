use crate::{
    tile_map::TileCollider,
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
    MyLabel, TILE_SCALE, TILE_SIZE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
//use bevy_inspector_egui::Inspectable;
use crate::prelude::*;

pub struct CameraFollowPlugin;

//todo depending on state will calculate different index

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_follow.in_base_set(CoreSet::Last));
    }
}

//todo make separate system for camera movement (maybe just rename it and move player struct into components.rs)
fn camera_follow(
    player_query: Query<(Entity, &Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    //todo is it a different transform from tiles? calculate transform helper?
    //TODO some error when changing direction in the same axis
    let (e, player_transform, player) = player_query.single();

    for (mut transform) in camera_query.iter_mut(){
        transform.translation.x = player_transform.translation.x;
        transform.translation.y = player_transform.translation.y;
    }
}

//old code:
// fn player_movement(
//     mut player_query: Query<(&Player, &mut Transform, &Name)>,
//     wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>, //with constraint on the collider because we are not using data on it
//     keyboard: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     match player_query.get_single_mut() {
//         Ok((player, mut transform, _name)) => {
//             let mut y_delta = 0.0;
//             if keyboard.pressed(KeyCode::W) {
//                 y_delta += player.speed * TILE_SIZE * TILE_SCALE * time.delta_seconds();
//             }
//             if keyboard.pressed(KeyCode::S) {
//                 y_delta -= player.speed * TILE_SIZE * TILE_SCALE * time.delta_seconds();
//             }

//             let mut x_delta = 0.0;
//             if keyboard.pressed(KeyCode::A) {
//                 x_delta -= player.speed * TILE_SIZE * TILE_SCALE * time.delta_seconds();
//             }
//             if keyboard.pressed(KeyCode::D) {
//                 x_delta += player.speed * TILE_SIZE * TILE_SCALE * time.delta_seconds();
//             }

//             let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0); //how does this work as sliding along the wall?
//             if wall_collision_check(target, &wall_query) {
//                 transform.translation = target;
//             }

//             let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
//             if wall_collision_check(target, &wall_query) {
//                 transform.translation = target;
//             }
//         }
//         Err(message) => println!("Error: {}", message),
//     }
// }

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * TILE_SCALE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE * TILE_SCALE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

//run once after the map is generated and spawn player at chosen location
// fn spawn_player(mut commands: Commands, sheet: Res<TileSheet>) {
//     let player = spawn_sprite_from_tile_sheet(
//         &mut commands,
//         &sheet,
//         0,
//         Default::default(),
//         Vec3::new(
//             2.0 * TILE_SIZE * TILE_SCALE,
//             -2.0 * TILE_SIZE * TILE_SCALE,
//             900.0,
//         ),
//     );

//     let player = commands
//         .entity(player)
//         .insert(Name::new("Player"))
//         //.insert(Player { speed: 3.0 })
//         .id();

    // let background = spawn_sprite_from_tile_sheet(
    //     &mut commands,
    //     &tiles,
    //     0,
    //     Color::rgb(0.5, 0.5, 0.5),
    //     Vec3::new(0.0, 0.0, -1.0),
    // );

    // let background = commands
    //     .entity(background)
    //     .insert(Name::new("Background"))
    //     .id(); //id() gives back the entity after creation

//     commands.entity(player); //.push_children(&[background]);
// }

// #[allow(dead_code)]
// fn setup_rockford(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture_handle = asset_server.load("textures/rpg/chars/rockford/boulder_dash.png");
//     let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 10, 8);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     commands
//         .spawn_bundle(
//             SpriteSheetBundle
//             {
//                 sprite: default(),
//                 texture_atlas: texture_atlas_handle,
//                 transform: Transform::from_scale(Vec3::splat(6.0)),
//                 global_transform: default(),
//                 visibility: Visibility { is_visible: false }
//             }
//         )
//         .insert(AnimationTimer(Timer::from_seconds(0.15, true)));
// }

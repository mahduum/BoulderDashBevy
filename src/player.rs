use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use crate::{
    tile_sheet::{spawn_sprite_from_tile_sheet, TileSheet},
    //tilemap::TileCollider,
    TILE_SIZE, tile_map::TileCollider,
};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player{
    speed: f32
}

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_startup_system(spawn_player)
        //.add_system(camera_follow.after("movement"))
        .add_system(player_movement.label("movement"));
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform, &Name)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,//with constraint on the collider because we are not using data on it
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
)
{
    match player_query.get_single_mut(){
        Ok((player, mut transform, _name)) => {

            let mut y_delta = 0.0;
            if keyboard.pressed(KeyCode::W) {
                y_delta += player.speed * TILE_SIZE * time.delta_seconds();
            }
            if keyboard.pressed(KeyCode::S) {
                y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
            }

            let mut x_delta = 0.0;
            if keyboard.pressed(KeyCode::A) {
                x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
            }
            if keyboard.pressed(KeyCode::D) {
                x_delta += player.speed * TILE_SIZE * time.delta_seconds();
            }

            let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);//how does this work as sliding along the wall?
            if wall_collision_check(target, &wall_query) {
                transform.translation = target;
            }

            let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
            if wall_collision_check(target, &wall_query) {
                transform.translation = target;
            }
        },
        Err(message) => println!("Error: {}", message)
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn spawn_player(mut commands: Commands, sheet: Res<TileSheet>)
{
    let player = spawn_sprite_from_tile_sheet(
        &mut commands,
        &sheet,
        0,
        Default::default(),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );

    let player = commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 })
        .id();

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

    commands.entity(player);//.push_children(&[background]);
}

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
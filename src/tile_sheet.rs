use bevy::{prelude::*, ecs::{system::Command, entity::Entities}};
use crate::TILE_SIZE;

pub struct TileSheetPlugin;
pub struct TileSheet(Handle<TextureAtlas>);

impl Plugin for TileSheetPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_tiles);
    }
}

pub fn spawn_sprite_from_tile_sheet(
    commands: &mut Commands,
    sheet: &TileSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> Entity
{
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(16.0 * 0.01));

    commands.spawn_bundle(SpriteSheetBundle
    {
        sprite,
        texture_atlas: sheet.0.clone(),//can be texture atlas handle passed directly?
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    })
    .id()
}

pub fn sprite_sheet_bundle(
    commands: &mut Commands,
    sheet: &TileSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> SpriteSheetBundle
{
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    SpriteSheetBundle
    {
        sprite,
        texture_atlas: sheet.0.clone(),//can be texture atlas handle passed directly?
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn load_tiles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
)
{
    let image: Handle<Image> = assets.load("textures/chars/boulder_dash.png");
    let texture_atlas = TextureAtlas::from_grid(image, Vec2::new(16.0, 16.0), 10, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // let atlas = TextureAtlas::from_grid_with_padding(
    //     image,
    //     Vec2::splat(16.0),
    //     10,
    //     8,
    // Vec2::splat(1.0)
    // );

    //let atlas_handle = texture_atlases.add(atlas);
    
    commands.insert_resource(TileSheet(texture_atlas_handle));
}
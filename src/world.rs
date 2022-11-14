use crate::{components::*, loading::TextureAssets, prelude::*};
use bevy_ecs_tilemap::prelude::*;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .register_type::<MyTile>()
            .add_enter_system(GameState::WorldBuilding, generate);
    }
}

pub fn generate(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn_bundle(Camera2dBundle::default());
    let tilemap_size = TilemapSize { x: 12, y: 12 };

    let tilemap_entity = commands.spawn().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);

    let mut rng = rand::thread_rng();
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let is_walkable = rng.gen_bool(0.8);
            let tile = MyTile { is_walkable };
            let texture_index = if is_walkable { 4 } else { 2 };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
                    ..Default::default()
                })
                .insert(tile)
                .insert(Name::new("Tile"))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            map_type,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_assets.tiles.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        });

    commands.insert_resource(NextState(GameState::Playing));
}

use crate::prelude::*;

use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

pub fn startup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let tilemap_size = TilemapSize { x: 512, y: 512 };

    let tilemap_entity = commands.spawn().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);

    let mut rng = rand::thread_rng();
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(rng.gen_range(0..5)),
                    ..Default::default()
                })
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
}

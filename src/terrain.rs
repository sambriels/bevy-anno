use crate::{cursor::CursorPosition, loading::TextureAssets, prelude::*};
use bevy::math::Vec4Swizzles;
use bevy_ecs_tilemap::prelude::*;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .insert_resource(Terrain::new(UVec2::splat(64), Vec2::splat(64.0)))
            .insert_resource(CurrentTile::default())
            .register_type::<MovementCost>()
            .add_enter_system(GameState::WorldBuilding, init)
            .add_system(update_current_tile.run_in_state(GameState::Playing));
    }
}

#[derive(Component, Default)]
pub struct Terrain {
    pub size: TilemapSize,
    pub tile_size: TilemapTileSize,
    pub map_type: TilemapType,
    pub grid_size: TilemapGridSize,
}

impl Terrain {
    pub fn new(size: UVec2, tile_size: Vec2) -> Self {
        let size = TilemapSize::from(size);
        Self {
            size,
            map_type: TilemapType::square(false),
            tile_size: TilemapTileSize::from(tile_size),
            grid_size: TilemapGridSize::from(tile_size),
        }
    }
    pub fn generate(&self, mut commands: Commands, texture_assets: Res<TextureAssets>) {
        let tilemap_entity = commands.spawn().id();
        let mut storage = TileStorage::empty(self.size);
        let mut rng = rand::thread_rng();
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let tile_pos = TilePos { x, y };
                let is_walkable = rng.gen_bool(0.8);
                let texture_index = if is_walkable { 4 } else { 2 };
                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(texture_index),
                        ..Default::default()
                    })
                    .insert(MovementCost(if is_walkable { 0 } else { 999 }))
                    .insert(Name::new("Tile"))
                    .id();
                storage.set(&tile_pos, tile_entity);
            }
        }

        commands
            .entity(tilemap_entity)
            .insert_bundle(TilemapBundle {
                grid_size: self.grid_size,
                map_type: self.map_type,
                size: self.size,
                storage,
                texture: TilemapTexture::Single(texture_assets.tiles.clone()),
                tile_size: self.tile_size,
                transform: get_tilemap_center_transform(
                    &self.size,
                    &self.grid_size,
                    &self.map_type,
                    0.0,
                ),
                ..Default::default()
            });

        commands.insert_resource(NextState(GameState::Playing));
    }
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct MovementCost(pub i32);

fn init(commands: Commands, terrain: Res<Terrain>, texture_assets: Res<TextureAssets>) {
    terrain.generate(commands, texture_assets);
}

#[derive(Component, Debug, Default)]
pub struct CurrentTile {
    pub hovered: Option<TilePos>,
    pub selected: Option<TilePos>,
}

pub trait TileWorldPosition {
    fn world_position(
        &self,
        grid_size: &TilemapGridSize,
        transform: &Transform,
        map_type: &TilemapType,
    ) -> Vec2;
}
impl TileWorldPosition for TilePos {
    fn world_position(
        &self,
        grid_size: &TilemapGridSize,
        transform: &Transform,
        map_type: &TilemapType,
    ) -> Vec2 {
        self.center_in_world(grid_size, map_type) + transform.translation.truncate()
    }
}
fn update_current_tile(
    cursor_position: Res<CursorPosition>,
    tilemap_query: Query<(&TilemapSize, &TilemapGridSize, &Transform, &TilemapType)>,
    mut current_tile: ResMut<CurrentTile>,
) {
    if cursor_position.is_changed() {
        let (map_size, grid_size, map_transform, tilemap_type) = tilemap_query.single();

        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 1.0
            let cursor_pos = Vec4::from((cursor_position.world, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        let current_hover =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, tilemap_type);

        match (current_tile.hovered, current_hover) {
            (Some(a), Some(b)) if a == b => {}
            _ => {
                current_tile.hovered = current_hover;
            }
        }
    }
}

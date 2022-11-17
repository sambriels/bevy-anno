use crate::{cursor::CursorPosition, loading::TextureAssets, prelude::*};
use bevy::math::Vec3Swizzles;
use bevy_ecs_tilemap::prelude::*;
use pathfinding::prelude::astar;
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
// TODO: this is not really well thought out, update this once I have more knowledge of what it should be
#[derive(Component, Default)]
pub struct Terrain {
    pub size: TilemapSize,
    pub tile_size: TilemapTileSize,
    pub map_type: TilemapType,
    pub grid_size: TilemapGridSize,
}

impl Terrain {
    pub fn new(size: UVec2, tile_size: Vec2) -> Self {
        Self {
            size: TilemapSize::from(size),
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
        let (map_size, grid_size, map_transform, map_type) = tilemap_query.single();

        let current_hover = TilePos::from_world_pos(
            &(cursor_position.world - map_transform.translation).xy(),
            map_size,
            grid_size,
            map_type,
        );

        match (current_tile.hovered, current_hover) {
            (Some(a), Some(b)) if a == b => {}
            _ => {
                current_tile.hovered = current_hover;
            }
        }
    }
}

pub trait Pathfinding {
    fn distance(&self, other: &TilePos) -> i32;
    fn successors(
        &self,
        tile_storage: &TileStorage,
        tilemap_type: &TilemapType,
        tile_query: &Query<(&MovementCost, &TilePos)>,
    ) -> Vec<(TilePos, i32)>;
    fn find_path_to(
        &self,
        to: &TilePos,
        tile_storage: &TileStorage,
        tilemap_type: &TilemapType,
        tile_query: &Query<(&MovementCost, &TilePos)>,
    ) -> Option<(Vec<TilePos>, i32)>;
}

impl Pathfinding for TilePos {
    fn distance(&self, other: &TilePos) -> i32 {
        Vec2::from(self).distance(other.into()) as i32
    }

    fn successors(
        &self,
        tile_storage: &TileStorage,
        tilemap_type: &TilemapType,
        tile_query: &Query<(&MovementCost, &TilePos)>,
    ) -> Vec<(TilePos, i32)> {
        get_tile_neighbors(self, tile_storage, tilemap_type)
            .into_iter()
            .map(|entity| {
                if let Ok((cost, tile)) = tile_query.get(entity) {
                    Some((*tile, cost.0))
                } else {
                    None
                }
            })
            .filter(|option| match option {
                Some((_, cost)) => *cost < 5,
                None => false,
            })
            .map(|option| option.unwrap())
            .collect()
    }

    fn find_path_to(
        &self,
        to: &TilePos,
        tile_storage: &TileStorage,
        tilemap_type: &TilemapType,
        tile_query: &Query<(&MovementCost, &TilePos)>,
    ) -> Option<(Vec<TilePos>, i32)> {
        astar(
            self,
            |tile| tile.successors(tile_storage, tilemap_type, tile_query),
            |p| p.distance(to),
            |p| *p == *to,
        )
    }
}

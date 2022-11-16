use crate::{prelude::*, terrain::MovementCost};
use pathfinding::prelude::astar;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app;
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
            .map(|a| {
                if let Ok((cost, tile)) = tile_query.get(a) {
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

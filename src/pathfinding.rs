use crate::{components::GameState, cursor::CursorPosition, prelude::*, terrain::MovementCost};
use bevy::math::Vec4Swizzles;
use pathfinding::prelude::astar;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(find_path.run_in_state(GameState::Playing));
    }
}

fn find_path(
    mouse_input: Res<Input<MouseButton>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TileStorage,
        &Transform,
        &TilemapType,
        &TilemapTileSize,
    )>,
    tile_query: Query<(&MovementCost, &TilePos)>,
    cursor_pos: Res<CursorPosition>,
    lines: ResMut<DebugLines>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (map_size, grid_size, tile_storage, map_transform, tilemap_type, tile_size) =
            tilemap_query.single();

        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 1.0
            let cursor_pos = Vec4::from((cursor_pos.world, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, tilemap_type)
        {
            let goal = TilePos { x: 4, y: 6 };
            let path = goal.find_path_to(&tile_pos, tile_storage, tilemap_type, &tile_query);

            if let Some((path, _)) = path {
                show_path_debug(path, grid_size, tilemap_type, map_size, tile_size, lines)
            }
        };
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

fn show_path_debug(
    points: Vec<TilePos>,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    mut lines: ResMut<DebugLines>,
) {
    let half_grid_offset = Vec2::new(
        map_size.x as f32 * tile_size.x / 2.0,
        map_size.y as f32 * tile_size.y / 2.0,
    );
    for (i, pos) in points.iter().enumerate() {
        if i > 0 {
            let start = points[i - 1].center_in_world(grid_size, map_type) - half_grid_offset;
            let end = pos.center_in_world(grid_size, map_type) - half_grid_offset;
            lines.line_colored(
                Vec3::new(start.x, start.y, 1.1),
                Vec3::new(end.x, end.y, 1.1),
                20.0,
                Color::RED,
            );
        }
    }
}

use crate::{
    components::{CursorPos, GameState},
    prelude::*,
    utils::mouse_pos_in_map,
    world::MovementCost,
};
use pathfinding::prelude::astar;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(find_path.run_in_state(GameState::Playing));
    }
}

pub fn find_path(
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
    cursor_pos: Res<CursorPos>,
    lines: ResMut<DebugLines>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (map_size, grid_size, tile_storage, map_transform, tilemap_type, tile_size) =
            tilemap_query.single();

        let mouse_pos = mouse_pos_in_map(cursor_pos, map_transform);
        if let Some(tile_pos) =
            TilePos::from_world_pos(&mouse_pos, map_size, grid_size, tilemap_type)
        {
            let goal = TilePos { x: 4, y: 6 };
            let result =
                find_path_in_tilemap(&tile_pos, &goal, tile_storage, tilemap_type, tile_query);

            if let Some((path, _)) = result {
                show_path_debug(path, grid_size, tilemap_type, map_size, tile_size, lines)
            }
        };
    }
}
// TODO: Impl successor on TilePos if possible, otherwise impl seccessor on `MyTilePos`?
pub fn find_path_in_tilemap(
    from: &TilePos,
    to: &TilePos,
    tile_storage: &TileStorage,
    tilemap_type: &TilemapType,
    tile_query: Query<(&MovementCost, &TilePos)>,
) -> Option<(Vec<TilePos>, i32)> {
    astar(
        from,
        |tile| {
            get_tile_neighbors(tile, tile_storage, tilemap_type)
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
        },
        |p| Vec2::from(p).distance(to.into()) as i32,
        |p| p == to,
    )
}

pub fn show_path_debug(
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

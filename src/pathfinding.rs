use crate::{
    components::{CursorPos, GameState},
    prelude::*,
    utils::mouse_pos_in_map,
};
use bevy_ecs_tilemap::prelude::*;
use pathfinding::prelude::astar;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(find_path.run_in_state(GameState::Playing));
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

pub fn find_path(
    mouse_input: Res<Input<MouseButton>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &Transform,
        &TilemapType,
        &TilemapTileSize,
    )>,
    cursor_pos: Res<CursorPos>,
    lines: ResMut<DebugLines>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (map_size, grid_size, map_transform, tilemap_type, tile_size) = tilemap_query.single();

        let mouse_pos = mouse_pos_in_map(cursor_pos, map_transform);
        if let Some(tile_pos) =
            TilePos::from_world_pos(&mouse_pos, map_size, grid_size, tilemap_type)
        {
            let goal = TilePos { x: 4, y: 6 };
            let result = find_path_in_tilemap(&tile_pos, &goal, map_size, tilemap_type);

            if let Some((path, _)) = result {
                show_path_debug(path, grid_size, tilemap_type, map_size, tile_size, lines)
            }
            // if let Ok(mut tile_text) = tile_label_q.get_mut(tile_entity) {
            //     for mut section in tile_text.sections.iter_mut() {
            //         section.style.color = Color::RED;
            //     }
            //     commands.entity(tile_entity).insert(HighlightedLabel);
            // }
        };
    }
}

pub fn find_path_in_tilemap(
    from: &TilePos,
    to: &TilePos,
    map_size: &TilemapSize,
    tilemap_type: &TilemapType,
) -> Option<(Vec<TilePos>, i32)> {
    astar(
        from,
        |p| {
            get_neighboring_pos(p, map_size, tilemap_type)
                .into_iter()
                .map(|a| (a, 1))
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
            let prev_pos = points[i - 1];
            let start = TilePos::center_in_world(
                &TilePos::new(prev_pos.x, prev_pos.y),
                grid_size,
                map_type,
            ) - half_grid_offset;
            let end = TilePos::center_in_world(&TilePos::new(pos.x, pos.y), grid_size, map_type)
                - half_grid_offset;

            lines.line_colored(
                Vec3::new(start.x, start.y, 1.1),
                Vec3::new(end.x, end.y, 1.1),
                20.0,
                Color::RED,
            );
        }
    }
}

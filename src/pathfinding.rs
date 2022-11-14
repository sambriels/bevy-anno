use crate::{
    components::{CursorPos, GameState, MyTile},
    prelude::*,
};
use bevy::math::Vec4Swizzles;
use bevy_ecs_tilemap::prelude::{get_tile_neighbors, *};
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
        &TileStorage,
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform,
    )>,
    cursor_pos: Res<CursorPos>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (tile_storage, map_size, grid_size, map_type, map_transform) = tilemap_query.single();
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 1.0
            let cursor_pos = Vec4::from((cursor_pos.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                println!("Clicked on tile: {:?}, {:?}", tile_entity, tile_pos);
                if let Ok(name) = tilemap_query.get(tile_entity) {
                    println!("Tile name: {:?}", name);
                }
                static GOAL: UVec2 = UVec2::new(4, 6);
                let result = astar(
                    &UVec2::new(tile_pos.x, tile_pos.y),
                    |p| {
                        get_neighboring_pos(&TilePos::from(*p), map_size, map_type)
                            .into_iter()
                            .map(|a| (UVec2::from(a), 1))
                    },
                    |p| p.as_vec2().distance(GOAL.as_vec2()) as i32,
                    |p| *p == GOAL,
                );
                // assert_eq!(result.expect("no path found").1, 4);
                println!("Result: {:?}", result);
                // if let Ok(mut tile_text) = tile_label_q.get_mut(tile_entity) {
                //     for mut section in tile_text.sections.iter_mut() {
                //         section.style.color = Color::RED;
                //     }
                //     commands.entity(tile_entity).insert(HighlightedLabel);
                // }
            };
        }
    }
}

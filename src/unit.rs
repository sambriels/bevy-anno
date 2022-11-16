use bevy::math::Vec4Swizzles;

use crate::{
    components::GameState, cursor::CursorPosition, loading::TextureAssets,
    pathfinding::Pathfinding, prelude::*, terrain::MovementCost,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .add_system(spawn_on_click.run_in_state(GameState::Playing))
            .add_system(debug_path.run_in_state(GameState::Playing))
            .add_system(move_unit.run_in_state(GameState::Playing));
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Unit;

#[derive(Component, Default)]
pub struct TargetLocation {
    pub current_waypoint: Option<Vec2>,
    pub waypoints: Vec<Vec2>,
}

#[derive(Component, Default)]
pub struct DebugPath;

pub fn spawn_unit(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    position: Vec2,
    path: Vec<Vec2>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: texture_assets.texture_bevy.clone(),
            transform: Transform::from_xyz(position.x, position.y, 1.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Unit)
        .insert(TargetLocation {
            current_waypoint: None,
            waypoints: path,
        })
        .insert(DebugPath);
}
fn spawn_on_click(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    texture_assets: Res<TextureAssets>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TileStorage,
        &Transform,
        &TilemapType,
        &TilemapTileSize,
    )>,
    tile_query: Query<(&MovementCost, &TilePos)>,
    cursor_position: Res<CursorPosition>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (map_size, grid_size, tile_storage, map_transform, tilemap_type, tile_size) =
            tilemap_query.single();

        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 1.0
            let cursor_pos = Vec4::from((cursor_position.world, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, tilemap_type)
        {
            let goal = TilePos { x: 4, y: 6 };
            let path = goal.find_path_to(&tile_pos, tile_storage, tilemap_type, &tile_query);

            if let Some((path, _)) = path {
                let half_grid_offset = Vec2::new(
                    map_size.x as f32 * tile_size.x / 2.0,
                    map_size.y as f32 * tile_size.y / 2.0,
                );
                spawn_unit(
                    commands,
                    texture_assets,
                    tile_pos.center_in_world(grid_size, tilemap_type) - half_grid_offset,
                    path.into_iter()
                        .map(|p| p.center_in_world(grid_size, tilemap_type) - half_grid_offset)
                        .collect(),
                );
            }
        };
    }
}

fn move_unit(
    mut commands: Commands,
    time: Res<Time>,
    mut units_q: Query<(Entity, &mut Transform, &mut TargetLocation)>,
) {
    for (entity, mut transform, mut target_location) in &mut units_q {
        // TODO: Clean this up, can probably be done much more efficiently
        match target_location.current_waypoint {
            Some(current_waypoint) => {
                match current_waypoint.distance(transform.translation.truncate()) {
                    x if x < 1.0 => match target_location.waypoints.pop() {
                        Some(waypoint) => {
                            target_location.current_waypoint = Some(waypoint);
                        }
                        None => {
                            target_location.current_waypoint = None;
                        }
                    },
                    _ => {
                        let direction = (current_waypoint.extend(transform.translation.z)
                            - transform.translation)
                            .normalize();

                        transform.translation +=
                            direction.normalize() * time.delta_seconds() * 100.0;
                    }
                }
            }
            _ => match target_location.waypoints.pop() {
                Some(waypoint) => {
                    target_location.current_waypoint = Some(waypoint);
                }
                None => {
                    commands.entity(entity).remove::<TargetLocation>();
                }
            },
        }
    }
}
pub fn debug_path(mut lines: ResMut<DebugLines>, query: Query<(&TargetLocation, &DebugPath)>) {
    for (target_location, _) in query.iter() {
        for (i, waypoint) in target_location.waypoints.iter().enumerate().skip(1) {
            lines.line_colored(
                target_location.waypoints[i - 1].extend(0.0),
                waypoint.extend(0.0),
                0.0,
                Color::rgb(1.0, 0.0, 0.0),
            );
        }
    }
}

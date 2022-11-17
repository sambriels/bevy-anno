use bevy::math::Vec3Swizzles;

use crate::{
    loading::TextureAssets,
    pathfinding::Pathfinding,
    prelude::*,
    terrain::{CurrentTile, MovementCost, TileWorldPosition},
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
        .insert(TargetLocation { waypoints: path })
        .insert(DebugPath);
}
fn spawn_on_click(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    current_tile: Res<CurrentTile>,
    texture_assets: Res<TextureAssets>,
    tilemap_query: Query<(&TilemapGridSize, &TileStorage, &Transform, &TilemapType)>,
    tile_query: Query<(&MovementCost, &TilePos)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (grid_size, storage, map_transform, map_type) = tilemap_query.single();

        if let Some(tile_pos) = current_tile.hovered {
            let goal = TilePos { x: 4, y: 6 };
            let path = goal.find_path_to(&tile_pos, storage, map_type, &tile_query);

            if let Some((path, _)) = path {
                spawn_unit(
                    commands,
                    texture_assets,
                    tile_pos.world_position(grid_size, map_transform, map_type),
                    path.into_iter()
                        .map(|p| p.world_position(grid_size, map_transform, map_type))
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
        match target_location.waypoints.last() {
            Some(target) => match target.distance(transform.translation.xy()) {
                d if d < 1.0 => {
                    target_location.waypoints.pop();
                }
                _ => {
                    let direction = (target.extend(transform.translation.z)
                        - transform.translation)
                        .normalize();
                    transform.translation += direction * time.delta_seconds() * 100.0;
                }
            },
            None => {
                commands.entity(entity).remove::<TargetLocation>();
            }
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

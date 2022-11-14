use crate::{components::GameState, loading::TextureAssets, prelude::*};

#[derive(Component, Reflect)]
pub struct Worker;

pub fn spawn_worker(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: texture_assets.texture_bevy.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Worker)
        .insert(Name::new("Worker"));
}

pub fn move_worker(mut query: Query<(&mut Transform, &Worker)>, t: Res<Time>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.x += 100.0 * t.delta_seconds();
    }
}

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Worker>()
            .add_enter_system(GameState::Playing, spawn_worker)
            .add_system(move_worker.run_in_state(GameState::Playing));
    }
}

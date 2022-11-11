use crate::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_tilemap::prelude::*;

mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .insert_resource(ImageSettings::default_nearest())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(systems::startup));
    }
}

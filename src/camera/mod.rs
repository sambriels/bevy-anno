use bevy::prelude::*;
pub struct CameraPlugin;

mod systems;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::movement);
    }
}

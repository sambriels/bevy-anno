mod camera;
mod components;
mod loading;
mod worker;
mod world;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub use iyes_progress::prelude::*;
    pub use rand::prelude::*;
}

use bevy_inspector_egui::WorldInspectorPlugin;
use prelude::*;

fn main() {
    App::new()
        .add_loopless_state(components::GameState::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(loading::LoadingPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(worker::WorkerPlugin)
        .run();

    // #[cfg(debug_assertions)]
    // {
    //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
    //         .add_plugin(LogDiagnosticsPlugin::default());
    // }
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let sprite_handle = asset_server.load("workman.png");

//     commands.spawn_bundle(SpriteBundle {
//         texture: sprite_handle,
//         transform: Transform::from_xyz(0.0, 0.0, 1.0),
//         ..default()
//     });
// }

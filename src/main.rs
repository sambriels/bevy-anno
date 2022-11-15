mod camera;
mod components;
mod loading;
mod pathfinding;
mod utils;
mod worker;
mod world;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_prototype_debug_lines::*;
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
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(loading::LoadingPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(pathfinding::PathfindingPlugin)
        .add_plugin(worker::WorkerPlugin)
        .run();

    // #[cfg(debug_assertions)]
    // {
    //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
    //         .add_plugin(LogDiagnosticsPlugin::default());
    // }
}

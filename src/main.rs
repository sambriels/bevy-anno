mod camera;
mod loading;
mod world;

// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod prelude {
    pub use crate::camera::*;
    pub use crate::loading::*;
    pub use crate::world::*;
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    // pub use iyes_loopless::prelude::*;
    // This example game uses States to separate logic
    // See https://bevy-cheatbook.github.io/programming/states.html
    // Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    pub enum GameState {
        // During the loading State the LoadingPlugin will load our assets
        AssetLoading,
        // During this State the actual game logic is executed
        Playing,
        // Here the menu is drawn and waiting for player interaction
        Menu,
    }
}

use prelude::*;

fn main() {
    App::new()
        // .add_loopless_state(GameState::Loading)
        // .add_loading_state(
        //     LoadingState::new(GameState::Loading)
        //         .continue_to_state(GameState::Playing)
        //         .with_collection::<TextureAssets>(),
        // )
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldPlugin)
        // .add_startup_system(setup)
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

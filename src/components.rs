use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    AssetLoading,
    WorldBuilding,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    // Menu,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct MyTile {
    pub is_walkable: bool,
}

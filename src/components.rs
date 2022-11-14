use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    WorldBuilding,
    Playing,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct MyTile {
    pub is_walkable: bool,
}
#[derive(Default)]
pub struct CursorPos(pub Vec3);

use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    WorldBuilding,
    Playing,
}

#[derive(Default)]
pub struct CursorPos(pub Vec3);

use crate::prelude::*;
use bevy::render::texture::ImageSettings;
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::WorldBuilding)
                .with_collection::<FontAssets>()
                .with_collection::<TextureAssets>(),
        )
        .insert_resource(ImageSettings::default_nearest());
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/workman.png")]
    pub workman: Handle<Image>,
    #[asset(path = "textures/tiles.png")]
    pub tiles: Handle<Image>,
}

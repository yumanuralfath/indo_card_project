use bevy::prelude::*;

/// Card visual assets
#[derive(Resource)]
pub struct CardAssets {
    pub font: Handle<Font>,
}

impl CardAssets {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

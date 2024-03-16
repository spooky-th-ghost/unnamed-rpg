use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Preload)
                .continue_to_state(GameState::Overworld)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/character_models.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/player_animations.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/materials.assets.ron",
                )
                .load_collection::<CharacterCache>()
                .load_collection::<PlayerAnimationCache>()
                .load_collection::<MaterialCache>(),
        );
    }
}

#[derive(Resource, AssetCollection)]
pub struct MaterialCache {
    #[asset(key = "checkerboard")]
    pub checkerboard: Handle<StandardMaterial>,
}

#[derive(Resource, AssetCollection)]
pub struct CharacterCache {
    #[asset(key = "uli")]
    pub uli: Handle<Scene>,
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAnimationCache {
    #[asset(key = "idle")]
    pub idle: Handle<AnimationClip>,
    #[asset(key = "run")]
    pub run: Handle<AnimationClip>,
}

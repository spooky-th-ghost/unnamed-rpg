use crate::physics::types::MeshColliderMarker;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_registry_export::*;
use bevy_xpbd_3d::prelude::*;

use crate::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExportRegistryPlugin::default(),
            BlueprintsPlugin {
                library_folder: "scenes/library".into(),
                format: GltfFormat::GLB,
                legacy_mode: false,
                ..Default::default()
            },
        ))
        .add_loading_state(
            LoadingState::new(GameState::Preload)
                .continue_to_state(GameState::Overworld)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/character_models.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/player_animations.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/environmental_animation.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "manifests/materials.assets.ron",
                )
                .load_collection::<CharacterCache>()
                .load_collection::<PlayerAnimationCache>()
                .load_collection::<EnvironmentalAnimationCache>()
                .load_collection::<MaterialCache>()
                .load_collection::<BlueprintCache>(),
        )
        .add_systems(Update, insert_mesh_colliders);
    }
}

#[derive(Resource, AssetCollection)]
pub struct BlueprintCache {
    #[asset(path = "scenes/library", collection(typed))]
    pub models: Vec<Handle<bevy::gltf::Gltf>>,
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
pub struct EnvironmentalAnimationCache {
    #[asset(key = "dumpster_closed")]
    pub dumpster_closed: Handle<AnimationClip>,
    #[asset(key = "dumpster_opening")]
    pub dumpster_opening: Handle<AnimationClip>,
    #[asset(key = "dumpster_open")]
    pub dumpster_open: Handle<AnimationClip>,
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAnimationCache {
    #[asset(key = "idle")]
    pub idle: Handle<AnimationClip>,
    #[asset(key = "run")]
    pub run: Handle<AnimationClip>,
}

fn insert_mesh_colliders(
    mut commands: Commands,
    parent_query: Query<(Entity, &MeshColliderMarker)>,
    children: Query<&Children>,
    mesh_query: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, _marker) in &parent_query {
        for child in children.iter_descendants(entity) {
            if let Ok(mesh_handle) = mesh_query.get(child) {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    if let Some(collider) = Collider::trimesh_from_mesh(&mesh) {
                        commands.entity(child).insert(collider);
                        commands.entity(entity).remove::<MeshColliderMarker>();
                    }
                }
            }
        }
    }
}

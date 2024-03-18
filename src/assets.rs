use crate::physics::types::MeshColliderMarker;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_rapier3d::prelude::*;
use bevy_registry_export::*;

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
                    "manifests/materials.assets.ron",
                )
                .load_collection::<CharacterCache>()
                .load_collection::<PlayerAnimationCache>()
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
                    if let Some(collider) =
                        Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
                    {
                        let mut child_entity = commands.entity(child);
                        child_entity.insert(collider).insert(RigidBody::Fixed);
                        commands.entity(entity).remove::<MeshColliderMarker>();
                    }
                }
            }
        }
    }
}

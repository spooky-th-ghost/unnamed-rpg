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
    idle: Handle<AnimationClip>,
    #[asset(key = "run")]
    run: Handle<AnimationClip>,
    #[asset(key = "jump")]
    jump: Handle<AnimationClip>,
    #[asset(key = "rising")]
    rising: Handle<AnimationClip>,
    #[asset(key = "long-jump")]
    long_jump: Handle<AnimationClip>,
    #[asset(key = "long-jump-held")]
    long_jump_held: Handle<AnimationClip>,
    #[asset(key = "dive")]
    dive: Handle<AnimationClip>,
    #[asset(key = "dive-held")]
    dive_held: Handle<AnimationClip>,
}

type Clip = Handle<AnimationClip>;

impl PlayerAnimationCache {
    pub fn idle(&self) -> Clip {
        self.idle.clone_weak()
    }

    pub fn run(&self) -> Clip {
        self.run.clone_weak()
    }

    pub fn jump(&self) -> Clip {
        self.jump.clone_weak()
    }

    pub fn rising(&self) -> Clip {
        self.rising.clone_weak()
    }

    pub fn long_jump(&self) -> Clip {
        self.long_jump.clone_weak()
    }

    pub fn long_jump_held(&self) -> Clip {
        self.long_jump_held.clone_weak()
    }

    pub fn dive(&self) -> Clip {
        self.dive.clone_weak()
    }

    pub fn dive_held(&self) -> Clip {
        self.dive_held.clone_weak()
    }
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

mod victimless_asset {
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::*;
    use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
    use bevy_registry_export::*;
    use std::marker::PhantomData;

    pub struct VictimlessAssetPlugin<T, B, A>
    where
        T: States + Clone + Copy + Default + Eq + PartialEq + std::hash::Hash + std::fmt::Debug,
        B: Resource + AssetCollection,
        A: Resource + AssetCollection,
    {
        _blueprint_cache: PhantomData<B>,
        _animation_cache: PhantomData<A>,
        loading_state: T,
        post_loading_state: T,
    }

    impl<T, B, A> VictimlessAssetPlugin<T, B, A>
    where
        T: States + Clone + Copy + Default + Eq + PartialEq + std::hash::Hash + std::fmt::Debug,
        B: Resource + AssetCollection,
        A: Resource + AssetCollection,
    {
        pub fn new(loading_state: T, post_loading_state: T) -> Self {
            Self {
                _blueprint_cache: PhantomData,
                _animation_cache: PhantomData,
                loading_state,
                post_loading_state,
            }
        }
    }

    impl<T, B, A> Plugin for VictimlessAssetPlugin<T, B, A>
    where
        T: States + Clone + Copy + Default + Eq + PartialEq + std::hash::Hash + std::fmt::Debug,
        B: Resource + AssetCollection,
        A: Resource + AssetCollection,
    {
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
                LoadingState::new(self.loading_state)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        "manifests/animations.assets.ron",
                    )
                    .load_collection::<B>()
                    .load_collection::<A>(),
            );
        }
    }
}

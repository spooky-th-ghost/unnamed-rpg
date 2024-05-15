use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(PhysicsLayer, Default, Clone, Copy, Debug, Reflect)]
pub enum CollisionLayer {
    #[default]
    Character,
    Object,
    Vehicle,
    Terrain,
    AreaTransition,
    Item,
}

impl CollisionLayer {
    pub fn standable_mask() -> LayerMask {
        LayerMask(0b1010)
    }

    pub fn terrain_layers() -> CollisionLayers {
        CollisionLayers::new(
            CollisionLayer::Terrain,
            [
                CollisionLayer::Vehicle,
                CollisionLayer::Character,
                CollisionLayer::Object,
            ],
        )
    }

    pub fn character_layers() -> CollisionLayers {
        CollisionLayers::new(
            CollisionLayer::Character,
            [
                CollisionLayer::Vehicle,
                CollisionLayer::Character,
                CollisionLayer::Item,
                CollisionLayer::Terrain,
                CollisionLayer::Object,
            ],
        )
    }

    pub fn object_layers() -> CollisionLayers {
        CollisionLayers::new(
            CollisionLayer::Object,
            [
                CollisionLayer::Vehicle,
                CollisionLayer::Character,
                CollisionLayer::Terrain,
                CollisionLayer::Object,
            ],
        )
    }

    pub fn item_layers() -> CollisionLayers {
        CollisionLayers::new(CollisionLayer::Item, [CollisionLayer::Character])
    }

    pub fn transition_layers() -> CollisionLayers {
        CollisionLayers::new(CollisionLayer::AreaTransition, [CollisionLayer::Character])
    }
}

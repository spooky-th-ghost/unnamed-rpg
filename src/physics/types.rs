use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

pub struct PhysicsTypesPlugin;

impl Plugin for PhysicsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_type::<MoveDirection>()
            .register_type::<Character>()
            .register_type::<Grounded>()
            .register_type::<MeshColliderMarker>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MoveDirection(Vec3);

impl MoveDirection {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn is_any(&self) -> bool {
        self.0 != Vec3::ZERO
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Momentum(Vec3);

impl Momentum {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn is_any(&self) -> bool {
        self.0 != Vec3::ZERO
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Speed(f32);

impl Speed {
    pub fn new(value: f32) -> Self {
        Speed(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MeshColliderMarker(f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Grounded;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Jumping;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Regrab;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Character {
    pub ride_height: f32,
    pub spring_strength: f32,
    pub spring_damper: f32,
    pub jump_strength: f32,
    pub base_gravity_scale: f32,
    pub regrab_gravity_scale: f32,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            ride_height: 1.4,
            spring_strength: 23.0,
            spring_damper: 5.0,
            jump_strength: 17.5,
            base_gravity_scale: 2.0,
            regrab_gravity_scale: 1.5,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
    external_force: ExternalForce,
    restitution: Restitution,
    character: Character,
    shape_caster: ShapeCaster,
    gravity_scale: GravityScale,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::capsule(1.0, 0.5),
            external_force: ExternalForce::new(Vec3::ZERO).with_persistence(false),
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            character: Character {
                ride_height: 1.4,
                spring_strength: 23.0,
                spring_damper: 5.0,
                jump_strength: 17.5,
                base_gravity_scale: 2.0,
                regrab_gravity_scale: 1.5,
            },
            shape_caster: ShapeCaster::new(
                Collider::capsule(1.0, 0.35),
                Vec3::NEG_Y * 0.05,
                Quaternion::default(),
                Direction3d::NEG_Y,
            )
            .with_max_time_of_impact(1.0)
            .with_max_hits(1)
            .with_ignore_self(true),
            gravity_scale: GravityScale(2.0),
        }
    }
}

#[derive(Component)]
pub struct GroundSensor;

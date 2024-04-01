use bevy::prelude::*;

pub struct PhysicsTypesPlugin;

impl Plugin for PhysicsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_type::<MoveDirection>()
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

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct GroundSensor;

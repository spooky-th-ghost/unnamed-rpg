use bevy::prelude::*;

#[derive(Component, Default)]
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

#[derive(Component, Default)]
pub struct Speed(f32);

impl Speed {
    pub fn new(value: f32) -> Self {
        Speed(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct Character;

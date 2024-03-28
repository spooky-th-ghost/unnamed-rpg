use bevy::prelude::*;

pub mod systems;
pub mod types;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((systems::PhysicsSystemPlugin, types::PhysicsTypesPlugin));
    }
}

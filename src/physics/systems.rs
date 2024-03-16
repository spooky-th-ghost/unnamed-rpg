use super::types::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_to_target);
    }
}

fn move_to_target(time: Res<Time>, mut query: Query<(&mut Velocity, &Speed, &MoveDirection)>) {
    for (mut velocity, speed, direction) in &mut query {
        if direction.any() {
            velocity.linvel = direction.get() * time.delta_seconds() * speed.get();
        }
    }
}

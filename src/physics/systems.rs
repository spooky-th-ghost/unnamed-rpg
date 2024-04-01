use super::types::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_to_target, rotate_to_direction).run_if(in_state(GameState::Overworld)),
        );
    }
}

fn move_to_target(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &Speed, &MoveDirection)>,
) {
    for (mut velocity, speed, direction) in &mut query {
        if direction.is_any() {
            let desired_velocity = direction.get() * time.delta_seconds() * speed.get();
            velocity.x = desired_velocity.x;
            velocity.z = desired_velocity.z;
        }
    }
}

fn rotate_to_direction(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveDirection, &Speed), (With<Character>, With<Grounded>)>,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, direction, speed) in &mut query {
        rotation_target.translation = transform.translation;
        let dir = direction.get();
        let flat_velo_direction = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
        if flat_velo_direction != Vec3::ZERO {
            let target_position = rotation_target.translation + flat_velo_direction;

            rotation_target.look_at(target_position, Vec3::Y);
            let turn_speed = speed.get() * 0.085;

            transform.rotation = transform
                .rotation
                .slerp(rotation_target.rotation, time.delta_seconds() * turn_speed);
        }
    }
}

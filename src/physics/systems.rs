use super::types::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_to_target, rotate_to_direction, floating_capsule)
                .run_if(in_state(GameState::Overworld)),
        );
    }
}

fn move_to_target(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &Transform, &Speed, &MoveDirection)>,
) {
    for (mut velocity, transform, speed, direction) in &mut query {
        if direction.is_any() {
            let desired_velocity = time.delta_seconds() * speed.get() * *transform.forward();
            velocity.x = desired_velocity.x;
            velocity.z = desired_velocity.z;
        }
    }
}

fn rotate_to_direction(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveDirection, &Speed, &ShapeHits), With<Character>>,
    mut rotation_target: Local<Transform>,
) {
    for (mut transform, direction, speed, ground_hits) in &mut query {
        if !ground_hits.is_empty() {
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
}

fn floating_capsule(
    mut character_query: Query<(&mut ExternalForce, &LinearVelocity, &ShapeHits, &Character)>,
    velocity_query: Query<&LinearVelocity, Without<Character>>,
) {
    for (mut force, velocity, ground_hits, character) in &mut character_query {
        if !ground_hits.is_empty() {
            let ray_dir = Vec3::Y;
            let mut other_velocity = LinearVelocity::default();
            let mut distance = 0.0_f32;

            if let Some(shape_hit_data) = ground_hits.iter().next() {
                distance = shape_hit_data.time_of_impact;
                if let Ok(lin_vel) = velocity_query.get(shape_hit_data.entity) {
                    other_velocity.0 = lin_vel.0;
                }
            }

            let self_downward_force = ray_dir.dot(velocity.0);
            let other_downard_force = ray_dir.dot(other_velocity.0);

            let relative_force = self_downward_force - other_downard_force;

            let x = -distance + character.ride_height;

            let spring_force =
                (x * character.spring_strength) - (relative_force * character.spring_damper);

            let applied_force = ray_dir * spring_force;
            force.set_force(applied_force);
        }
    }
}

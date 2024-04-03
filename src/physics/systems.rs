use super::types::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::{math::Quaternion, prelude::*};

pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                replace_character_physics_settings,
                move_to_target,
                rotate_to_direction,
                floating_capsule,
                lateral_damping,
                handle_coyote_time,
            )
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

fn lateral_damping(time: Res<Time>, mut query: Query<(&mut LinearVelocity, &LateralDamping)>) {
    for (mut velocity, damping) in &mut query {
        let mut velocity_vec = velocity.0;
        velocity_vec *= 1.0 / (1.0 + time.delta_seconds() * damping.0);
        velocity.x = velocity_vec.x;
        velocity.z = velocity_vec.z;
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
    mut commands: Commands,
    mut character_query: Query<(
        Entity,
        &mut ExternalForce,
        &LinearVelocity,
        &ShapeHits,
        &Character,
        Has<Grounded>,
        Has<Jumping>,
    )>,
    velocity_query: Query<&LinearVelocity, Without<Character>>,
) {
    for (entity, mut force, velocity, ground_hits, character, has_grounded, has_jumping) in
        &mut character_query
    {
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
            if !has_grounded && !has_jumping {
                commands.entity(entity).insert(Grounded);
            }
        } else {
            if has_grounded {
                commands.entity(entity).remove::<Grounded>();
                if !has_jumping {
                    commands.entity(entity).insert(CoyoteTime::default());
                }
            }
        }
    }
}

fn handle_coyote_time(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CoyoteTime)>,
) {
    for (entity, mut coyote_time) in &mut query {
        coyote_time.tick(time.delta());
        if coyote_time.finished() {
            commands.entity(entity).remove::<CoyoteTime>();
        }
    }
}

fn replace_character_physics_settings(
    mut commands: Commands,
    query: Query<(Entity, &CharacterPhysicsSettings), Added<CharacterPhysicsSettings>>,
) {
    for (entity, settings) in &query {
        commands
            .entity(entity)
            .remove::<CharacterPhysicsSettings>()
            .insert(CharacterBundle {
                collider: Collider::capsule(settings.collider_height, settings.collider_radius),
                character: Character {
                    ride_height: 1.4 * settings.collider_height,
                    spring_strength: 23.0,
                    spring_damper: 5.0,
                    jump_strength: 17.5,
                    base_gravity_scale: 2.0,
                    regrab_gravity_scale: 1.5,
                },
                shape_caster: ShapeCaster::new(
                    Collider::capsule(settings.collider_height, settings.collider_radius * 0.7),
                    Vec3::NEG_Y * (0.05 * settings.collider_height),
                    Quaternion::default(),
                    Direction3d::NEG_Y,
                )
                .with_max_time_of_impact(settings.collider_height)
                .with_max_hits(1)
                .with_ignore_self(true),
                ..default()
            });
    }
}

use super::types::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_to_target,
                handle_grounded,
                rotate_to_direction,
                detect_ground,
            )
                .run_if(in_state(GameState::Overworld)),
        );
    }
}

fn move_to_target(time: Res<Time>, mut query: Query<(&mut Velocity, &Speed, &MoveDirection)>) {
    for (mut velocity, speed, direction) in &mut query {
        if direction.is_any() {
            velocity.linvel = direction.get() * time.delta_seconds() * speed.get();
        }
    }
}

fn handle_grounded(
    mut commands: Commands,
    character_query: Query<(Entity, &Transform, Has<Grounded>), With<RigidBody>>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, has_grounded) in &character_query {
        let ray_pos = transform.translation;
        let ray_dir = Vec3::Y * -1.0;
        let max_distance = 1.1;
        let solid = true;
        let filter = QueryFilter::exclude_dynamic().exclude_sensors();

        let ray_result =
            rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_distance, solid, filter);

        if let Some((_, _)) = ray_result {
            if !has_grounded {
                commands.entity(entity).insert(Grounded);
            }
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

fn detect_ground(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &Transform, Has<Grounded>), With<GroundSensor>>,
) {
    for (entity, transform, is_grounded) in &query {
        let max_toi = 1.2;

        if let Some(_) = rapier_context.cast_ray(
            transform.translation,
            Vec3::NEG_Y,
            max_toi,
            false,
            QueryFilter::exclude_dynamic(),
        ) {
            if !is_grounded {
                commands.entity(entity).insert(Grounded);
            }
        } else {
            if is_grounded {
                commands.entity(entity).remove::<Grounded>();
            }
        }
    }
}

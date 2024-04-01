use crate::animation::{Animated, AnimationInit, AnimationMap, AnimationTransitionEvent};
use crate::assets::{CharacterCache, PlayerAnimationCache};
use crate::camera::CameraData;
use crate::input::{InputBuffer, InputListenerBundle, PlayerAction};
use crate::physics::types::{Character, Grounded, Momentum, MoveDirection, Speed};
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*, PhysicsSchedule, PhysicsStepSet};
use leafwing_input_manager::action_state::ActionState;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerData::default())
            .add_systems(OnEnter(GameState::Overworld), spawn_overworld_player)
            .add_systems(
                Update,
                (
                    set_player_direction,
                    play_idle_animation,
                    transition_player_state,
                    update_player_data,
                    jump,
                )
                    .run_if(in_state(GameState::Overworld)),
            );
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Diving,
    BellySliding,
    #[default]
    Idle,
    Walking,
    Running,
    LongJumping,
    Rising,
    Freefall,
    Walljumping,
    Carrying,
    ButtSliding,
    Sliding,
}

#[derive(Component, Default, Clone, Copy, Deref)]
pub struct Player {
    #[deref]
    pub state: PlayerState,
}

#[derive(Event)]
pub struct PlayerStateTransitionEvent {
    pub current_state: PlayerState,
    pub new_state: PlayerState,
}

#[derive(Resource, Default)]
pub struct PlayerData {
    pub player_position: Vec3,
    pub distance_from_floor: f32,
    pub floor_normal: Vec3,
    pub speed: f32,
    pub defacto_speed: f32,
    pub kicked_wall: Option<Entity>,
    pub jump_stage: u8,
}

fn spawn_overworld_player(mut commands: Commands, characters: Res<CharacterCache>) {
    commands.spawn((
        Name::from("Player"),
        SceneBundle {
            scene: characters.uli.clone_weak(),
            ..default()
        },
        Player {
            state: PlayerState::Idle,
        },
        Character,
        RigidBody::Dynamic,
        InputBuffer::default(),
        InputListenerBundle::input_map(),
        MoveDirection::default(),
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule(0.4, 0.4),
        ShapeCaster::new(
            Collider::capsule(0.9, 0.35),
            Vec3::NEG_Y * 0.05,
            Quaternion::default(),
            Direction3d::NEG_Y,
        )
        .with_max_time_of_impact(0.2)
        .with_max_hits(1)
        .with_ignore_self(true),
        GravityScale(2.0),
        Speed::new(200.0),
        Momentum::default(),
        Animated,
    ));
}

fn play_idle_animation(
    mut commands: Commands,
    animation_map: Res<AnimationMap>,
    player_query: Query<Entity, (With<Player>, Without<AnimationInit>)>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    assets: Res<AssetServer>,
) {
    for entity in &player_query {
        if let Some(animation_entity) = animation_map.get(entity) {
            if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity) {
                animation_player
                    .play(assets.load("models/uli.glb#Animation0"))
                    .repeat();

                commands.entity(entity).insert(AnimationInit);
            }
        }
    }
}

fn set_player_direction(
    camera_data: Res<CameraData>,
    mut query: Query<(&mut MoveDirection, &ActionState<PlayerAction>)>,
) {
    for (mut direction, action) in &mut query {
        if action.pressed(&PlayerAction::Move) {
            let axis_pair = action.clamped_axis_pair(&PlayerAction::Move).unwrap();
            let x = axis_pair.x();
            let z = axis_pair.y();

            direction.set(camera_data.translate_direction_in_camera_space(x, z));
        } else {
            direction.set(Vec3::ZERO);
        }
    }
}

fn handle_state_transition_events(
    mut state_events: EventWriter<PlayerStateTransitionEvent>,
    player_query: Query<&Player>,
    mut previous_state: Local<PlayerState>,
) {
    for player in &player_query {
        if player.state != *previous_state {
            state_events.send(PlayerStateTransitionEvent {
                current_state: *previous_state,
                new_state: player.state,
            });
        }
        *previous_state = player.state;
    }
}

fn update_player_data(
    mut player_data: ResMut<PlayerData>,
    player_query: Query<&Transform, With<Player>>,
) {
    for transform in &player_query {
        player_data.player_position = transform.translation;
    }
}

fn transition_player_state(
    mut animation_transitions: EventWriter<AnimationTransitionEvent>,
    animation_cache: Res<PlayerAnimationCache>,
    mut player_query: Query<(Entity, &mut Player, &MoveDirection, Has<Grounded>)>,
) {
    for (entity, mut player, direction, is_grounded) in &mut player_query {
        if is_grounded {
            if direction.is_any() {
                if player.state != PlayerState::Running {
                    player.state = PlayerState::Running;
                    animation_transitions.send(AnimationTransitionEvent {
                        entity,
                        clip: animation_cache.run.clone_weak(),
                        transition: Duration::from_secs_f32(0.2),
                    });
                }
            } else {
                if player.state != PlayerState::Idle {
                    player.state = PlayerState::Idle;
                    animation_transitions.send(AnimationTransitionEvent {
                        entity,
                        clip: animation_cache.idle.clone_weak(),
                        transition: Duration::from_secs_f32(0.3),
                    });
                }
            }
        }
    }
}

fn jump(mut player_query: Query<(&mut LinearVelocity, &InputBuffer, &ShapeHits)>) {
    for (mut velocity, action, ground_hits) in &mut player_query {
        if action.pressed(PlayerAction::Jump) && !ground_hits.is_empty() {
            velocity.y += 8.0;
        }
    }
}

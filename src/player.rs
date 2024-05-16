use crate::animation::{Animated, AnimationInit, AnimationMap, AnimationTransitionEvent};
use crate::assets::{CharacterCache, PlayerAnimationCache};
use crate::camera::CameraData;
use crate::environment::{Transition, TransitionDestination};
use crate::input::{InputBuffer, InputListenerBundle, PlayerAction};
use crate::physics::types::{
    Character, CharacterBundle, CoyoteTime, Grounded, Jumping, Momentum, MoveDirection, MoveSpeed,
    Regrab,
};
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerData::new(200.0))
            .register_type::<PlayerData>()
            .add_systems(OnEnter(GameState::Overworld), spawn_overworld_player)
            .add_systems(
                Update,
                (
                    set_player_direction,
                    play_idle_animation,
                    transition_player_state,
                    update_player_data,
                    jump,
                    handle_regrab,
                    handle_jumping,
                    handle_transitions,
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
          
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerData {
    pub player_position: Vec3,
    pub player_velocity: Vec3,
    pub distance_from_floor: f32,
    pub floor_normal: Vec3,
    pub speed: f32,
    pub defacto_speed: f32,
    pub kicked_wall: Option<Entity>,
    pub jump_stage: u8,
    pub player_base_speed: f32,
    pub player_current_speed: f32,
    pub player_max_speed: f32,
}

impl PlayerData {
    pub fn new(speed: f32) -> Self {
        PlayerData {
            player_base_speed: speed,
            player_current_speed: speed,
            player_max_speed: speed * 2.0,
            ..default()
        }
    }
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
        CharacterBundle::default(),
        InputBuffer::default(),
        InputListenerBundle::input_map(),
        MoveDirection::default(),
        MoveSpeed::new(200.0),
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
                    .play(assets.load("models/uli.glb#Animation1"))
                    .repeat();

                commands.entity(entity).insert(AnimationInit);
            }
        }
    }
}

fn set_player_direction(
    camera_data: Res<CameraData>,
    mut query: Query<(
        &mut MoveDirection,
        &mut MoveSpeed,
        &ActionState<PlayerAction>,
        Has<Grounded>,
    )>,
) {
    for (mut direction, mut speed, action, has_grounded) in &mut query {
        if has_grounded {
            if action.pressed(&PlayerAction::Move) {
                let axis_pair = action.clamped_axis_pair(&PlayerAction::Move).unwrap();
                let x = axis_pair.x();
                let z = axis_pair.y();

                direction.set(camera_data.translate_direction_in_camera_space(x, z));
            } else {
                direction.set(Vec3::ZERO);
            }

            if direction.started_moving() {
                speed.start_moving();
            }

            if direction.stopped_moving() {
                speed.stop_moving();
            }
        }
    }
}

fn update_player_data(
    mut player_data: ResMut<PlayerData>,
    player_query: Query<(&Transform, &LinearVelocity, &MoveSpeed), With<Player>>,
) {
    for (transform, velocity, speed) in &player_query {
        player_data.player_position = transform.translation;
        player_data.player_velocity = velocity.0;
        player_data.player_current_speed = speed.get();
    }
}

fn transition_player_state(
    mut animation_transitions: EventWriter<AnimationTransitionEvent>,
    animation_cache: Res<PlayerAnimationCache>,
    mut player_query: Query<(Entity, &mut Player, &MoveDirection, &ShapeHits)>,
) {
    for (entity, mut player, direction, ground_hits) in &mut player_query {
        if !ground_hits.is_empty() {
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

fn handle_jumping(
    mut commands: Commands,
    mut character_query: Query<(Entity, &mut LinearVelocity, &InputBuffer), With<Jumping>>,
) {
    for (entity, mut velocity, input_buffer) in &mut character_query {
        if input_buffer.released(PlayerAction::Jump) || velocity.y <= 0.0 {
            commands.entity(entity).remove::<Jumping>();
            velocity.y = 0.0;
        }
    }
}

fn jump(
    mut commands: Commands,
    mut character_query: Query<(
        Entity,
        &mut LinearVelocity,
        &Character,
        &InputBuffer,
        Has<Grounded>,
        Has<CoyoteTime>,
    )>,
) {
    for (entity, mut velocity, character, input, has_grounded, has_coyote_time) in
        &mut character_query
    {
        if (has_grounded || has_coyote_time) && input.just_pressed(PlayerAction::Jump) {
            velocity.y = character.jump_strength;
            commands.entity(entity).insert(Jumping);
            if has_coyote_time {
                commands.entity(entity).remove::<CoyoteTime>();
            }
            if has_grounded {
                commands.entity(entity).remove::<Grounded>();
            }
        }
    }
}

fn handle_regrab(
    mut commands: Commands,
    mut character_query: Query<(
        Entity,
        &mut GravityScale,
        &ShapeHits,
        &Character,
        &InputBuffer,
        Has<Regrab>,
        Has<Jumping>,
    )>,
) {
    for (entity, mut gravity_scale, ground_hits, character, input, is_regrabbing, is_jumping) in
        &mut character_query
    {
        if (!ground_hits.is_empty() || input.released(PlayerAction::Jump)) && is_regrabbing {
            commands.entity(entity).remove::<Regrab>();
            gravity_scale.0 = character.base_gravity_scale;
        }

        if input.just_pressed(PlayerAction::Jump)
            && ground_hits.is_empty()
            && !is_jumping
            && !is_regrabbing
        {
            commands.entity(entity).insert(Regrab);
            gravity_scale.0 = character.regrab_gravity_scale;
        }
    }
}

fn handle_transitions(
    // Here we query the players entity, to check if it has collided with anything, as well as its
    // Transform, to move it if we hit a transition
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    // Here we query for any transitions in the world, the Without<Player> filter here is needed to
    // ensure we don't have overlap between a mutable (writeable) and immutable (read-only) query
    transitions_query: Query<&Transition, Without<Player>>,
    // This is a resource from our physics library (bevy_xpdb) that lists all colllisions happening
    // on the current frame
    collisions: Res<Collisions>,
) {
    // We know there is only one entity with the player component so we grab it with .get_single()
    // here
    if let Ok((player_entity, mut player_transform)) = player_query.get_single_mut() {
        // Here we iterate through each collision this frame with our player entity
        for collision in collisions.collisions_with_entity(player_entity) {
            //The player could be entity1 or entity2 in the collision so this code just ensures we
            //are grabbing the entity that is not our player
            let transition_entity = if collision.entity1 == player_entity {
                collision.entity2
            } else {
                collision.entity1
            };

            // 1. Check the transitions_query to find what if we hit an object with a `Transition`
            //    component
            if let Ok(transition) = transitions_query.get(transition_entity)
            {
                match transition.destination
                    {
                        TransitionDestination::Location(transitionVector) => player_transform.translation = transitionVector,
                        
                        _ => ()
                    }
            }
            // 2. If we did, we need to `match` against transition.destination what type of transition we should do (Location or Scene, just focus on Location for now
            // 3. Once we have the target location, set `player_transform.translation` to match it)
        }
    }
}

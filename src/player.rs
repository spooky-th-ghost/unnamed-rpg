use crate::animation::{Animated, AnimationCharacterMap, AnimationInit};
use crate::assets::CharacterCache;
use crate::camera::CameraData;
use crate::input::{InputBuffer, InputListenerBundle, PlayerAction};
use crate::physics::types::{MoveDirection, Speed};
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerData::default())
            .add_systems(OnEnter(GameState::Overworld), spawn_overworld_player)
            .add_systems(
                Update,
                (set_player_direction, play_idle_animation).run_if(in_state(GameState::Overworld)),
            );
    }
}

#[derive(Component)]
pub struct Player;

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
        SceneBundle {
            scene: characters.uli.clone_weak(),
            ..default()
        },
        Player,
        Collider::capsule_y(0.5, 0.5),
        RigidBody::Dynamic,
        Velocity::default(),
        InputBuffer::default(),
        InputListenerBundle::input_map(),
        MoveDirection::default(),
        LockedAxes::ROTATION_LOCKED,
        Speed::new(500.0),
        Animated,
    ));
}

fn play_idle_animation(
    mut commands: Commands,
    animation_map: Res<AnimationCharacterMap>,
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

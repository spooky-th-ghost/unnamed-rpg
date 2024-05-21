use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::GameState;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Animated;

#[derive(Component)]
pub struct AnimationInit;

#[derive(Resource, Default)]
pub struct AnimationMap(HashMap<Entity, Entity>);

impl AnimationMap {
    pub fn get(&self, key_entity: Entity) -> Option<Entity> {
        self.0.get(&key_entity).copied()
    }

    pub fn insert(&mut self, key_entity: Entity, value_entity: Entity) {
        self.0.insert(key_entity, value_entity);
    }
}

#[derive(Event)]
pub struct AnimationTransitionEvent {
    pub entity: Entity,
    pub clip: Handle<AnimationClip>,
    pub transition: Duration,
    pub play_after: Option<Handle<AnimationClip>>,
    pub looping: bool,
}

impl AnimationTransitionEvent {
    pub fn single(entity: Entity, clip: Handle<AnimationClip>, transition_seconds: f32) -> Self {
        Self {
            entity,
            clip,
            transition: Duration::from_secs_f32(transition_seconds),
            play_after: None,
            looping: true,
        }
    }

    pub fn double(
        entity: Entity,
        clip: Handle<AnimationClip>,
        transition_seconds: f32,
        second_clip: Handle<AnimationClip>,
    ) -> Self {
        Self {
            entity,
            clip,
            transition: Duration::from_secs_f32(transition_seconds),
            play_after: Some(second_clip),
            looping: false,
        }
    }
}

#[derive(Component)]
pub struct QueuedAnimation(Handle<AnimationClip>);

fn store_animation_relationships(
    mut commands: Commands,
    mut animation_map: ResMut<AnimationMap>,
    child_query: Query<(Entity, &Parent), Added<AnimationPlayer>>,
    grandparent_query: Query<(Entity, &Children), With<Animated>>,
) {
    for (grandchild_entity, grandchild_parent) in &child_query {
        for (grandparent_entity, grandparent_children) in &grandparent_query {
            if grandparent_children
                .into_iter()
                .any(|entity| *entity == grandchild_parent.get())
            {
                animation_map.insert(grandparent_entity, grandchild_entity);
                commands.entity(grandparent_entity).remove::<Animated>();
            }
        }
    }
}

fn handle_animation_transition_events(
    mut commands: Commands,
    mut transition_events: EventReader<AnimationTransitionEvent>,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer)>,
    animation_character_map: Res<AnimationMap>,
) {
    for event in transition_events.read() {
        if let Some(animation_player_entity) = animation_character_map.get(event.entity) {
            if let Ok((entity, mut animation_player)) =
                animation_player_query.get_mut(animation_player_entity)
            {
                animation_player.play_with_transition(event.clip.clone_weak(), event.transition);
                let repeat_value = if event.looping {
                    bevy::animation::RepeatAnimation::Forever
                } else {
                    bevy::animation::RepeatAnimation::Never
                };
                animation_player.set_repeat(repeat_value);

                if let Some(queued_clip) = &event.play_after {
                    commands
                        .entity(entity)
                        .insert(QueuedAnimation(queued_clip.clone_weak()));
                }
            }
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationSet;

fn play_queued_animations(
    mut commands: Commands,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer, Option<&QueuedAnimation>)>,
) {
    for (entity, mut animation_player, has_queued_animation) in &mut animation_player_query {
        if let Some(queued_animation) = has_queued_animation {
            if animation_player.is_finished() {
                animation_player.play(queued_animation.0.clone_weak());
                animation_player.repeat();
                commands.entity(entity).remove::<QueuedAnimation>();
            }
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationMap::default())
            .add_event::<AnimationTransitionEvent>()
            .add_systems(
                Update,
                (
                    store_animation_relationships,
                    handle_animation_transition_events,
                    play_queued_animations,
                )
                    .run_if(in_state(GameState::Overworld))
                    .in_set(AnimationSet),
            );
    }
}

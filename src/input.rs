use crate::types::EngineSystemSet;
use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::{prelude::*, *};
use std::collections::HashSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(FixedUpdate, buffer_inputs.in_set(EngineSystemSet::Input));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default, Reflect)]
pub enum PlayerAction {
    #[default]
    Jump,
    Move,
    Interact,
    Crouch,
    CamRotateRight,
    CamRotateLeft,
    CamModeChangePositive,
    CamModeChangeNegative,
}

#[derive(Component, Default)]
pub struct InputBuffer {
    pressed_actions: HashSet<PlayerAction>,
    stale_actions: HashSet<PlayerAction>,
    buffered_actions: HashMap<PlayerAction, Timer>,
}

#[allow(unused)]
impl InputBuffer {
    pub fn just_pressed(&self, action: PlayerAction) -> bool {
        match self.pressed_actions.get(&action) {
            Some(_) => match self.stale_actions.get(&action) {
                None => return true,
                _ => (),
            },
            _ => (),
        }

        match self.buffered_actions.get(&action) {
            Some(_) => match self.stale_actions.get(&action) {
                Some(_) => false,
                None => true,
            },
            None => false,
        }
    }

    pub fn pressed(&self, action: PlayerAction) -> bool {
        self.pressed_actions.get(&action).is_some()
    }

    pub fn released(&self, action: PlayerAction) -> bool {
        self.pressed_actions.get(&action).is_none() && self.buffered_actions.get(&action).is_none()
    }

    pub fn press(&mut self, action: PlayerAction) {
        self.buffered_actions
            .insert(action, Timer::from_seconds(0.166, TimerMode::Once));
        self.pressed_actions.insert(action);
    }

    pub fn release(&mut self, action: PlayerAction) {
        self.buffered_actions.remove(&action);
        self.stale_actions.remove(&action);
        self.pressed_actions.remove(&action);
    }

    pub fn tick(&mut self, delta: std::time::Duration) {
        let mut stale_buffers: Vec<PlayerAction> = Vec::new();
        self.buffered_actions
            .iter_mut()
            .for_each(|(action, timer)| {
                timer.tick(delta);
                match timer.finished() {
                    true => {
                        self.stale_actions.insert(*action);
                        stale_buffers.push(*action);
                    }
                    _ => (),
                }
            });
        for action in stale_buffers.iter() {
            self.buffered_actions.remove(action);
            self.stale_actions.insert(*action);
        }
    }
}
#[derive(Bundle)]
pub struct InputListenerBundle {
    input_manager: InputManagerBundle<PlayerAction>,
}

impl InputListenerBundle {
    pub fn input_map() -> InputListenerBundle {
        use PlayerAction::*;

        let input_map = input_map::InputMap::new([
            (Jump, KeyCode::Space),
            (Interact, KeyCode::KeyL),
            (CamRotateLeft, KeyCode::ArrowLeft),
            (CamRotateRight, KeyCode::ArrowRight),
            (CamModeChangePositive, KeyCode::ArrowUp),
            (CamModeChangeNegative, KeyCode::ArrowDown),
        ])
        .insert_multiple([
            (Jump, GamepadButtonType::South),
            (Interact, GamepadButtonType::West),
            (CamRotateLeft, GamepadButtonType::LeftTrigger2),
            (CamRotateRight, GamepadButtonType::RightTrigger2),
        ])
        .insert(Move, DualAxis::left_stick())
        .insert(Move, VirtualDPad::wasd())
        .set_gamepad(Gamepad { id: 0 })
        .build();

        InputListenerBundle {
            input_manager: InputManagerBundle {
                input_map,
                ..Default::default()
            },
        }
    }
}

fn buffer_inputs(
    time: Res<Time>,
    mut input_buffer_query: Query<(&mut InputBuffer, &ActionState<PlayerAction>)>,
) {
    for (mut buffer, input) in &mut input_buffer_query {
        buffer.tick(time.delta());

        //Handle Presses
        for action in input.get_just_pressed().iter() {
            buffer.press(*action);
        }

        //Handle Releases
        for action in input.get_just_released().iter() {
            buffer.release(*action);
        }
    }
}

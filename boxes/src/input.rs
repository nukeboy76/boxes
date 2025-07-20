use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    TurnLeft,
    TurnRight,
}

/// Default mapping of keys to actions
pub fn default_input_map() -> InputMap<Action> {
    use Action::*;
    InputMap::new([
        (Forward,   KeyCode::KeyW),
        (Backward,  KeyCode::KeyS),
        (Left,      KeyCode::KeyA),
        (Right,     KeyCode::KeyD),
        (Jump,      KeyCode::Space),
        (TurnLeft,  KeyCode::KeyZ),
        (TurnRight, KeyCode::KeyX),
    ])
}
use bevy::prelude::*;
use leafwing_input_manager::prelude::{Actionlike, InputMap, VirtualDPad, GamepadStick};

// ---------------------------------------------------------------------------
// Action enum
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[derive(Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    /// Unified analog movement (keyboard + D‑Pad + left stick)
    #[actionlike(DualAxis)]
    Move,

    /// Jump via Space or South face button (A / Cross)
    Jump,
    
    #[actionlike(DualAxis)]
    Look,

    /// Mouse cursor capture
    ToggleCursor,
}

// ---------------------------------------------------------------------------
// Default bindings
// ---------------------------------------------------------------------------
#[must_use]
pub fn default_input_map() -> InputMap<Action> {
    use Action::*;
    use KeyCode::*;
    use bevy::prelude::GamepadButton as GPB;

    let mut map = InputMap::default();

    // -- Unified Move (analog & digital) --
    map.insert_dual_axis(Move, VirtualDPad::wasd())
       .insert_dual_axis(Move, VirtualDPad::new(GPB::DPadUp, GPB::DPadDown, GPB::DPadLeft, GPB::DPadRight))
       .insert_dual_axis(Move, GamepadStick::LEFT);

    // -- Jump --
    map.insert(Jump, Space)
       .insert(Jump, GPB::South);

    map.insert_dual_axis(Look, GamepadStick::RIGHT);

    map.insert(ToggleCursor, Escape);

    map
}

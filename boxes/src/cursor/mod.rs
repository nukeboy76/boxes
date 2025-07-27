use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use leafwing_input_manager::prelude::ActionState;

use crate::input::Action;
use crate::player::Player;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorState>()
            .add_systems(Startup, unlock_cursor)
            .add_systems(Update, (grab_on_click, release_on_toggle));
    }
}

#[derive(Resource, Default)]
struct CursorState {
    locked: bool,
}

fn grab_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut state: ResMut<CursorState>,
) {
    if state.locked || !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        state.locked = true;
    }
}

fn release_on_toggle(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut state: ResMut<CursorState>,
    q_input: Query<&ActionState<Action>, With<Player>>,
) {
    let input = match q_input.single() {
        Ok(i) => i,
        Err(_) => return,
    };
    if !state.locked || !input.just_pressed(&Action::ToggleCursor) {
        return;
    }
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
        state.locked = false;
    }
}

fn unlock_cursor(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

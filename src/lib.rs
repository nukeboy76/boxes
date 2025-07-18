use bevy::prelude::*;
use bevy::window::{WindowPlugin, ExitCondition};

mod net;
mod level;
mod player;

pub fn run() {
    let is_server = std::env::args().any(|a| a == "--server");

    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        exit_condition: ExitCondition::DontExit,
        ..default()
    });

    App::new()
        .add_plugins(plugins)
        .insert_resource(net::Role::from_flag(is_server))
        .add_plugins((net::NetPlugin, level::LevelPlugin, player::PlayerPlugin))
        .run();
}
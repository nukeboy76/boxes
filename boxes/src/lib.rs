use bevy::prelude::*;
use bevy::window::{WindowPlugin, ExitCondition};

mod net;
mod level;
mod player;

pub fn run() {
	let plugins = DefaultPlugins.set(WindowPlugin {
		primary_window: None,
		exit_condition: ExitCondition::DontExit,
		..default()
	});

	App::new()
		.add_plugins(plugins)
		.add_plugins((/*net::NetPlugin,*/ level::LevelPlugin, player::PlayerPlugin))
		.run();
}

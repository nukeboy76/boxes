use bevy::prelude::*;

mod net;
mod level;
mod player;

pub fn run() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins((/*net::NetPlugin,*/ level::LevelPlugin, player::PlayerPlugin))
		.run();
}

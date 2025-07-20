use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

mod net;
mod level;
mod player;
mod input;


pub fn run() {
	App::new()
		.add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<input::Action>::default())
		.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugins(RapierDebugRenderPlugin::default())
		.add_plugins((/*net::NetPlugin,*/ level::LevelPlugin, player::PlayerPlugin))
		.run();
}

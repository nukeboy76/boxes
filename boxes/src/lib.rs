use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

mod camera;
mod input;
mod level;
mod net;
mod player;


pub fn run() {
	App::new()
		.add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<input::Action>::default())
		.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugins(RapierDebugRenderPlugin::default())
		.add_plugins((
			/*net::NetPlugin,*/
			camera::CameraPlugin,
			level::LevelPlugin,
			player::PlayerPlugin,
		))
		.run();
}

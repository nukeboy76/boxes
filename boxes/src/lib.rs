use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

mod assets;
mod camera;
mod cursor;
mod input;
mod level;
mod net;
mod player;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(InputManagerPlugin::<input::Action>::default())
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            default_collider_debug: ColliderDebug::AlwaysRender,
            ..default()
        })
        .add_plugins(assets::AssetPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(cursor::CursorPlugin)
        .run();
}

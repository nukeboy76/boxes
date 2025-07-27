use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::assets::{AppState, LevelAssets};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_level);
    }
}

fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands
        .spawn((
            SceneRoot(level_assets.level.clone()),
            Transform::from_xyz(0., -5., 0.),
            GlobalTransform::default(),
            Visibility::Visible,
        ))
        .insert(AsyncSceneCollider {
            shape: Some(ComputedColliderShape::ConvexHull),
            ..default()
        })
        .insert(Friction::coefficient(0.5));

    // свет
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4., 8., 4.),
    ));
}

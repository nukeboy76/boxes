use bevy::prelude::*;
use bevy::render::mesh::PlaneMeshBuilder;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    // visual ground
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(PlaneMeshBuilder::from_size(Vec2::splat(100.0))))),
        MeshMaterial3d(mats.add(Color::srgb(0.1, 0.6, 0.1))),
    ));
    // physical ground
    commands.spawn((
        Collider::cuboid(50.0, 0.1, 50.0),
        RigidBody::Fixed,
    ));

    // свет
    commands.spawn((
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // камера
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

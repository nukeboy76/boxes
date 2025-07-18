use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};


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
    // бесконечная «земля»
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(PlaneMeshBuilder::from_size(Vec2::splat(100.0))))),
        MeshMaterial3d(mats.add(Color::srgb(0.1, 0.6, 0.1))),
    ));
    // куб-трамплин
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(mats.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 1.0, 0.0),
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

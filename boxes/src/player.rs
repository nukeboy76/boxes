use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
	pub id: u64,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	pub mesh: Mesh3d,
	pub material: MeshMaterial3d<StandardMaterial>,
	pub visibility: Visibility,
	pub transform: Transform,
	pub velocity: Velocity,
}

impl PlayerBundle {
	pub fn new(
		id: u64,
		mesh_handle: Handle<Mesh>,
		mat_handle: Handle<StandardMaterial>,
	) -> Self {
		Self {
			player: Player { id },
			mesh: Mesh3d(mesh_handle),
			material: MeshMaterial3d(mat_handle),
			visibility: Visibility::Visible,
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			velocity: Velocity::default(),
		}
	}
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, apply_velocity);
	}
}

fn apply_velocity(time: Res<Time>, mut q: Query<(&mut Transform, &mut Velocity)>) {
	for (mut t, mut v) in &mut q {
		t.translation += (v.0 * time.delta_secs());
		v.0 *= 0.9;
		if t.translation.y < 0.5 {
			t.translation.y = 0.5;
		}
	}
}

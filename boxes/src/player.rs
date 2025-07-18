use bevy::prelude::*;
use bevy::render::mesh::Mesh3d;
use bevy::math::prelude::Cuboid;

#[derive(Component)]
pub struct Player {
    pub id: u64,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct AngularVelocity(pub f32);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub visibility: Visibility,
    pub transform: Transform,
    pub velocity: Velocity,
    pub spin: AngularVelocity,
}

impl PlayerBundle {
    pub fn new(
        id: u64,
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
    ) -> Self {
        PlayerBundle {
            player: Player { id },
            mesh: Mesh3d(mesh_handle),
            material: MeshMaterial3d(material_handle),
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            velocity: Velocity::default(),
            spin: AngularVelocity::default(),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (apply_movement, apply_rotation, apply_velocity));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle: Handle<Mesh> = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mat_handle: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        ..Default::default()
    });
    commands.spawn(PlayerBundle::new(1, cube_handle, mat_handle));
}

fn apply_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    // параметры инерции
    let max_speed = 5.0;
    let accel = 10.0;      // (units per second) ^ 2
    let damping = 0.8_f32; // экспоненциальная фрикция

    for mut velocity in &mut query {
        let mut dir = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) { dir.z -= 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { dir.z += 1.0; }
        if keyboard.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) { dir.x += 1.0; }
        if keyboard.just_pressed(KeyCode::Space) { dir.y += 1.0; }
        if dir.length_squared() > 1.0 { dir = dir.normalize(); }
        let target = dir * max_speed;

        // ускоряемся к цели: v += (target - v) * accel * dt
        let dv = (target - velocity.0) * accel * time.delta_secs();
        velocity.0 += dv;

        // простое демпфирование (чтобы без удержания клавиши останавливался не сразу)
        velocity.0 *= damping.powf(time.delta_secs());
    }
}

fn apply_rotation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut AngularVelocity), With<Player>>,
    time: Res<Time>,
) {
    let rot_accel = std::f32::consts::PI * 2.0;
    let rot_damping = 0.8_f32;

    for (mut transform, mut ang_vel) in &mut query {
        let mut a = 0.0;
        if keyboard.pressed(KeyCode::KeyZ) { a += rot_accel; }
        if keyboard.pressed(KeyCode::KeyX) { a -= rot_accel; }

        // w += a * dt
        ang_vel.0 += a * time.delta_secs();

        // delta = w * dt
        let delta = ang_vel.0 * time.delta_secs();
        if delta.abs() > std::f32::EPSILON {
            transform.rotate_y(delta);
        }

        // демпфирование угл. скорости
        ang_vel.0 *= rot_damping.powf(time.delta_secs());
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
        if transform.translation.y < 0.5 {
            transform.translation.y = 0.5;
        }
    }
}
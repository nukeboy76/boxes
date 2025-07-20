use bevy::prelude::*;
use bevy::pbr::MeshMaterial3d;
use bevy::math::prelude::Cuboid;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::{Action, default_input_map};

/// Component marking a player entity
#[derive(Component)]
pub struct Player {
    pub id: u64,
}

/// Bundle grouping visuals, physics, and input for a player cube
#[derive(Bundle)]
pub struct PlayerBundle {
    // Visual
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub visibility: Visibility,
    pub transform: Transform,

    // Identity
    pub player: Player,

    // Physics
    pub body: RigidBody,
    pub collider: Collider,
    pub damping: Damping,
    pub impulse: ExternalImpulse,

    // Input
    pub input_map: InputMap<Action>,
    pub action_state: ActionState<Action>,
}

impl PlayerBundle {
    /// Create a new player cube with the given id, mesh handle, and material
    pub fn new(id: u64, mesh: Handle<Mesh>, mat: Handle<StandardMaterial>) -> Self {
        PlayerBundle {
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(mat),
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            player: Player { id },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            impulse: ExternalImpulse::default(),
            input_map: default_input_map(),
            action_state: Default::default(),
        }
    }
}

/// Plugin to spawn and drive player systems
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (apply_movement, face_camera));
    }
}

/// Spawn a single player cube
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        ..default()
    });

    commands.spawn(PlayerBundle::new(1, cube, blue));
}

/// Применяем движение относительно поворота игрока
fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&ActionState<Action>, &mut ExternalImpulse, &Transform), With<Player>>,
) {
    const IMPULSE: f32 = 20.0;
    const JUMP_FORCE: f32 = 5.0;
    const DEADZONE: f32 = 0.15;

    for (state, mut imp, transform) in query.iter_mut() {
        imp.impulse = Vec3::ZERO;

        // получаем ввод: горизонталь → x, вперед/назад → y (инвертируем y для удобства)
        let v: Vec2 = state.axis_pair(&Action::Move) * Vec2 { x: -1.0, y: 1.0 };
        if v.length_squared() > DEADZONE * DEADZONE {
            // локальное направление в плоскости XZ
            let local_dir = Vec3::new(v.x, 0.0, v.y).normalize();
            let world_dir = transform.rotation * local_dir;
            imp.impulse += world_dir * IMPULSE * time.delta_secs();
        }

        // прыжок остаётся без изменений
        if state.just_pressed(&Action::Jump) {
            imp.impulse.y += JUMP_FORCE;
        }
    }
}

fn face_camera(
    time: Res<Time>,
    q_cam: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut q_player: Query<&mut Transform, With<Player>>,
) {
    let cam_tf = if let Ok(t) = q_cam.single() { t } else { return };
    let mut player_tf = if let Ok(p) = q_player.single_mut() { p } else { return };

    // получаем горизонтальную проекцию forward-камеры
    let f = cam_tf.forward();
    let dir = Vec3::new(f.x, 0.0, f.z);
    if dir.length_squared() < 1e-4 {
        return;
    }
    let target_yaw = dir.normalize().x.atan2(dir.normalize().z);
    let target_rot = Quat::from_rotation_y(target_yaw);

    // сглаживание: интерполируем между текущим и цельным
    const SMOOTH_FACTOR: f32 = 5.0;
    player_tf.rotation = target_rot;
}

use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::{default_input_map, Action};

/// Component marking a player entity
#[derive(Component)]
pub struct Player {
    pub id: u64,
}

/// Bundle grouping visuals, physics, and input for a player capsule
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
    pub force: ExternalForce,

    // Input
    pub input_map: InputMap<Action>,
    pub action_state: ActionState<Action>,
}
impl PlayerBundle {
    pub fn new(id: u64, mesh: Handle<Mesh>, mat: Handle<StandardMaterial>) -> Self {
        let mut pl = PlayerBundle {
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(mat),
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            player: Player { id },
            body: RigidBody::Dynamic,
            collider: Collider::capsule(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 0.5, 0.0), 0.5),
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            force: ExternalForce::default(),
            input_map: default_input_map(),
            action_state: Default::default(),
        };
        pl.transform.rotate_y(45.);
        pl
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(PostUpdate, update_all.before(PhysicsSet::SyncBackend));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let capsule = meshes.add(Capsule3d::new(0.5, 1.0));
    let blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        ..default()
    });

    commands
        .spawn(PlayerBundle::new(1, capsule, blue))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Ccd::enabled())
        .insert(SoftCcd { prediction: 2. });
}

fn update_all(
    time: Res<Time>,
    q_cam: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut params: ParamSet<(
        Query<(&ActionState<Action>, &mut ExternalForce, &Transform), With<Player>>,
        Query<&mut Transform, With<Player>>,
    )>,
) {
    const FORCE_MAG: f32 = 2000.0;
    const JUMP_FORCE: f32 = 500.0;
    const DEADZONE: f32 = 0.15;

    for (state, mut ext_force, transform) in params.p0().iter_mut() {
        ext_force.force = Vec3::ZERO;

        // получаем ввод: влево/вправо → x, вперед/назад → y (инвертируем y для удобства)
        let v: Vec2 = state.axis_pair(&Action::Move) * Vec2 { x: -1.0, y: 1.0 };
        if v.length_squared() > DEADZONE * DEADZONE {
            let local_dir = Vec3::new(v.x, 0.0, v.y).normalize();
            let world_dir = transform.rotation * local_dir;
            ext_force.force += world_dir * FORCE_MAG * time.delta_secs();
        }

        if state.just_pressed(&Action::Jump) {
            ext_force.force.y += JUMP_FORCE;
        }
    }

    let cam_tf = if let Ok(t) = q_cam.single() {
        t
    } else {
        return;
    };
    for mut player_tf in params.p1().iter_mut() {
        let f = cam_tf.forward();
        let dir = Vec3::new(f.x, 0.0, f.z);
        if dir.length_squared() < 1e-4 {
            return;
        }
        let target_yaw = dir.normalize().x.atan2(dir.normalize().z);
        let target_rot = Quat::from_rotation_y(target_yaw);

        player_tf.rotation = target_rot;
    }
}

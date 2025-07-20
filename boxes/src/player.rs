use bevy::prelude::*;
use bevy::pbr::MeshMaterial3d;
use bevy::math::prelude::Cuboid;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::{Action, default_input_map};

///  Components --------------------------------------------------------------

#[derive(Component)]
pub struct Player {
    pub id: u64,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    // visual
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub visibility: Visibility,
    pub transform: Transform,

    // logic
    pub player: Player,
    
    // physics
    pub body: RigidBody,
    pub collider: Collider,
    pub damping: Damping,
    pub impulse: ExternalImpulse,

    // input
    pub input_map:    InputMap<Action>,
    pub action_state: ActionState<Action>,
}

impl PlayerBundle {
    pub fn new(id: u64, mesh: Handle<Mesh>, mat: Handle<StandardMaterial>) -> Self {
        PlayerBundle {
            player: Player { id },
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(mat),
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),

            body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            damping: Damping { linear_damping: 1.0, angular_damping: 1.0 },
            impulse: ExternalImpulse::default(),

            input_map: InputMap::new([
                (Action::Forward,  KeyCode::KeyW),
                (Action::Backward, KeyCode::KeyS),
                (Action::Left,     KeyCode::KeyA),
                (Action::Right,    KeyCode::KeyD),
                (Action::Jump,     KeyCode::Space),
                (Action::TurnLeft,  KeyCode::KeyZ),
                (Action::TurnRight, KeyCode::KeyX),
            ]),
            action_state: Default::default(),
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (apply_movement, apply_rotation));
    }
}

/// Spawning -----------------------------------------------------------------

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
    commands.spawn(PlayerBundle {
        player: Player { id: 1 },
        mesh: Mesh3d(cube),
        material: MeshMaterial3d(blue),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        visibility: Visibility::Visible,
        body: RigidBody::Dynamic,
        collider: Collider::cuboid(0.5, 0.5, 0.5),
        damping: Damping { linear_damping: 1.0, angular_damping: 1.0 },
        impulse: ExternalImpulse::default(),
        input_map: default_input_map(),
        action_state: Default::default(),
    });
}

/// Movement system ----------------------------------------------------------

fn apply_movement(
    time: Res<Time>,
    mut q: Query<(&ActionState<Action>, &mut ExternalImpulse)>,
) {
    const IMPULSE: f32 = 20.0;
    for (state, mut imp) in q.iter_mut() {
        imp.impulse = Vec3::ZERO;

        let mut dir = Vec3::ZERO;
        if state.pressed(&Action::Forward)  { dir.z -= 1.0; }
        if state.pressed(&Action::Backward) { dir.z += 1.0; }
        if state.pressed(&Action::Left)     { dir.x -= 1.0; }
        if state.pressed(&Action::Right)    { dir.x += 1.0; }
        if state.just_pressed(&Action::Jump){ imp.impulse.y += 5.0; }

        if dir != Vec3::ZERO {
            imp.impulse += dir.normalize() * IMPULSE * time.delta_secs();
        }
    }
}

/// Rotation system ----------------------------------------------------------

fn apply_rotation(
    time: Res<Time>,
    mut q: Query<(&ActionState<Action>, &mut ExternalImpulse)>,
) {
    const TORQUE: f32 = 10.0;
    for (state, mut imp) in q.iter_mut() {
        imp.torque_impulse = Vec3::ZERO;
        if state.pressed(&Action::TurnLeft)  { imp.torque_impulse.y += TORQUE * time.delta_secs(); }
        if state.pressed(&Action::TurnRight) { imp.torque_impulse.y -= TORQUE * time.delta_secs(); }
    }
}
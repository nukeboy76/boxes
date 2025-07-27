use bevy::prelude::*;
use dolly::prelude::*;
use mint::{Point3, Vector3};
use crate::player::Player;
use bevy::input::mouse::MouseMotion;
use crate::input::Action;
use leafwing_input_manager::prelude::ActionState;
use bevy::window::PrimaryWindow;

/// -------------------------------------------------------------------------
/// Плагин
/// -------------------------------------------------------------------------
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (update_rig, orbit_camera));
    }
}

/// -------------------------------------------------------------------------
/// Состояние камеры (ресурс)
/// -------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq)]
enum CamMode { Pause, Follow }

#[derive(Resource)]
struct CamState {
    mode: CamMode,
    start: Transform,
}
impl Default for CamState {
    fn default() -> Self {
        Self {
            mode: CamMode::Follow,
            start: Transform::from_xyz(-8.0, 8.0, 8.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

/// -------------------------------------------------------------------------
/// Компонент‑обёртка над `dolly::CameraRig`
/// -------------------------------------------------------------------------
#[derive(Component, Deref, DerefMut)]
#[deref(mutable)]
struct DollyRig(CameraRig);

/// -------------------------------------------------------------------------
/// Спавн камеры
/// -------------------------------------------------------------------------
fn spawn_camera(mut commands: Commands, state: Res<CamState>) {
    let rig = CameraRig::builder()
        .with(Position::new(Point3 {
            x: state.start.translation.x,
            y: state.start.translation.y,
            z: state.start.translation.z,
        }))
        .with(YawPitch::new())
        .with(Arm::new(Vector3 { x: 0.0, y: 3.0, z: -6.0 }))
        .with(LookAt::new(Point3 { x: 0.0, y: 0.0, z: 0.0 }))
        .build();

    commands.spawn((
        Camera3d::default(),
        state.start,
        DollyRig(rig),
    ));
}

/// -------------------------------------------------------------------------
/// Обновление рига и трансформа камеры
/// -------------------------------------------------------------------------
fn update_rig(
    time: Res<Time>,
    state: Res<CamState>,
    mut q_cam: Query<(&mut DollyRig, &mut Transform), Without<Player>>,
    q_player: Query<&Transform, With<Player>>,
) {
    let (mut rig_wrap, mut cam_tf) = match q_cam.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    let rig: &mut CameraRig = &mut rig_wrap;

    match state.mode {
        CamMode::Pause => {
            rig.driver_mut::<Position>().position = Point3 {
                x: state.start.translation.x,
                y: state.start.translation.y,
                z: state.start.translation.z,
            };
            rig.driver_mut::<LookAt>().target = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        }
        CamMode::Follow => {
            let player_tf = match q_player.single() {
                Ok(p) => p,
                Err(_) => return,
            };
            rig.driver_mut::<Position>().position = Point3 {
                x: player_tf.translation.x,
                y: player_tf.translation.y,
                z: player_tf.translation.z,
            };
            rig.driver_mut::<LookAt>().target = Point3 {
                x: player_tf.translation.x,
                y: player_tf.translation.y,
                z: player_tf.translation.z,
            };
        }
    }

    let view = rig.update(time.delta_secs()); // CameraView
    cam_tf.translation = Vec3::new(view.position.x, view.position.y, view.position.z);
    cam_tf.rotation    = Quat::from_xyzw(
        view.rotation.v.x,
        view.rotation.v.y,
        view.rotation.v.z,
        view.rotation.s,
    );
}

fn orbit_camera(
    time: Res<Time>,
    mut rig_q: Query<&mut DollyRig>,
    mut mouse: EventReader<MouseMotion>,
    player_input: Query<&ActionState<Action>, With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.single() {
        if window.cursor_options.visible { return; }
    }

    let mut rig = match rig_q.single_mut() { Ok(r) => r, Err(_) => return };
    let yp     = rig.driver_mut::<YawPitch>();

    let delta = mouse.read().fold(Vec2::ZERO, |d, e| d + e.delta);
    yp.rotate_yaw_pitch(-0.15 * delta.x, 0.15 * delta.y);

    const DEADZONE: f32 = 0.15;
    if let Ok(state) = player_input.single() {
        let raw = state.axis_pair(&Action::Look);
        let v = Vec2::new(
            if raw.x.abs() > DEADZONE { raw.x } else { 0.0 },
            if raw.y.abs() > DEADZONE { raw.y } else { 0.0 },
        );
        if v != Vec2::ZERO {
            yp.rotate_yaw_pitch(
                -v.x * 90.0 * time.delta_secs(),
                -v.y * 360.0 * time.delta_secs(),
            );
        }
    }

    const MIN_PITCH: f32 = -1200.0;
    const MAX_PITCH: f32 =  60.0;
    yp.pitch_degrees = yp.pitch_degrees.clamp(MIN_PITCH, MAX_PITCH);
 }
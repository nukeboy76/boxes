use bevy::prelude::*;
use bevy_renet::{
    netcode::{
        ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport,
        NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication,
        ServerConfig,
    },
    renet::{
        ChannelConfig, ClientId, ConnectionConfig, DefaultChannel, RenetClient,
        RenetServer, SendType, ServerEvent,
    },
    RenetClientPlugin, RenetServerPlugin,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::UdpSocket,
    time::{Duration, SystemTime},
};

const PORT: u16 = 5000;
const PROTOCOL_ID: u64 = 7;

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Server,
    Client,
}
impl Role {
    pub fn from_flag(server: bool) -> Self {
        if server { Self::Server } else { Self::Client }
    }
}

#[derive(Resource, Default)]
struct Lobby {
    players: HashMap<ClientId, Entity>,
}

#[derive(Component)]
struct PlayerId(ClientId);

#[derive(Serialize, Deserialize)]
struct InputMsg {
    dir: Vec2,
    jump: bool,
}

pub struct NetPlugin;
impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        let role = *app.world().resource::<Role>();
        if role == Role::Server {
            server_setup(app);
        } else {
            client_setup(app);
        }
    }
}

fn channels() -> ConnectionConfig {
    let reliable = ChannelConfig {
        channel_id: 0,
        max_memory_usage_bytes: 1 << 20,
        send_type: SendType::ReliableOrdered { resend_time: Duration::from_millis(200) },
    };
    ConnectionConfig {
        server_channels_config: vec![reliable.clone()],
        client_channels_config: vec![reliable],
        ..default()
    }
}
fn addr() -> String { format!("127.0.0.1:{PORT}") }


fn server_setup(app: &mut App) {
    let socket = UdpSocket::bind(addr()).unwrap();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let cfg = ServerConfig {
        current_time: now,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![addr().parse().unwrap()],
        authentication: ServerAuthentication::Unsecure,
    };
    let server = RenetServer::new(channels());

    app.insert_resource(server)
        .insert_resource(NetcodeServerTransport::new(cfg, socket).unwrap())
        .insert_resource(Lobby::default())
        .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
        .add_systems(
            Update,
            (sv_events, sv_recv_inputs, sv_broadcast_transforms),
        );
}

fn sv_events(
    mut ev: EventReader<ServerEvent>,
    _events: Res<Events<ServerEvent>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut lobby: ResMut<Lobby>,
) {
    for event in ev.read() {
        if let ServerEvent::ClientConnected { client_id } = event {
            let cid: ClientId = *client_id;
            let e = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
                    MeshMaterial3d(mats.add(Color::srgb(0.8, 0.7, 0.6))),
                    Transform::from_xyz(0.0, 0.5, 0.0),
                    crate::player::Velocity(Vec3::ZERO),
                    PlayerId(cid),
                ))
                .id();
            lobby.players.insert(cid, e);
        }
    }
}

fn sv_recv_inputs(
    mut server: ResMut<RenetServer>,
    mut q: Query<(&mut crate::player::Velocity, &PlayerId)>,
) {
    for cid in server.clients_id() {
        while let Some(raw) = server.receive_message(cid, DefaultChannel::ReliableOrdered) {
            if let Ok(inp) = bincode::deserialize::<InputMsg>(&raw) {
                for (mut vel, pid) in &mut q {
                    if pid.0 == cid {
                        vel.0 = inp.dir.extend(0.0);
                        if inp.jump && vel.0.z.abs() < 0.01 {
                            vel.0.z = 5.0;
                        }
                    }
                }
            }
        }
    }
}

fn sv_broadcast_transforms(
    mut server: ResMut<RenetServer>,
    q: Query<(&Transform, &PlayerId)>,
) {
    let map: HashMap<ClientId, Vec3> =
        q.iter().map(|(t, id)| (id.0, t.translation)).collect();
    server.broadcast_message(
        DefaultChannel::Unreliable,
        bincode::serialize(&map).unwrap(),
    );
}


fn client_setup(app: &mut App) {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let auth = ClientAuthentication::Unsecure {
        client_id: now.as_millis() as u64,
        server_addr: addr().parse().unwrap(),
        protocol_id: PROTOCOL_ID,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(now, auth, socket).unwrap();
    let client = RenetClient::new(channels());

    app.insert_resource(client)
        .insert_resource(transport)
        .insert_resource(Lobby::default())
        .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
        .add_systems(Update, (cl_send_input, cl_apply_transforms));
}

fn cl_send_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut client: ResMut<RenetClient>,
) {
    if !client.is_connected() { return; }
    let dir = Vec2::new(
        keys.pressed(KeyCode::KeyD) as i8 as f32 - keys.pressed(KeyCode::KeyA) as i8 as f32,
        keys.pressed(KeyCode::KeyW) as i8 as f32 - keys.pressed(KeyCode::KeyS) as i8 as f32,
    );
    let jump = keys.just_pressed(KeyCode::Space);
    client.send_message(
        DefaultChannel::ReliableOrdered,
        bincode::serialize(&InputMsg { dir, jump }).unwrap(),
    );
}

fn cl_apply_transforms(
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    while let Some(raw) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(map) = bincode::deserialize::<HashMap<ClientId, Vec3>>(&raw) {
            for (id, pos) in map {
                let ent = *lobby.players.entry(id).or_insert_with(|| {
                    commands
                        .spawn((
                            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
                            MeshMaterial3d(mats.add(Color::srgb(0.2, 0.8, 1.0))),
                            Transform::default(),
                            PlayerId(id),
                        ))
                        .id()
                });
                commands.entity(ent).insert(Transform::from_translation(pos));
            }
        }
    }
}

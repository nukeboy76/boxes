use spacetimedb::{reducer, table, Identity, ReducerContext, ScheduleAt, SpacetimeType, Table};

pub type Id = u64;
pub type Unit = f64;
pub type Angle = f64;

#[derive(SpacetimeType, Default)]
pub struct Position<T = Unit> {
    x: T,
    y: T,
    z: T,
}

impl std::ops::Add<Velocity> for Position {
    type Output = Self;

    fn add(mut self, rhs: Velocity) -> Position {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.dx;
        self.y += rhs.dy;
        self.z += rhs.dz;
    }
}

#[derive(SpacetimeType, Default)]
pub struct Velocity<T = Unit> {
    dx: T,
    dy: T,
    dz: T,
}

#[derive(SpacetimeType, Clone, Default)]
pub struct Orientation {
    yaw: Angle,
    pitch: Angle,
    roll: Angle,
}

impl From<[Angle; 3]> for Orientation {
    fn from([yaw, pitch, roll]: [Angle; 3]) -> Self {
        Self { yaw, pitch, roll }
    }
}

impl From<Orientation> for [Angle; 3] {
    fn from(Orientation { yaw, pitch, roll }: Orientation) -> Self {
        [yaw, pitch, roll]
    }
}

impl std::ops::Add<Spin> for Orientation {
    type Output = Self;

    fn add(mut self, rhs: Spin) -> Orientation {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Spin> for Orientation {
    fn add_assign(&mut self, rhs: Spin) {
        *self = quaternion::rotate_vector((rhs.a, rhs.look.into()), self.clone().into()).into();
    }
}

#[derive(SpacetimeType, Default)]
pub struct Spin {
    a: Angle,
    look: Orientation,
}

#[table(name=entity, public, scheduled(update_entity))]
struct Entity {
    #[primary_key]
    #[auto_inc]
    id: Id,

    pos: Position,
    look: Orientation,
    vel: Velocity,
    spin: Spin,

    scheduled_at: ScheduleAt,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: 0,
            pos: Position::default(),
            look: Orientation::default(),
            vel: Velocity::default(),
            spin: Spin::default(),
            scheduled_at: ScheduleAt::Interval(std::time::Duration::from_millis(1000 / 125).into()),
        }
    }
}

#[reducer]
fn update_entity(_ctx: &ReducerContext, mut entity: Entity) -> Result<(), String> {
    entity.pos += entity.vel;
    entity.look += entity.spin;

    Ok(())
}

#[table(name=player, public)]
struct Player {
    #[primary_key]
    id: Identity,

    #[unique]
    name: String,

    #[unique]
    entity_id: Id,
}

#[reducer(init)]
fn init(_ctx: &ReducerContext) {}

#[reducer(client_connected)]
fn client_connected(_ctx: &ReducerContext) {}

#[reducer(client_disconnected)]
fn client_disconnected(_ctx: &ReducerContext) {}

#[reducer]
fn join(ctx: &ReducerContext, name: String) {
    let entities = ctx.db.entity();

    let entity = entities.insert(Entity::default());

    let players = ctx.db.player();

    players.insert(Player {
        id: ctx.sender,
        name,
        entity_id: entity.id,
    });
}

#[reducer]
fn player_move(ctx: &ReducerContext, vel: Velocity, spin: Spin) -> Result<(), String> {
    let players = ctx.db.player();

    let player = players.id().find(ctx.sender).unwrap();

    let entities = ctx.db.entity();

    let mut entity = entities.id().find(player.entity_id).unwrap();

    entity.vel = vel;
    entity.spin = spin;

    update_entity(ctx, entity)
}

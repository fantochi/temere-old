use actix::prelude::{Actor, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};

pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new() -> std::io::Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<Conn>::new(database_url.into());
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(Self(pool))
    }
}
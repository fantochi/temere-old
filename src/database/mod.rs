pub mod lobby;

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

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Database instance started.")
    }
}

impl DbExecutor {
    pub fn new(database_pool: PgPool) -> Self {
        Self(database_pool)
    }
}

pub fn new_pool() -> Result<PgPool, ()> {
    info!("Postgres pool has been started");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<Conn>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager);

    match pool {
        Ok(p) =>  Ok(p),
        Err(e) => Err(())       
    }
}
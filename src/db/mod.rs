pub mod oauth;
pub mod user;

use actix::prelude::{Actor, SyncContext};
use diesel::{

    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager, Pool},
};

pub type Conn = MysqlConnection;
pub type MySqlPool = Pool<ConnectionManager<Conn>>;

pub struct DbExecutor(pub MySqlPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Pool<ConnectionManager<Conn>> {
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}

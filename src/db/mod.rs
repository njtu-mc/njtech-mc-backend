pub mod oauth;
pub mod user;

use std::sync::Mutex;
use actix::prelude::{Actor, SyncContext};
use std::env;
use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager, Pool},
};
use redis::{Connection};
use lazy_static::lazy_static;

pub type Conn = MysqlConnection;
pub type MySqlPool = Pool<ConnectionManager<Conn>>;

pub struct DbExecutor(pub MySqlPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

lazy_static! {
    pub static ref REDIS_CONN: Mutex<Connection> = {
        let redis_pass = env::var("REDIS_PASS").expect("REDIS_PASS must be set");
        let coninfo = redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp("127.0.0.1".to_string(), 6379),
            redis: redis::RedisConnectionInfo {
                db: 1,
                password: Some(redis_pass),
                username: None,
            },
        };

        let conn = redis::Client::open(coninfo).unwrap().get_connection().unwrap();
        Mutex::new(conn)
    };
}

pub fn new_redis() -> () {
    lazy_static::initialize(&REDIS_CONN)
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Pool<ConnectionManager<Conn>> {
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}

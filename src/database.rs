//! Database-related functions
use crate::config::CONFIG;
use diesel::{
  mysql::MysqlConnection,
  r2d2::{ConnectionManager, PoolError},
};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Result<Pool, PoolError> {
  let manager = ConnectionManager::<MysqlConnection>::new(&CONFIG.database_url);
  Pool::builder().build(manager)
}

use diesel::{pg::PgConnection, r2d2::ConnectionManager};

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresPool;

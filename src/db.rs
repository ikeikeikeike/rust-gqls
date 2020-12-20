use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn get_db_pool() -> Pool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);

    r2d2::Pool::new(manager).expect("Failed to create DB Pool")
}

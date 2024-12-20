mod schema;

pub mod channel;
pub mod comment;
pub mod post;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_db_pool() -> DBPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.")
}

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;


pub mod schema {
    use diesel::table;

    table! {
        rss_items {
            id -> Integer,
            title -> Text,
            link -> Text,
        }
    }
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("POSTGRES_USER").unwrap(),
        env::var("POSTGRES_PASSWORD").unwrap(),
        env::var("POSTGRES_HOST").unwrap(),
        env::var("POSTGRES_PORT").unwrap(),
        env::var("POSTGRES_DATABASE").unwrap(),
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

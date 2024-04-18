#[macro_use]
extern crate diesel;

use actix_rt::System;
use std::time::Duration;
use tokio::time;

mod db;
mod rss;
mod schema;

#[actix_rt::main]
async fn main() {
    dotenv::dotenv().ok();

    // Setup database connection pool
    let pool = db::establish_connection();

    // Run the background task to continuously fetch and store RSS feed
    System::new().block_on(async move {
        loop {
            if let Err(err) = rss::fetch_and_store_rss(&pool).await {
                eprintln!("Error: {}", err);
            }
            // Fetch RSS feed every hour
            time::sleep(Duration::from_secs(3600)).await;
        }
    });
}

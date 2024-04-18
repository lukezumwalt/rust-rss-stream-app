use crate::db;
use crate::schema::{rss_items, NewRssItem};
use crate::diesel::RunQueryDsl;
use rss;
use reqwest;

pub async fn fetch_and_store_rss(pool: &db::Pool) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://startingstrength.com/index/rss").await?.text().await?;
    let channel = rss::Channel::read_from(response.as_bytes())?;

    let conn = pool.get().expect("couldn't get db connection from pool");

    for item in channel.items {
        let new_item = NewRssItem {
            title: item.title.unwrap_or_else(|| "".to_string()),
            link: item.link.unwrap_or_else(|| "".to_string()),
        };
        diesel::insert_into(rss_items::table)
            .values(&new_item)
            .execute(&conn)?;
    }
    Ok(())
}

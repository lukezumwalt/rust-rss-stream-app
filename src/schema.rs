table! {
    rss_items (id) {
        id -> Int4,
        title -> Text,
        link -> Text,
    }
}

#[derive(Insertable)]
#[table_name = "rss_items"]
pub struct NewRssItem {
    pub title: String,
    pub link: String,
}

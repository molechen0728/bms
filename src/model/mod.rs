pub(crate) mod book;
pub(crate) mod book_class;
pub(crate) mod lend_record;
pub(crate) mod reader;
pub(crate) mod usr;

use chrono::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Page {
    pub(crate) page_no: i32,
    pub(crate) page_size: i32,
}

pub async fn now() -> i64 {
    rocket::tokio::spawn(async {
        let utc: DateTime<Utc> = Utc::now();
        utc.timestamp_millis()
    })
    .await
    .unwrap()
}

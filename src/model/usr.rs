use crate::db;
use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Usr {
    pub(crate) uid: Option<i32>,
    pub(crate) uname: Option<String>,
    pub(crate) upass: Option<String>,
}

pub(crate) struct Table<'a> {
    pub(crate) db: &'a mut Connection<db::Conn>,
}

impl<'a, 'b: 'a> Table<'a> {
    pub fn new(conn: &'b mut Connection<db::Conn>) -> Table<'a> {
        Table { db: conn }
    }

    pub async fn find_with_id_and_password(&mut self, a: Usr) -> Result<Option<Usr>, sqlx::Error> {
        sqlx::query_as!(
            Usr,
            r#"
            SELECT 
                uid AS "uid?", 
                uname AS "uname?",
                '**********' AS "upass?"
            FROM 
                usr 
            WHERE 
                uname = $1 
            AND upass = $2"#,
            &a.uname.unwrap(),
            &a.upass.unwrap()
        )
        .fetch_optional(&mut **self.db)
        .await
    }

    pub async fn find_with_id(&mut self, id: i32) -> Result<Option<Usr>, sqlx::Error> {
        sqlx::query_as!(
            Usr,
            r#"
            SELECT 
                uid AS "uid?", 
                uname AS "uname?",
                '**********' AS "upass?"
            FROM 
                usr 
            WHERE 
                uid = $1 "#,
            &id,
        )
        .fetch_optional(&mut **self.db)
        .await
    }

    pub async fn edit_password(&mut self, a: Usr) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(r#"UPDATE usr SET upass = $1 WHERE uid = $2 "#)
            .bind(&a.upass)
            .bind(&a.uid)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }
}

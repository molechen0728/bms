use crate::db;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;
use sqlx::FromRow;

use super::now;

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(crate = "rocket::serde")]
pub(crate) struct LendRecord {
    pub(crate) rid: Option<i32>,
    pub(crate) book_id: Option<i32>,
    pub(crate) reader_id: Option<i32>,
    pub(crate) lend_date: Option<i64>,
    pub(crate) back_date: Option<i64>,
}

pub(crate) struct Table<'a> {
    pub(crate) db: &'a mut Connection<db::Conn>,
}

impl<'a, 'b: 'a> Table<'a> {
    pub fn new(conn: &'b mut Connection<db::Conn>) -> Table<'a> {
        Table { db: conn }
    }

    pub async fn list(&mut self, off_set: i32, size: i32) -> Result<Vec<LendRecord>, sqlx::Error> {
        sqlx::query_as!(
            LendRecord,
            r#"
            SELECT
                rid AS "rid?",
                book_id AS "book_id?",
                reader_id AS "reader_id?",
                lend_date AS "lend_date?",
                back_date AS "back_date?"
            FROM 
                lend_record 
            ORDER BY rid ASC
            LIMIT $1 OFFSET $2"#,
            size as i64,
            off_set as i64
        )
        .fetch_all(&mut **self.db)
        .await
    }

    pub async fn count(&mut self) -> Result<i32, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT
                count(1) AS "count!:i64"
            FROM
                lend_record
            "#,
        )
        .fetch_one(&mut **self.db)
        .await?
        .count as i32)
    }

    pub async fn find_by_id(&mut self, id: i32) -> Result<Option<LendRecord>, sqlx::Error> {
        sqlx::query_as!(
            LendRecord,
            r#"
            SELECT
                rid AS "rid?",
                book_id AS "book_id?",
                reader_id AS "reader_id?",
                lend_date AS "lend_date?",
                back_date AS "back_date?"
            FROM
                lend_record
            WHERE
                rid = $1
            "#,
            &id,
        )
        .fetch_optional(&mut **self.db)
        .await
    }

    pub async fn add(&mut self, bc: LendRecord) -> Result<u64, sqlx::Error> {
        let ts = now().await;
        Ok(
            sqlx::query(r#"INSERT INTO lend_record(book_id,reader_id,lend_date) VALUES ($1,$2,$3)"#)
                .bind(&bc.book_id)
                .bind(&bc.reader_id)
                .bind(&ts)
                .execute(&mut **self.db)
                .await?
                .rows_affected(),
        )
    }

    pub async fn remove_by_id(&mut self, id: i32) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(r#"DELETE FROM lend_record WHERE rid = $1 "#)
            .bind(&id)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }

    pub async fn edit(&mut self, bc: LendRecord) -> Result<u64, sqlx::Error> {
        let ts = now().await;
        Ok(sqlx::query(r#"UPDATE lend_record SET back_date=$1 WHERE rid = $2 "#)
            .bind(&ts)
            .bind(&bc.rid)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }
}

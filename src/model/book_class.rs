use crate::db;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct BookClass {
    pub(crate) cid: Option<i32>,
    pub(crate) cname: Option<String>,
}

pub(crate) struct Table<'a> {
    pub(crate) db: &'a mut Connection<db::Conn>,
}

impl<'a, 'b: 'a> Table<'a> {
    pub fn new(conn: &'b mut Connection<db::Conn>) -> Table<'a> {
        Table { db: conn }
    }

    pub async fn list(&mut self, off_set: i32, size: i32) -> Result<Vec<BookClass>, sqlx::Error> {
        sqlx::query_as!(
            BookClass,
            r#"
            SELECT 
                cid AS "cid?", 
                cname AS "cname?"
            FROM 
                book_class 
            ORDER BY cid ASC
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
            book_class
            "#,
        )
        .fetch_one(&mut **self.db)
        .await?
        .count as i32)
    }

    pub async fn find_by_id(&mut self, id: i32) -> Result<Option<BookClass>, sqlx::Error> {
        sqlx::query_as!(
            BookClass,
            r#"
            SELECT
                cid AS "cid?", 
                cname AS "cname?"
            FROM
                book_class
            WHERE
                cid = $1
            "#,
            &id,
        )
        .fetch_optional(&mut **self.db)
        .await
    }

    pub async fn add(&mut self, bc: BookClass) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(r#"INSERT INTO book_class(cname) VALUES ($1)"#)
            .bind(&bc.cname)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }

    pub async fn remove_by_id(&mut self, id: i32) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(r#"DELETE FROM book_class WHERE cid = $1 "#)
            .bind(&id)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }

    pub async fn edit(&mut self, bc: BookClass) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(
            r#"
                UPDATE book_class SET cname=$1 WHERE cid = $2 "#,
        )
        .bind(&bc.cname)
        .bind(&bc.cid)
        .execute(&mut **self.db)
        .await?
        .rows_affected())
    }
}

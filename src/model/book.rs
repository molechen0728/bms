use crate::db;
use crate::model::now;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;
use sqlx::FromRow;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug, FromRow, Validate)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    bid: Option<i32>,
    #[validate(required, custom = "validate_unique_username")]
    bname: Option<String>,
    author: Option<String>,
    publish: Option<String>,
    isbn: Option<String>,
    introduction: Option<String>,
    language: Option<String>,
    price: Option<f32>,
    pub_time: Option<i64>,
    class_id: Option<i32>,
    pressmark: Option<i32>,
    #[validate(range(min = 0, max = 1))]
    state: Option<i16>,
}

fn validate_unique_username(_username: &str) -> Result<(), ValidationError> {
    Ok(())
}

pub(crate) struct Table<'a> {
    pub(crate) db: &'a mut Connection<db::Conn>,
}

impl<'a, 'b: 'a> Table<'a> {
    pub fn new(conn: &'b mut Connection<db::Conn>) -> Table<'a> {
        Table { db: conn }
    }

    pub async fn find_with_page(&mut self, off_set: i32, size: i32) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            r#"
            SELECT
                bid AS "bid?:i32",
                bname AS "bname?",
                author AS "author?",
                publish AS "publish?",
                isbn AS "isbn?",
                introduction AS "introduction?",
                language AS "language?",
                price AS "price?:f32",
                pub_time AS "pub_time?",
                class_id AS "class_id?:i32",
                pressmark AS "pressmark?:i32",
                state AS "state?:i16"
            FROM
                book
            ORDER BY bid ASC
            LIMIT
                $1
            OFFSET
                $2
            "#,
            size as i64,
            off_set as i64
        )
        .fetch_all(&mut **self.db)
        .await
    }

    pub async fn count_with_page(&mut self) -> Result<i32, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT
                count(1) AS "count!:i64"
            FROM
                book
            "#,
        )
        .fetch_one(&mut **self.db)
        .await?
        .count as i32)
    }

    pub async fn find_by_id_or_name(
        &mut self,
        id: i32,
        name: String,
        off_set: i32,
        size: i32,
    ) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            r#"
            SELECT 
                bid AS "bid?:i32",
                bname AS "bname?",
                author AS "author?",
                publish AS "publish?",
                isbn AS "isbn?",
                introduction AS "introduction?",
                language AS "language?",
                price AS "price?:f32",
                pub_time AS "pub_time?",
                class_id AS "class_id?:i32",
                pressmark AS "pressmark?:i32",
                state AS "state?:i16"
            FROM 
                book 
            WHERE
                bid = $1
            OR
                bname like $2
            ORDER BY bid ASC   
            LIMIT
                $3
            OFFSET
                $4
            "#,
            &id,
            format!("%{}%", &name),
            size as i64,
            off_set as i64
        )
        .fetch_all(&mut **self.db)
        .await
    }

    pub async fn count_by_id_or_name(&mut self, id: i32, name: String) -> Result<i32, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
        SELECT 
            count(*) AS "count!:i64"
        FROM 
            book 
        WHERE
            bid = $1
        OR
            bname like $2
        "#,
            &id,
            format!("%{}%", &name),
        )
        .fetch_one(&mut **self.db)
        .await?
        .count as i32)
    }

    pub async fn remove_by_id(&mut self, id: i32) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(r#"DELETE FROM book WHERE bid = $1 "#)
            .bind(&id)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }

    pub async fn add(&mut self, mut b: Book) -> Result<u64, sqlx::Error> {
        b.pub_time = Some(now().await);
        Ok(sqlx::query(
            r#"
            INSERT INTO book(bname, author, publish, isbn, introduction, language, price, pub_time, class_id, pressmark, state)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);"#,
        )
        .bind(&b.bname)
        .bind(&b.author)
        .bind(&b.publish)
        .bind(&b.isbn)
        .bind(&b.introduction)
        .bind(&b.language)
        .bind(&b.price)
        .bind(&b.pub_time)
        .bind(&b.class_id)
        .bind(&b.pressmark)
        .bind(&b.state)
        .execute(&mut **self.db)
        .await?
        .rows_affected())
    }

    pub async fn find_by_id(&mut self, id: i32) -> Result<Option<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            r#"
            SELECT
                bid AS "bid?:i32",
                bname AS "bname?",
                author AS "author?",
                publish AS "publish?",
                isbn AS "isbn?",
                introduction AS "introduction?",
                language AS "language?",
                price AS "price?:f32",
                pub_time AS "pub_time?",
                class_id AS "class_id?:i32",
                pressmark AS "pressmark?:i32",
                state AS "state?:i16"
            FROM
                book
            WHERE
                bid = $1
            "#,
            &id,
        )
        .fetch_optional(&mut **self.db)
        .await
    }

    pub async fn edit(&mut self, mut b: Book) -> Result<u64, sqlx::Error> {
        b.pub_time = Some(now().await);
        Ok(sqlx::query(
            r#"
                UPDATE book
                SET bname=$1, author=$2, publish=$3, isbn=$4, introduction=$5, language=$6, price=$7, class_id=$8, pressmark=$9, state=$10, pub_time=$11
                WHERE bid = $12 "#,
        )
        .bind(&b.bname)
        .bind(&b.author)
        .bind(&b.publish)
        .bind(&b.isbn)
        .bind(&b.introduction)
        .bind(&b.language)
        .bind(&b.price)
        .bind(&b.class_id)
        .bind(&b.pressmark)
        .bind(&b.state)
        .bind(&b.pub_time)
        .bind(&b.bid)
        .execute(&mut **self.db)
        .await?
        .rows_affected())
    }
}

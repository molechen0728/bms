use crate::db;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;
use sqlx::FromRow;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, FromRow, Validate)]
#[serde(crate = "rocket::serde")]
pub struct Reader {
    rid: Option<i32>,
    rname: Option<String>,
    rpass: Option<String>,
    #[validate(range(min = 0, max = 1))]
    state: Option<i16>,
    gender: Option<String>,
    birth: Option<i64>,
    address: Option<String>,
    phone: Option<String>,
}

pub(crate) struct Table<'a> {
    pub(crate) db: &'a mut Connection<db::Conn>,
}

impl<'a, 'b: 'a> Table<'a> {
    pub fn new(conn: &'b mut Connection<db::Conn>) -> Table<'a> {
        Table { db: conn }
    }

    pub async fn list(&mut self, off_set: i32, size: i32) -> Result<Vec<Reader>, sqlx::Error> {
        sqlx::query_as!(
            Reader,
            r#"
            SELECT
                rid AS "rid?",
                rname AS "rname?",
                rpass AS "rpass?",
                state AS "state?",
                gender AS "gender?",
                birth AS "birth?",
                address AS "address?",
                phone AS "phone?"
            FROM
                reader
            ORDER BY rid ASC
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

    pub async fn count(&mut self) -> Result<i32, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT
                count(1) AS "count!:i64"
            FROM
                reader
            "#,
        )
        .fetch_one(&mut **self.db)
        .await?
        .count as i32)
    }

    pub async fn remove(&mut self, id: i32) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(r#"DELETE FROM reader WHERE rid = $1 "#)
            .bind(&id)
            .execute(&mut **self.db)
            .await?
            .rows_affected())
    }

    pub async fn add(&mut self, b: Reader) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(
            r#"
            INSERT INTO reader(rname, rpass, state, gender, birth, address, phone)
            VALUES ($1, $2, $3, $4, $5, $6, $7);"#,
        )
        .bind(&b.rname)
        .bind(&b.rpass)
        .bind(&b.state)
        .bind(&b.gender)
        .bind(&b.birth)
        .bind(&b.address)
        .bind(&b.phone)
        .execute(&mut **self.db)
        .await?
        .rows_affected())
    }

    pub async fn find(&mut self, id: i32) -> Result<Option<Reader>, sqlx::Error> {
        sqlx::query_as!(
            Reader,
            r#"
            SELECT
                rid AS "rid?",
                rname AS "rname?",
                rpass AS "rpass?",
                state AS "state?",
                gender AS "gender?",
                birth AS "birth?",
                address AS "address?",
                phone AS "phone?"
            FROM
                reader
            WHERE
                rid = $1
            "#,
            &id,
        )
        .fetch_optional(&mut **self.db)
        .await
    }

    pub async fn edit(&mut self, b: Reader) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(
            r#"
                UPDATE reader
                SET rname = $1, rpass = $2, state = $3, gender = $4, birth = $5, address = $6, phone = $7
                WHERE rid = $8 "#,
        )
        .bind(&b.rname)
        .bind(&b.rpass)
        .bind(&b.state)
        .bind(&b.gender)
        .bind(&b.birth)
        .bind(&b.address)
        .bind(&b.phone)
        .bind(&b.rid)
        .execute(&mut **self.db)
        .await?
        .rows_affected())
    }
}

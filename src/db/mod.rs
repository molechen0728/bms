use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlx_postgres")]
pub(crate) struct Conn(sqlx::PgPool);

#[derive(Database)]
#[database("sqlx_postgres")]
pub(crate) struct TestConn(sqlx::PgPool);

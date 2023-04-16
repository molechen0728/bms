use crate::db;
use crate::model::usr::Usr;
use crate::service::{self, Resp};
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::{get, info, post, State};
use rocket_db_pools::Connection;

use super::User;

#[post("/login", data = "<a>")]
pub(crate) async fn login(db: Connection<db::Conn>, a: Form<Usr>, up_time: &State<i64>) -> Resp<String> {
    info!("{:?}", a);
    service::usr::login(db, a.into_inner(), *up_time.inner()).await
}

#[post("/usr/reset-password", data = "<a>")]
pub(crate) async fn reset_password(db: Connection<db::Conn>, a: Json<Usr>, _u: User) -> Resp<u64> {
    info!("{:?}", a);
    service::usr::edit_password(db, a.into_inner()).await
}

#[get("/usr/<id>")]
pub(crate) async fn find(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<Usr> {
    info!("{:?}", id);
    service::usr::find_with_id(db, id).await
}

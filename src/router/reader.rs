use crate::db;
use crate::model::reader::Reader;
use crate::model::Page;
use crate::router::User;
use crate::service::{self, PageWrap, Resp};
use rocket::serde::json::Json;
use rocket::{get, info, post};
use rocket_db_pools::Connection;

#[post("/reader/list", data = "<p>")]
pub(crate) async fn list(db: Connection<db::Conn>, p: Json<Page>, _u: User) -> Resp<PageWrap<Vec<Reader>>> {
    info!("{:?}", p);
    let p = p.into_inner();
    service::reader::list(db, p.page_no, p.page_size).await
}

#[post("/reader/add", data = "<b>")]
pub(crate) async fn add(db: Connection<db::Conn>, b: Json<Reader>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::reader::add(db, b.into_inner()).await
}

#[get("/reader/remove/<id>")]
pub(crate) async fn remove(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<u64> {
    info!("{:?}", id);
    service::reader::remove(db, id).await
}

#[get("/reader/find-by-id/<id>")]
pub(crate) async fn find_by_id(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<Option<Reader>> {
    info!("{:?}", id);
    service::reader::find_by_id(db, id).await
}

#[post("/reader/edit", data = "<b>")]
pub(crate) async fn edit(db: Connection<db::Conn>, b: Json<Reader>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::reader::edit(db, b.into_inner()).await
}

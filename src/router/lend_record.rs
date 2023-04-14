use crate::db;
use crate::model::lend_record::LendRecord;
use crate::model::Page;
use crate::router::User;
use crate::service::{self, PageWrap, Resp};
use rocket::serde::json::Json;
use rocket::{get, info, post};
use rocket_db_pools::Connection;

#[post("/lend-record/list", data = "<p>")]
pub(crate) async fn list(db: Connection<db::Conn>, p: Json<Page>, _u: User) -> Resp<PageWrap<Vec<LendRecord>>> {
    info!("{:?}", p);
    let p = p.into_inner();
    service::lend_record::list(db, p.page_no, p.page_size).await
}

#[post("/lend-record/add", data = "<b>")]
pub(crate) async fn add(db: Connection<db::Conn>, b: Json<LendRecord>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::lend_record::add(db, b.into_inner()).await
}

#[get("/lend-record/remove/<id>")]
pub(crate) async fn remove(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<u64> {
    info!("{:?}", id);
    service::lend_record::remove(db, id).await
}

#[get("/lend-record/find-by-id/<id>")]
pub(crate) async fn find_by_id(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<Option<LendRecord>> {
    info!("{:?}", id);
    service::lend_record::find_by_id(db, id).await
}

#[post("/lend-record/edit", data = "<b>")]
pub(crate) async fn edit(db: Connection<db::Conn>, b: Json<LendRecord>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::lend_record::edit(db, b.into_inner()).await
}

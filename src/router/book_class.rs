use crate::db;
use crate::model::book_class::BookClass;
use crate::model::Page;
use crate::router::User;
use crate::service::{self, PageWrap, Resp};
use rocket::serde::json::Json;
use rocket::{get, info, post};
use rocket_db_pools::Connection;

#[post("/book-class/list", data = "<p>")]
pub(crate) async fn list(db: Connection<db::Conn>, p: Json<Page>, _u: User) -> Resp<PageWrap<Vec<BookClass>>> {
    info!("{:?}", p);
    let p = p.into_inner();
    service::book_class::list(db, p.page_no, p.page_size).await
}

#[post("/book-class/add", data = "<b>")]
pub(crate) async fn add(db: Connection<db::Conn>, b: Json<BookClass>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::book_class::add(db, b.into_inner()).await
}

#[get("/book-class/remove/<id>")]
pub(crate) async fn remove(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<u64> {
    info!("{:?}", id);
    service::book_class::remove(db, id).await
}

#[get("/book-class/find-by-id/<id>")]
pub(crate) async fn find_by_id(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<Option<BookClass>> {
    info!("{:?}", id);
    service::book_class::find_by_id(db, id).await
}

#[post("/book-class/edit", data = "<b>")]
pub(crate) async fn edit(db: Connection<db::Conn>, b: Json<BookClass>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::book_class::edit(db, b.into_inner()).await
}

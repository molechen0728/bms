use crate::db;
use crate::model::book::Book;
use crate::model::Page;
use crate::router::User;
use crate::service::{self, PageWrap, Resp};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, info, post};
use rocket_db_pools::Connection;
use validator::{Validate, ValidateArgs};

#[post("/book/find", data = "<p>")]
pub(crate) async fn find_with_page(db: Connection<db::Conn>, p: Json<Page>, _u: User) -> Resp<PageWrap<Vec<Book>>> {
    info!("{:?}", p);
    let p = p.into_inner();
    service::book::find_with_page(db, p.page_no, p.page_size).await
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BookReq {
    pub page: Page,
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[post("/book/find_id_name", data = "<p>")]
pub(crate) async fn find_by_id_or_name(db: Connection<db::Conn>, p: Json<BookReq>, _u: User) -> Resp<PageWrap<Vec<Book>>> {
    info!("{:?}", p);
    let mut p = p.into_inner();
    if p.id.is_none() {
        p.id = Some(0);
    }
    if p.name.is_none() {
        p.name = Some("".to_string());
    }
    service::book::find_by_id_or_name(db, p.id.unwrap(), p.name.unwrap(), p.page.page_no, p.page.page_size).await
}

#[post("/book/add", data = "<b>")]
pub(crate) async fn add(db: Connection<db::Conn>, b: Json<Book>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    let b = b.into_inner();
    let _ = b.validate_args(());

    match b.validate() {
        Ok(_) => (),
        Err(e) => return Resp::fail(400, "invalid param", e.to_string()),
    }
    service::book::add(db, b).await
}

#[get("/book/remove/<id>")]
pub(crate) async fn remove(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<u64> {
    info!("{:?}", id);
    service::book::remove(db, id).await
}

#[get("/book/find-by-id/<id>")]
pub(crate) async fn find_by_id(db: Connection<db::Conn>, id: i32, _u: User) -> Resp<Option<Book>> {
    info!("{:?}", id);
    service::book::find_by_id(db, id).await
}

#[post("/book/edit", data = "<b>")]
pub(crate) async fn edit(db: Connection<db::Conn>, b: Json<Book>, _u: User) -> Resp<u64> {
    info!("{:?}", b);
    service::book::edit(db, b.into_inner()).await
}

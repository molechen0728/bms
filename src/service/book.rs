use crate::{
    db,
    model::{self, book::Book},
};
use rocket::error;
use rocket_db_pools::Connection;

use super::{PageWrap, Resp};

pub(crate) async fn find_with_page(mut db: Connection<db::Conn>, page_no: i32, page_size: i32) -> Resp<PageWrap<Vec<Book>>> {
    let mut tab = model::book::Table::new(&mut db);
    let off_set = if page_no == 0 { 0 } else { (page_no - 1) * page_size };
    let arr = match tab.find_with_page(off_set, page_size).await {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    };
    if arr.is_err() {
        return Resp::fail(500, "internal error", arr.err().unwrap().to_string());
    }

    let count = match tab.count_with_page().await {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    };

    if count.is_err() {
        return Resp::fail(500, "internal error", count.err().unwrap().to_string());
    }
    Resp::success(
        200,
        "success",
        Some(PageWrap::new(page_no, page_size, count.unwrap(), arr.unwrap())),
    )
}

pub(crate) async fn find_by_id_or_name(
    mut db: Connection<db::Conn>,
    id: i32,
    name: String,
    page_no: i32,
    page_size: i32,
) -> Resp<PageWrap<Vec<Book>>> {
    let mut tab = model::book::Table::new(&mut db);
    let off_set = if page_no == 0 { 0 } else { (page_no - 1) * page_size };
    let cp = name.clone();
    let arr = match tab.find_by_id_or_name(id, name, off_set, page_size).await {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    };

    if arr.is_err() {
        return Resp::fail(500, "internal error", arr.err().unwrap().to_string());
    }

    let count = match tab.count_by_id_or_name(id, cp).await {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    };

    if count.is_err() {
        return Resp::fail(500, "internal error", count.err().unwrap().to_string());
    }

    Resp::success(
        201,
        "success",
        Some(PageWrap::new(page_no, page_size, count.unwrap(), arr.unwrap())),
    )
}

pub(crate) async fn add(mut db: Connection<db::Conn>, b: Book) -> Resp<u64> {
    let mut tab = model::book::Table::new(&mut db);
    match tab.add(b).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn remove(mut db: Connection<db::Conn>, id: i32) -> Resp<u64> {
    let mut tab = model::book::Table::new(&mut db);
    match tab.remove_by_id(id).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn find_by_id(mut db: Connection<db::Conn>, id: i32) -> Resp<Option<Book>> {
    let mut tab = model::book::Table::new(&mut db);
    match tab.find_by_id(id).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn edit(mut db: Connection<db::Conn>, b: Book) -> Resp<u64> {
    let mut tab = model::book::Table::new(&mut db);
    match tab.edit(b).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

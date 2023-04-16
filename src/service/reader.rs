use crate::{
    db,
    model::{self, reader::Reader},
    utils::delay,
};
use rocket::{error, log::private::info};
use rocket_db_pools::Connection;

use super::{PageWrap, Resp};

pub(crate) async fn list(mut db: Connection<db::Conn>, page_no: i32, page_size: i32) -> Resp<PageWrap<Vec<Reader>>> {
    let mut tab = model::reader::Table::new(&mut db);
    let off_set = if page_no == 0 { 0 } else { (page_no - 1) * page_size };
    let arr = match tab.list(off_set, page_size).await {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    };

    if arr.is_err() {
        return Resp::fail(500, "internal error", arr.err().unwrap().to_string());
    }

    let count = match tab.count().await {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    };

    if count.is_err() {
        return Resp::fail(500, "internal error", count.err().unwrap().to_string());
    }
    info!("{:?}", &count);
    Resp::success(
        200,
        "success",
        Some(PageWrap::new(page_no, page_size, count.unwrap(), arr.unwrap())),
    )
}

pub(crate) async fn add(mut db: Connection<db::Conn>, b: Reader) -> Resp<u64> {
    delay(2).await;
    let mut tab = model::reader::Table::new(&mut db);
    match tab.add(b).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn remove(mut db: Connection<db::Conn>, id: i32) -> Resp<u64> {
    let mut tab = model::reader::Table::new(&mut db);
    match tab.remove(id).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn find_by_id(mut db: Connection<db::Conn>, id: i32) -> Resp<Option<Reader>> {
    let mut tab = model::reader::Table::new(&mut db);
    match tab.find(id).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn edit(mut db: Connection<db::Conn>, b: Reader) -> Resp<u64> {
    let mut tab = model::reader::Table::new(&mut db);
    match tab.edit(b).await {
        Ok(v) => Resp::success(201, "success", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

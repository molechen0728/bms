pub(crate) mod book;
pub(crate) mod book_class;
pub(crate) mod lend_record;
pub(crate) mod reader;
pub(crate) mod usr;

use rocket::{
    serde::{json::Json, Serialize},
    Responder,
};

#[derive(Responder)]
#[response(status = 200, content_type = "json")]

pub(crate) struct Resp<D> {
    pub(crate) inner: Json<Inner<D>>,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Inner<D> {
    pub(crate) status: i32,
    pub(crate) msg: Option<String>,
    pub(crate) err: Option<String>,
    pub(crate) data: Option<D>,
}

impl<D> Resp<D> {
    pub fn fail(status: i32, msg: &'static str, err: String) -> Self {
        Self {
            inner: Json(Inner {
                status,
                msg: Some(msg.to_string()),
                err: Some(err),
                data: None,
            }),
        }
    }

    pub fn success(status: i32, msg: &'static str, data: Option<D>) -> Self {
        Self {
            inner: Json(Inner {
                status,
                msg: Some(msg.to_string()),
                err: None,
                data: data,
            }),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub(crate) struct PageWrap<D> {
    pub(crate) page_no: i32,
    pub(crate) page_size: i32,
    pub(crate) count: i32,
    pub(crate) data: D,
}

impl<D> PageWrap<D> {
    pub fn new(page_no: i32, page_size: i32, count: i32, data: D) -> Self {
        Self {
            page_no,
            page_size,
            count,
            data,
        }
    }
}

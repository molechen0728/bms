use crate::db;
use crate::router;
use crate::router::User;
use rocket::catch;
use rocket::catchers;
use rocket::http::ContentType;
use rocket::local::asynchronous::Client;
use rocket::serde::Deserialize;
use rocket::uri;
use rocket::Request;
use rocket::{routes, Build, Rocket};
use rocket_db_pools::Database;

mod book;
mod book_class;
mod health;
mod lend_record;
mod reader;
#[cfg(test)]
mod usr;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Ret<D> {
    #[serde(rename = "status")]
    _status: i32,
    #[serde(rename = "msg")]
    _msg: Option<String>,
    #[serde(rename = "err")]
    _err: Option<String>,
    #[serde(rename = "data")]
    _data: Option<D>,
}

#[catch(404)]
async fn _not_found(req: &Request<'_>) -> () {
    println!("{}", req.headers().get_one("token").unwrap());
    println!("{}", req.to_string());
    let _u = req.guard::<User>().await.unwrap();
    println!("uid: {}, uname: {}", _u.user_id, _u.user_name);
}

fn _rocket() -> Rocket<Build> {
    rocket::build()
        .attach(db::Conn::init())
        .register("/", catchers![_not_found])
        .manage(1234i64)
        .mount(
            "/api",
            routes![
                router::health::health,
                router::usr::login,
                router::usr::reset_password,
                router::usr::find,
                router::book::find_with_page,
                router::book::find_by_id_or_name,
                router::book::add,
                router::book::remove,
                router::book::find_by_id,
                router::book::edit,
                router::book_class::list,
                router::book_class::find_by_id,
                router::book_class::add,
                router::book_class::remove,
                router::book_class::edit,
                router::lend_record::list,
                router::lend_record::find_by_id,
                router::lend_record::add,
                router::lend_record::remove,
                router::lend_record::edit,
                router::reader::list,
                router::reader::find_by_id,
                router::reader::add,
                router::reader::remove,
                router::reader::edit,
            ],
        )
}

async fn _get_token() -> String {
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::usr::login))
        .header(ContentType::new("application", "x-www-form-urlencoded"))
        .body("uname=admin1&upass=123456")
        .dispatch()
        .await;
    let ret = resp.into_json::<Ret<String>>().await;
    ret.unwrap()._data.unwrap()
}

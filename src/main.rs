mod db;
mod model;
mod router;
mod service;
mod tests;
mod utils;

use model::now;
use rocket::Request;
use rocket::{catch, catchers, launch, routes};
use rocket_db_pools::Database;

use crate::router::User;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(422)]
async fn login_param_invalid(req: &Request<'_>) -> Option<String> {
    match req.guard::<User>().await {
        rocket::outcome::Outcome::Success(_) => None,
        rocket::outcome::Outcome::Failure(_) => Some(format!("User name and password are both required.")),
        rocket::outcome::Outcome::Forward(_) => None,
    }
}

#[catch(401)]
fn unauthorized(_req: &Request) -> String {
    "Unauthorized".to_string()
}

#[launch]
async fn rocket() -> _ {
    let ts = now().await;
    rocket::build()
        .attach(db::Conn::init())
        .manage(ts)
        .register("/", catchers![not_found, unauthorized, login_param_invalid])
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

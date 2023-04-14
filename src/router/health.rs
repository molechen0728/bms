use crate::utils::http_client::network_accessible;
use rocket::get;

#[get("/health")]
pub(crate) async fn health() -> String {
    let s = network_accessible().await;
    format!("{s}")
}

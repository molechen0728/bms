use super::{Ret, _rocket};
use crate::router;
use crate::tests::_get_token;
use rocket::http::Header;
use rocket::local::asynchronous::Client;
use rocket::uri;

#[rocket::async_test]
async fn test_health() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .get(uri!("/api", router::health::health))
        .header(Header::new("token", token))
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    assert_ne!(resp.into_string().await.unwrap(), "");
}

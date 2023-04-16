use super::{Ret, _rocket};
use crate::model::usr::Usr;
use crate::router;
use crate::tests::_get_token;
use rocket::http::Header;
use rocket::local::asynchronous::Client;
use rocket::uri;

#[rocket::async_test]
async fn test_login() {
    let token = _get_token().await;
    assert_ne!(token, "");
}

#[rocket::async_test]
async fn test_reset_password() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::usr::reset_password))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "uid": 2,
            "upass": "123456"
        }
        "#,
        )
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    let ret = resp.into_json::<Ret<i32>>().await;
    assert_eq!(ret.unwrap()._data.unwrap(), 1);
}

#[rocket::async_test]
async fn test_find() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .get(uri!("/api", router::usr::find(id = 1)))
        .header(Header::new("token", token))
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    let ret = resp.into_json::<Ret<Usr>>().await;
    assert_eq!(ret.unwrap()._data.unwrap().uid, Some(1));
}

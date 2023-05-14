#[allow(unused_imports)]
use super::{Ret, _rocket};
#[allow(unused_imports)]
use crate::router;
#[allow(unused_imports)]
use crate::tests::_get_token;
#[allow(unused_imports)]
use rocket::http::Header;
#[allow(unused_imports)]
use rocket::local::asynchronous::Client;
#[allow(unused_imports)]
use rocket::uri;

#[rocket::async_test]
async fn test_list() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::book_class::list))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "page_no": 1,
            "page_size": 1
        }
        "#,
        )
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    assert_ne!(resp.into_string().await.unwrap(), "");
}

#[rocket::async_test]
async fn test_add() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::book_class::add))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "cname": "历史"
        }
        "#,
        )
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    let ret = resp.into_json::<Ret<i32>>().await;
    assert_eq!(ret.unwrap()._status, 500);
}

#[rocket::async_test]
async fn test_remove() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .get(uri!("/api", router::book_class::remove(id = 100)))
        .header(Header::new("token", token))
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    let ret = resp.into_json::<Ret<i32>>().await;
    assert_eq!(ret.unwrap()._data.unwrap(), 0);
}

#[rocket::async_test]
async fn test_edit() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::book_class::edit))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "cid": 1,
            "cname": "马克思主义"
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
async fn test_find_by_id() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .get(uri!("/api", router::book_class::find_by_id(id = 1)))
        .header(Header::new("token", token))
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    assert_ne!(resp.into_string().await.unwrap(), "");
}

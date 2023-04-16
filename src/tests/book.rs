use super::{Ret, _rocket};
use crate::router;
use crate::tests::_get_token;
use rocket::http::Header;
use rocket::local::asynchronous::Client;
use rocket::uri;

#[rocket::async_test]
async fn test_find_with_page() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::book::find_with_page))
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
async fn test_find_by_id_or_name() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .post(uri!("/api", router::book::find_by_id_or_name))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "id": 0,
            "name": "秘密",
            "page": {
                "page_no": 1,
                "page_size": 10
            }
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
        .post(uri!("/api", router::book::add))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "bname": "画的秘密5",
            "author": "马克-安托万·马修 ",
            "publish": "北京联合出版公司·后浪出版公司",
            "isbn": "9787550265608",
            "introduction": "一本关于友情的疗伤图像小说，直击人内心最为隐秘的情感。 一部追寻艺术的纸上悬疑电影，揭示命运宇宙中奇诡的真相。 ★ 《画的秘密》荣获欧洲第二大漫画节“瑞士谢尔漫画节最佳作品奖”。 作者曾两度夺得安古兰国际漫画节重要奖项。 ★ 《画的秘密》是一部罕见的、结合了拼贴、镜像、3D等叙事手法的实验型漫画作品。作者巧妙地调度光线、纬度、时间、记忆，在一个悬念重重又温情治愈的故事中，注入了一个有关命运的哲学议题。",
            "language": "中文",
            "price": 60.0,
            "pub_time": null,
            "class_id": 9,
            "pressmark": 13,
            "state": 100
        }
        "#,
        )
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    let ret = resp.into_json::<Ret<i32>>().await;
    assert_eq!(ret.unwrap()._status, 400);
}

#[rocket::async_test]
async fn test_remove() {
    let token = _get_token().await;
    let client = Client::tracked(_rocket()).await.expect("valid rocket instance");
    let resp = client
        .get(uri!("/api", router::book::remove(id = 100)))
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
        .post(uri!("/api", router::book::edit))
        .header(Header::new("token", token))
        .body(
            r#"
        {
            "bid": 9,
            "bname": "画的秘密",
            "author": "马克-安托万·马修 ",
            "publish": "北京联合出版公司·后浪出版公司",
            "isbn": "9787550265608",
            "introduction": "一本关于友情的疗伤图像小说，直击人内心最为隐秘的情感。 一部追寻艺术的纸上悬疑电影，揭示命运宇宙中奇诡的真相。 ★ 《画的秘密》荣获欧洲第二大漫画节“瑞士谢尔漫画节最佳作品奖”。 作者曾两度夺得安古兰国际漫画节重要奖项。 ★ 《画的秘密》是一部罕见的、结合了拼贴、镜像、3D等叙事手法的实验型漫画作品。作者巧妙地调度光线、纬度、时间、记忆，在一个悬念重重又温情治愈的故事中，注入了一个有关命运的哲学议题。",
            "language": "中文",
            "price": 60.0,
            "pub_time": null,
            "class_id": 9,
            "pressmark": 13,
            "state": 0
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
        .get(uri!("/api", router::book::find_by_id(id = 1)))
        .header(Header::new("token", token))
        .dispatch()
        .await;

    assert_eq!(resp.status().code, 200);
    assert_ne!(resp.into_string().await.unwrap(), "");
}

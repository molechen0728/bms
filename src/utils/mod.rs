use rocket::tokio::time;

pub(crate) mod http_client;

pub(crate) async fn delay(seconds: i32) -> () {
    time::sleep(time::Duration::from_secs(seconds as u64)).await;
}

pub(crate) async fn network_accessible() -> String {
    let params = [("uname", "admin1"), ("upass", "123456")];
    let client = reqwest::Client::new();
    let s = match client.post("https://baidu.com/").form(&params).send().await {
        Ok(resp) => match resp.text().await {
            Ok(_) => "ok".to_string(),
            Err(e) => e.to_string(),
        },
        Err(e) => e.to_string(),
    };

    format!("{s}")
}

pub(crate) mod book;
pub(crate) mod book_class;
pub(crate) mod health;
pub(crate) mod lend_record;
pub(crate) mod reader;
pub(crate) mod usr;

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
    State,
};
use sha2::Sha384;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub(crate) struct User {
    pub(crate) user_name: String,
    pub(crate) up_time: i64,
    pub(crate) user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let up_time = req.guard::<&State<i64>>().await.unwrap().inner();

        match req.headers().get_one("token") {
            None => Outcome::Failure((Status::Unauthorized, "token is required".to_string())),
            Some(sign) => {
                let key: Hmac<Sha384> = Hmac::new_from_slice(b"~~~~~~~~~~~~~~~~Gii").unwrap();
                if let Ok(token) = VerifyWithKey::<Token<Header, BTreeMap<String, User>, _>>::verify_with_key(sign, &key) {
                    let header = token.header();
                    let claims = token.claims();
                    if header.algorithm != AlgorithmType::Hs384 || *up_time != claims["user"].up_time {
                        return Outcome::Failure((Status::Unauthorized, "wrong token".to_string()));
                    }
                    return Outcome::Success(claims["user"].clone());
                } else {
                    Outcome::Failure((Status::Unauthorized, "token is required".to_string()))
                }
            }
        }
    }
}

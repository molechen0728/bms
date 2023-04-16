use crate::{
    db,
    model::{self, usr::Usr},
    router::User,
    service::Resp,
};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use rocket::{error, log::private::info};
use rocket_db_pools::Connection;
use sha2::Sha384;
use std::collections::BTreeMap;

pub(crate) async fn login(mut db: Connection<db::Conn>, mut a: Usr, up_time: i64) -> Resp<String> {
    let mut tab = model::usr::Table::new(&mut db);
    if let Some(p) = &a.upass {
        a.upass = Some(format!("{:x}", md5::compute(&p)));
        info!("{:?}", a.upass);
    }
    match tab.find_with_id_and_password(a).await {
        Ok(v) => {
            if let Some(u) = v {
                let key: Hmac<Sha384> = Hmac::new_from_slice(b"~~~~~~~~~~~~~~~~Gii").unwrap();
                let header = Header {
                    algorithm: AlgorithmType::Hs384,
                    ..Default::default()
                };
                let mut claims: BTreeMap<&str, Option<User>> = BTreeMap::new();
                claims.insert(
                    "user",
                    Some(User {
                        user_name: u.clone().uname.unwrap(),
                        up_time: up_time,
                        user_id: u.uid.unwrap().clone(),
                    }),
                );
                let token = Token::new(header, claims).sign_with_key(&key).unwrap();
                Resp::success(101, "success", Some(token.as_str().to_string()))
            } else {
                return Resp::success(100, "no such user", None);
            }
        }
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn edit_password(mut db: Connection<db::Conn>, mut a: Usr) -> Resp<u64> {
    let mut tab = model::usr::Table::new(&mut db);
    if let Some(p) = &a.upass {
        a.upass = Some(format!("{:x}", md5::compute(&p)));
    }
    match tab.edit_password(a).await {
        Ok(v) => Resp::success(103, "", Some(v)),
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

pub(crate) async fn find_with_id(mut db: Connection<db::Conn>, id: i32) -> Resp<Usr> {
    let mut tab = model::usr::Table::new(&mut db);
    match tab.find_with_id(id).await {
        Ok(v) => {
            if v.is_none() {
                return Resp::success(100, "no such user", None);
            }
            Resp::success(101, "success", Some(v.unwrap()))
        }
        Err(e) => {
            error!("{}", e);
            Resp::fail(500, "internal error", e.to_string())
        }
    }
}

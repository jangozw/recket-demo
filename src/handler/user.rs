use crate::{app, model};
use sqlx::types::Json;


#[get("/user/list")]
pub async fn user_list() -> String {
    let list = model::user::get_user_list().await;
    for (_, u) in list.iter().enumerate() {
        println!("user: {:?}", u);
    }
    let resp = app::response::Response::ok(list);
    serde_json::to_string(&resp).unwrap()
}
#[get("/user/<uid>")]
pub async fn user_detail(uid: u64) -> String {
    let res = model::user::get_user_detail(uid).await;
    match res {
        Ok(user) => {
            let resp = app::response::Response::ok(user);
            return serde_json::to_string(&resp).unwrap();
        }
        Err(e) => {
            let msg = model::db::err_string(e);
            let resp = app::response::Response::fail(msg, "");
            return serde_json::to_string(&resp).unwrap();
        }
    }
}


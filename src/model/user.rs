use futures::executor::block_on;
use futures::TryStreamExt;
use crate::model::db;
use sqlx::{Error, Row};
use serde_derive::Serialize;

#[derive(Debug,Serialize,sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub info: String,
    pub age: i32,
}

pub async fn get_user_list()->Vec<User>{
    let mut list = Vec::new();
    let ins = db::Instance::get();
    let mut rows = sqlx::query("SELECT * FROM user").fetch(&ins.db);
    while let Some(row) = rows.try_next().await.unwrap() {
        // map the row into a user-defined domain type
        let id:u64 = row.try_get_unchecked("id").unwrap();
        let age:i32 = row.try_get_unchecked("age").unwrap();
        let name:&str = row.try_get("name").unwrap();
        let info:&str = row.try_get("info").unwrap();
        list.push(User{
            id:id,
            name: name.to_string(),
            info: info.to_string(),
            age: age,
        });
    }
    list
}

pub async fn get_user_detail(id :u64) -> Result<User, Error> {
    let ins = db::Instance::get();
    let stream = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ? ")
        .bind(id)
        .fetch_one(&ins.db).await;
    stream
}


#[test]
fn test_get_user_detail(){
    let res = block_on(get_user_detail(3));
    match res {
        Ok(user) => {
            println!("{:?}", user)
        }
        Err(e) => {
            let msg = e.as_database_error().unwrap().message();
            println!("err: {}", msg)
        }
    }

}

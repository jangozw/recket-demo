
const CODE_OK: i32 = 200;
const CODE_FAIL: i32 = 400;
// const CODE_NOT_LOGIN: i32 = 401;

use serde_derive::Serialize;
#[derive(Debug,Serialize)]
pub struct Response<T>{
    pub code: i32,
    pub msg: String,
    pub data: T
}

impl <T>Response<T> {
    pub fn ok(data: T)->Response<T>{
        Response{
            code: CODE_OK,
            msg: "success".to_string(),
            data: data
        }
    }
    pub fn err(code: i32, msg: String, data: T)->Response<T>{
        Response{
            code: code,
            msg: msg,
            data: data,
        }
    }
    pub fn fail(msg: String, data: T)->Response<T>{
        Response{
            code: CODE_FAIL,
            msg: msg,
            data: data,
        }
    }
}

#[test]
fn test_resp(){
    println!("------------asdf");
    println!("0000{}", "okk");
    let res = Response::ok(123);
    println!("{:?}", res);

}

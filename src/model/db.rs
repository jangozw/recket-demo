use std::sync::Arc;
use sqlx::{MySqlPool};
use futures::executor::block_on;
use sqlx::Error;

pub struct Instance{
    pub db: sqlx::pool::Pool<sqlx::MySql>
}
impl Instance {
    pub fn get() -> Arc<Self> {
        static mut DB_INSTANCE: Option<Arc<Instance>> = None;
        unsafe {// Rust中使用可变静态变量都是unsafe的
            DB_INSTANCE.get_or_insert_with(|| {
                let ins = block_on(new());
                Arc::new(ins)
            }).clone()
        }
    }
}
async fn new()->Instance{
    let pool =  MySqlPool::connect("mysql://root:123456@127.0.0.1/test").await.unwrap();
    Instance{
        db: pool,
    }
}




pub fn err_string(e: sqlx::Error)->String{
    return match e {
        Error::Database(d) => {
            d.message().to_string()
        }
        Error::RowNotFound => {
            "row not found".to_string()
        }
        Error::TypeNotFound { .. } => {
            "type not found".to_string()
        }
        _ => {
            "unknown err".to_string()
        }
    }
}
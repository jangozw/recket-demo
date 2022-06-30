mod handler;
mod model;
mod app;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // let ins = app::instance::Instance::new().await;
    let figment = rocket::Config::figment()
        .merge(("port", 8001));
        //.merge(("limits", Limits::new().limit("json", 2.mebibytes())));
    let _rocket = rocket::custom(figment)
        .mount("/v1", routes![
            handler::user::user_list,
            handler::user::user_detail,
            handler::order::order_list,
        ]).launch().await?;
    Ok(())
}

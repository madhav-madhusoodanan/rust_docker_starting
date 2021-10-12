use actix_web::{get,  web};
use std::sync::Mutex;
pub struct AppState {
    counter: Mutex<i32>,
}

#[get("/")]
pub async fn hello(data: web::Data<AppState>) -> String{
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("request id: {}", counter)
}

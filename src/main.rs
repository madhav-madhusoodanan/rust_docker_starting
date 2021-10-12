use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
struct AppState {
    counter: Mutex<i32>,
}
use std::env;

#[get("/")]
async fn hello(data: web::Data<AppState>) -> String{
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("request id: {}", counter)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from("8080")
    };
    let host = "127.0.0.1";
    let address = format!("{}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(hello)
    })
    .bind(&address)?
    .run()
    .await
}
use actix_web::{get, App, HttpResponse, HttpServer};
use std::env;

#[get("/")]
async fn hello() -> HttpResponse{
    HttpResponse::Ok().body("Yoooo!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from("8080")
    };
    let host = "0.0.0.0";
    let address = format!("{}:{}", host, port);
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(&address)?
    .run()
    .await
}
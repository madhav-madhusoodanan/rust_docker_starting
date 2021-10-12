use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello dude!")
}

#[post("/echo")]
async fn echo(req: String) -> impl Responder {
    HttpResponse::Ok().body(req)
}

#[get("/hey")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("This is a manual hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(manual_hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
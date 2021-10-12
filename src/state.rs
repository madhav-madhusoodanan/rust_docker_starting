use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> String{
    let app_name = &data.app_name;
    format!("Yooo {}!", app_name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
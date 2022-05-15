use actix_web::{get, web, App, HttpServer, Responder};

#[get("/users/{name}")]
async fn user(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello_world", web::get().to(|| async { "Hello World!" }))
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

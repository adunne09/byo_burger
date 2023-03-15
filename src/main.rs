use actix_web::{get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use env_logger::Env;

mod burger;
use burger::Burger;

#[get("/ingredients")]
async fn list_ingredients() -> impl Responder {
    HttpResponse::Ok().json(Burger::list_ingredients())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(status_handler)
            .service(list_ingredients)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/status")]
async fn status_handler(_req: HttpRequest) -> impl Responder {
    HttpResponse::NoContent()
}

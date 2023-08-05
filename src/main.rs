use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header};
use actix_cors::Cors;
mod handlers;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("https://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::CONTENT_TYPE])
            )
            .service(root)
            .configure(handlers::register_handlers_to)
    });
    let result = server.bind(("127.0.0.1", 8080))?.run().await;
    return result;
}

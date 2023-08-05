use actix_web::{web, Responder, HttpResponse};
mod math_handlers;

pub fn register_handlers_to(config: &mut web::ServiceConfig) {
    let config: &mut web::ServiceConfig = config.service(
        web::resource("/hello")
            .route(web::get().to(handler))
    );
    math_handlers::register_handlers_to(config)
}

async fn handler() -> impl Responder {
    HttpResponse::Ok().body("hello iei")
}

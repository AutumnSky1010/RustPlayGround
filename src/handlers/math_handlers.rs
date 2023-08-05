use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use super::super::domain::math;

#[derive(Deserialize)]
struct Values{
    x: i32,
    y: i32
}

async fn gcd_handler (values: web::Query<Values>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", math::gcd(values.x, values.y)))
}

async fn max_handler (values: web::Query<Values>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", math::max(values.x, values.y)))
}

async fn min_handler (values: web::Query<Values>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", math::min(values.x, values.y)))
}

async fn pow_handler (values: web::Query<Values>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", math::pow(values.x, values.y)))
}

pub fn register_handlers_to(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/math/gcd")
            .route(web::get().to(gcd_handler))
    );
    config.service(web::resource("/math/max")
        .route(web::get().to(max_handler))
    );
    config.service(web::resource("/math/min")
        .route(web::get().to(min_handler))
    );
    config.service(web::resource("/math/pow")
        .route(web::get().to(pow_handler))
    );
}
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header};
use actix_cors::Cors;
use serde::{Deserialize, de::value};

#[derive(Deserialize)]
struct Values{
    x: i32,
    y: i32
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[get("/gcd")]
async fn gcd_handler (values: web::Query<Values>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", gcd(values.x, values.y)))
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    if y > x {
        let tmp = x;
        x = y;
        y = tmp;
    }
    let result: i32 = if x % y == 0 {
        y
    } else {
        gcd(y, x % y)
    };
    return result;
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
            .service(gcd_handler)
    });
    let result = server.bind(("127.0.0.1", 8080))?.run().await;
    return result;
}

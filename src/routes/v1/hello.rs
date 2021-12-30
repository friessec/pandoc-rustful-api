use actix_web::HttpResponse;

#[get("/hello")]
fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .body("Hello World!".to_string())
}
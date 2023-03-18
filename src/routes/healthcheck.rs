use actix_web::{get, HttpResponse};

#[get("/healthcheck")]
pub async fn healthcheck() -> HttpResponse {
    println!("OK");
    HttpResponse::Ok().finish()
}

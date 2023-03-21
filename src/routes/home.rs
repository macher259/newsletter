use actix_web::{get, http::header::ContentType, HttpResponse};
use build_html::{Html, HtmlContainer, HtmlPage};

#[get("/")]
pub async fn home() -> HttpResponse {
    let body = HtmlPage::new()
        .with_title("home")
        .with_header(1, "Welcome to out newsletter!")
        .to_html_string();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}

use actix_web::{get, http::header::ContentType, HttpResponse};

#[get("/")]
pub async fn home() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Home</title>
            </head>
            <body>
                <p>Welcome to our newsletter!</p>
                <a href="/login">Log in</a>
            </body>
        </html>"#,
    )
}

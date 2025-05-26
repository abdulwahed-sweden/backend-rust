use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn home() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Rust Backend</title>
        </head>
        <body>
            <h1>Welcome to Rust Backend Home Page</h1>
        </body>
        </html>
    "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

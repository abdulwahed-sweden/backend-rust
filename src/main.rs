use actix_web::{web, App, HttpServer, Responder, HttpResponse, Result};
use actix_files as fs;
use std::fs::read_to_string;

async fn home() -> Result<impl Responder> {
    let html_content = read_to_string("templates/index.html")
        .unwrap_or_else(|_| {
            r#"
            <!DOCTYPE html>
            <html lang="ar" dir="rtl">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Rust Backend - صفحة البداية</title>
                <link rel="stylesheet" href="/static/css/style.css">
                <link href="https://fonts.googleapis.com/css2?family=Tajawal:wght@300;400;500;700&family=Poppins:wght@300;400;500;600;700&display=swap" rel="stylesheet">
            </head>
            <body>
                <div class="container">
                    <header class="header">
                        <h1 class="title">مرحباً بك في Rust Backend</h1>
                        <p class="subtitle">تطبيق ويب حديث مبني بـ Actix Web</p>
                    </header>
                    
                    <main class="main-content">
                        <div class="card">
                            <h2>الميزات المتاحة</h2>
                            <ul class="features-list">
                                <li>🦀 مبني بلغة Rust السريعة والآمنة</li>
                                <li>⚡ Actix Web Framework عالي الأداء</li>
                                <li>🐳 دعم Docker للنشر السهل</li>
                                <li>🎨 تصميم عصري وسريع الاستجابة</li>
                                <li>🌐 دعم متعدد اللغات</li>
                            </ul>
                        </div>
                        
                        <div class="card">
                            <h2>الإحصائيات</h2>
                            <div class="stats-grid">
                                <div class="stat-item">
                                    <span class="stat-number">100%</span>
                                    <span class="stat-label">آمان الذاكرة</span>
                                </div>
                                <div class="stat-item">
                                    <span class="stat-number">0ms</span>
                                    <span class="stat-label">زمن البدء</span>
                                </div>
                                <div class="stat-item">
                                    <span class="stat-number">⚡</span>
                                    <span class="stat-label">أداء فائق</span>
                                </div>
                            </div>
                        </div>
                    </main>
                    
                    <footer class="footer">
                        <p>© 2025 Rust Backend - تم التطوير بواسطة عبدالواحد</p>
                    </footer>
                </div>
                
                <script src="/static/js/main.js"></script>
            </body>
            </html>
            "#.to_string()
        });
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}

async fn api_status() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Rust Backend API is running",
        "version": "0.1.0"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080");
    println!("📁 Static files served from /static/");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/api/status", web::get().to(api_status))
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
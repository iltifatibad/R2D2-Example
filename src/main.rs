use actix_web::{web, App, HttpServer, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;  // r2d2 için sqlite bağlantı yöneticisi
use rusqlite::{params, Connection}; // rusqlite için gerekli importlar
use std::env;
use dotenv::dotenv;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
    
    let manager = SqliteConnectionManager::file(&db_url);
    
    let pool = Pool::builder().build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) 
            .route("/", web::get().to(api::add_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

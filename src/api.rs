use actix_web::{web, HttpResponse, Error};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, params};

pub async fn add_data(pool: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, Error> {
    // Havuzdan bir bağlantı alıyoruz
    let conn = pool.get().unwrap();
    
    // Veritabanına veri ekleme işlemi
    let result = conn.execute("INSERT INTO users (name) VALUES (?1)", params!["Pepe"]);
    
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("User inserted successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error inserting user")),
    }
}

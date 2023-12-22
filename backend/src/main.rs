use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPool;
use std::env;
use dotenv::dotenv;

mod models;
use models::prayer_wall::PrayerWall;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to database");

    // Check for existing tables
    let rows = sqlx::query("SHOW TABLES")
        .fetch_all(&pool)
        .await.expect("Failed to fetch tables");

    for row in rows {
        let table_name: String = row.get(0);
        println!("Table: {}", table_name);
    }
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/add_prayer_wall", web::post().to(add_prayer_wall))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn add_prayer_wall(
    pool: web::Data<MySqlPool>,
    prayer_request: web::Json<PrayerWall>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO prayerwall (title, content, userid) VALUES (?, ?, ?)",
        prayer_wall.title,
        prayer_wall.description,
        prayer_wall.userid
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Prayer request added"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}


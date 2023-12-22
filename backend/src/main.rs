use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPool;
use std::env;
use dotenv::dotenv;

mod models;
use models::prayer_request::PrayerRequest;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/add_prayer_request", web::post().to(add_prayer_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn add_prayer_request(
    pool: web::Data<MySqlPool>,
    prayer_request: web::Json<PrayerRequest>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO prayer_requests (title, description, user_id) VALUES (?, ?, ?)",
        prayer_request.title,
        prayer_request.description,
        prayer_request.user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Prayer request added"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}


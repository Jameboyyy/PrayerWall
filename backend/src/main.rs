use sqlx::mysql::MySqlPool;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&database_url).await?;
    println!("Connected to the database");

    // Your database operations go here

    Ok(())
}

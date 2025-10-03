use rocket::State;
use sqlx::{Pool, Sqlite};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index(pool: &State<Pool<Sqlite>>) -> &'static str {
    let mut conn = pool.acquire().await.expect("Failed to acquire database connection");
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM fortunes")
        .fetch_one(&mut *conn)
        .await
        .expect("Failed to execute query");
    println!("Query result: {}", row.0);
    "Hello, worlds!"
}

#[launch]
async fn rocket() -> _ {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);
    let db_pool = sqlx::SqlitePool::connect(&database_url).await.expect("Failed to connect to database");

    rocket::build().manage(db_pool).mount("/", routes![index])
}

use rocket::State;
use sqlx::{Pool, Sqlite};
use rocket::serde::{json::Json, Serialize};

#[derive(sqlx::FromRow, Serialize)]
#[derive(Debug)]
struct Fortune {
    id: u8,
    fortune: String,
}

#[macro_use] extern crate rocket;
#[get("/")]
async fn get_fortunes(pool: &State<Pool<Sqlite>>) -> Json<Vec<Fortune>> {
    let mut conn = pool.acquire().await.expect("Failed to acquire database connection");
    let rows: Vec<Fortune> = sqlx::query_as("SELECT id, fortune FROM fortunes")
        .fetch_all(&mut *conn)
        .await
        .expect("Failed to execute query");

    Json(rows)
}

#[get("/<id>")]
async fn get_fortune(pool: &State<Pool<Sqlite>>, id: u8) -> Result<Json<Fortune>, rocket::http::Status> {
    let mut conn = pool.acquire().await.expect("Failed to acquire database connection");
    let row: Fortune = sqlx::query_as("SELECT id, fortune FROM fortunes WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .map_err(|_| rocket::http::Status::NotFound)?;

    Ok(Json(row))
}


#[launch]
async fn rocket() -> _ {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);
    let db_pool = sqlx::SqlitePool::connect(&database_url).await.expect("Failed to connect to database");

    rocket::build().manage(db_pool).mount("/", routes![get_fortunes, get_fortune])
}

use bevy::reflect::erased_serde::__private::serde::de::Unexpected::Str;
use sqlx::{postgres::PgPoolOptions, PgPool, Error};
// use crate::score::Score;
use crate::Score;
pub async fn init_db() -> PgPool {
    let url = String::from("postgresql://postgres:Kasjauns2003@localhost:5432/hw7?schema=public");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    pool
}


pub async fn insert_score(
    pool: &PgPool,
    item: Score,
) -> Result<(), Error> {
    let query_result = match sqlx::query_as!(
            Score,
            "INSERT INTO High_Scores VALUES ($1, $2) RETURNING *",
            item.value,
            item.lives,  // Removed the extra comma here
        )
        .fetch_one(pool)
        .await
    {
        Ok(platform) => platform,
        Err(_) => return Err(Error::RowNotFound),
    };
    Ok(())
}



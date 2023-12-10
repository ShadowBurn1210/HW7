use bevy::reflect::erased_serde::__private::serde::de::Unexpected::Str;
use sqlx::{postgres::PgPoolOptions, PgPool, Error};
use crate::score::Score;

pub async fn init_db() -> PgPool {
    let url = String::from(DATABASE_URL);

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


async fn insert_score(
    pool: &PgPool,
    item: Score,
) -> Result<(), Error> {
    let query_result = match sqlx::query_as!(
            Score,
            "INSERT INTO High_Scores VALUES ($1, $2) RETURNING *",
            item.value,
        item.lives,

        )
        .fetch_one(pool)
        .await
    {
        Ok(platform) => platform,
        Err(_) => return Err(Error::RowNotFound),
    };
    Ok(())
}

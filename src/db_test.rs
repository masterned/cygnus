use std::error::Error;

use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = SqlitePool::connect("sqlite:reference.db").await?;

    list_items(&pool).await?;
    Ok(())
}

async fn list_items(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let recs = sqlx::query!(
        r#"
SELECT item_id, name, weight
FROM item
ORDER BY item_id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!("[{}] {}: {}", rec.item_id, rec.name, &rec.weight);
    }

    Ok(())
}

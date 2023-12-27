use sqlx::{Pool, Sqlite};

pub struct CachedPage {
    url: String,
    content: String,
    updated_at: i64,
}

pub async fn get_cached_page(
    url: &str,
    database: &Pool<Sqlite>,
) -> anyhow::Result<Option<CachedPage>> {
    let result = sqlx::query_as!(
        CachedPage,
        "SELECT url, content, updated_at FROM pages WHERE url = ?",
        url
    )
    .fetch_optional(database)
    .await?;

    Ok(result)
}

pub async fn add_cached_page(
    cached_page: &CachedPage,
    database: &Pool<Sqlite>,
) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO pages (url, content, updated_at) VALUES (?, ?, ?)",
        cached_page.url,
        cached_page.content,
        cached_page.updated_at
    )
    .execute(database)
    .await?;

    Ok(())
}

pub async fn update_cached_page(
    cached_page: &CachedPage,
    database: &Pool<Sqlite>,
) -> anyhow::Result<()> {
    sqlx::query!(
        "UPDATE pages SET content = ? WHERE url = ? AND updated_at < ?",
        cached_page.content,
        cached_page.url,
        cached_page.updated_at
    )
    .execute(database)
    .await?;

    Ok(())
}

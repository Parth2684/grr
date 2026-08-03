use sqlx::{Pool, Sqlite};

pub async fn vault_exists(db: &Pool<Sqlite>) -> bool {
    let check = sqlx::query!(
        r#"
        SELECT present FROM vault
        WHERE id = 1
    "#
    )
    .fetch_one(db)
    .await;
    match check {
        Err(err) => {
            eprintln!("Error checking in database if vault exists: {:?}", err);
            false
        }
        Ok(check) => check.present,
    }
}

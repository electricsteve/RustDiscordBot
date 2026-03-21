use serenity::all::UserId;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use super::constants::COMPONENT_ID;

pub async fn migrate(db: &Surreal<Db>) -> Result<(), surrealdb::Error> {
    db.query("
DEFINE TABLE IF NOT EXISTS todo SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS list ON TABLE todo TYPE array<string>;
        ",).await?;
    Ok(())
}

pub fn add_todo(user: UserId, content: String, db: &Surreal<Db>) {
    let uid = user.get();
}
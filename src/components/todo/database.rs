use serenity::all::UserId;
use surrealdb::engine::local::Db;
use surrealdb::opt::PatchOp;
use surrealdb::Surreal;
use surrealdb::types::SurrealValue;
use super::constants::COMPONENT_ID;

pub async fn migrate(db: &Surreal<Db>) -> Result<(), surrealdb::Error> {
    db.query("
DEFINE TABLE IF NOT EXISTS todo SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS list ON TABLE todo TYPE array<string>;
        ",).await?;
    Ok(())
}

pub async fn add_todo(user: UserId, content: String, db: &Surreal<Db>) {
    let uid = user.get();
    let todo: Option<Todo> = db.upsert(("todo", uid as i64)).patch(PatchOp::add("/list", [content])).await.expect("Error adding to-do item");
}

#[derive(SurrealValue)]
struct Todo {
    list: Vec<String>,
}
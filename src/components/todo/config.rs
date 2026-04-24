use std::sync::OnceLock;

use super::constants::COMPONENT_ID;
use crate::core::database::{get_component_config, set_component_config};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::{SurrealValue, ToSql};
use tokio::sync::RwLock;

static SETTINGS: OnceLock<RwLock<TodoConfig>> = OnceLock::new();

#[derive(SurrealValue, Clone, Default, Debug, PartialEq)]
pub struct TodoConfig {
    #[surreal(default)]
    pub testing: bool,
}

async fn ensure_loaded(db: &Surreal<Db>) -> Result<(), crate::Error> {
    if SETTINGS.get().is_none() {
        let cfg = get_component_config(COMPONENT_ID.to_string(), db).await?;
        let _ = SETTINGS.set(RwLock::new(cfg)); // ignore race if another task set first
    }
    Ok(())
}

pub async fn get_config(db: &Surreal<Db>) -> Result<TodoConfig, crate::Error> {
    ensure_loaded(db).await?;
    let lock = SETTINGS.get().ok_or_else(|| {
        crate::ErrorType::LockError("Config not initialized while it should have been".to_string())
    })?;
    let cfg = lock.read().await;
    Ok(cfg.clone())
}

pub async fn update_config(db: &Surreal<Db>, new_cfg: TodoConfig) -> Result<(), crate::Error> {
    dbg!(new_cfg.clone().into_value().to_sql());
    ensure_loaded(db).await?;
    set_component_config(COMPONENT_ID.to_string(), new_cfg.clone(), db).await?;
    let lock = SETTINGS.get().ok_or_else(|| {
        crate::ErrorType::LockError("Config not initialized while it should have been".to_string())
    })?;
    let mut cfg = lock.write().await;
    *cfg = new_cfg;
    Ok(())
}

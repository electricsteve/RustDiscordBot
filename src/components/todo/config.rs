use super::constants::COMPONENT_ID;
use crate::core::database::{get_component_config, set_component_config};
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CollectComponentInteractions, CreateInteractionResponse};
use std::sync::OnceLock;
use std::time::Duration;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::SurrealValue;
use tokio::sync::RwLock;

// TODO: Component config macro
// Issue URL: https://github.com/electricsteve/Rustey/issues/14
// The current way of getting config to work with a cache is too complicated, this should be made into a macro.
static SETTINGS: OnceLock<RwLock<TodoConfig>> = OnceLock::new();

#[derive(SurrealValue, Clone, Default, Debug, PartialEq)]
pub struct TodoConfig {
    #[surreal(default)]
    pub show_count: bool,
}

async fn ensure_loaded(db: &Surreal<Db>) -> Result<(), crate::Error> {
    if SETTINGS.get().is_none() {
        let cfg = get_component_config(COMPONENT_ID, db).await?;
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
    ensure_loaded(db).await?;
    set_component_config(COMPONENT_ID, new_cfg.clone(), db).await?;
    let lock = SETTINGS.get().ok_or_else(|| {
        crate::ErrorType::LockError("Config not initialized while it should have been".to_string())
    })?;
    let mut cfg = lock.write().await;
    *cfg = new_cfg;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn config(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let data = ctx.data();
    let cfg = get_config(&data.database).await?;
    let show_count_str = if cfg.show_count { "Yes" } else { "No" };
    let text = format!("Current configuration:\n- Show item count: {}", show_count_str);
    let button = serenity::CreateButton::new("show_count").label("Toggle \"Show item count\"");
    let buttons = [button];
    let action_row = serenity::CreateActionRow::buttons(&buttons);
    let component = serenity::CreateComponent::ActionRow(action_row);
    let message = CreateReply::new().content(text).components(vec![component]);
    let reply_handle = ctx.send(message).await?;
    let interaction = match reply_handle
        .message()
        .await?
        .id
        .collect_component_interactions(ctx.serenity_context())
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        Some(interaction) => interaction,
        None => {
            return Ok(());
        },
    };
    let response = match &interaction.data.custom_id.as_str() {
        &"show_count" => {
            let new_cfg = TodoConfig { show_count: !cfg.show_count };
            update_config(&data.database, new_cfg.clone()).await?;
            let show_count_str = if new_cfg.show_count { "Yes" } else { "No" };
            format!("Updated configuration:\n- Show item count: {}", show_count_str)
        },
        _ => panic!("unexpected interaction custom id"),
    };
    reply_handle.edit(ctx, CreateReply::new().content(response)).await?;
    interaction.create_response(ctx.as_ref(), CreateInteractionResponse::Acknowledge).await?;
    Ok(())
}

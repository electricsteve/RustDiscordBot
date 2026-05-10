use super::constants::COMPONENT_ID;
use crate::core::database::{get_component_config, set_component_config};
use crate::utils::config::component_config;
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CollectComponentInteractions, CreateInteractionResponse};
use std::sync::OnceLock;
use std::time::Duration;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::SurrealValue;
use tokio::sync::RwLock;

component_config!(
    TodoConfig,
    COMPONENT_ID,
    #[surreal(default)]
    pub show_count: bool
);

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

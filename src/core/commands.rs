use super::core_custom_data;
use crate::{Context, Error};

#[poise::command(prefix_command, owners_only, custom_data = "core_custom_data()")]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    let commands = &ctx.framework().options().commands;
    poise::builtins::register_globally(ctx.http(), commands).await?;

    ctx.say("Successfully registered slash commands!").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, owners_only, custom_data = "core_custom_data()")]
pub async fn toggle_component(
    ctx: Context<'_>,
    #[description = "The ID of the component to toggle"] component_id: String,
) -> Result<(), Error> {
    let data = ctx.data();
    if !data.components.iter().any(|c| c.id == component_id) {
        ctx.say(format!("Component with ID `{component_id}` not found!")).await?;
        return Ok(());
    }
    if data.is_component_enabled(&component_id).await? {
        if let Err(error) = data.disable_component(component_id.clone()).await {
            ctx.say(format!("An error occurred while toggling component! Error: {error}")).await?;
            return Ok(());
        }
        ctx.say(format!("Component `{component_id}` disabled!")).await?;
    } else {
        if let Err(error) = data.enable_component(component_id.clone()).await {
            ctx.say(format!("An error occurred while toggling component! Error: {error}")).await?;
            return Ok(());
        }
        ctx.say(format!("Component `{component_id}` enabled!")).await?;
    }
    // ctx.say("Successfully registered slash commands!").await?;
    Ok(())
}

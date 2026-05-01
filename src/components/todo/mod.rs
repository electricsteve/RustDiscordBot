#[allow(dead_code)]
mod config;
mod constants;
mod database;

use crate::component::Component;
use crate::component::InitializerFuture;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;
use std::sync::Arc;

pub fn component() -> Box<Component> {
    Box::new(Component {
        id: constants::COMPONENT_ID.to_string(),
        commands: vec![todo, config::config],
        event_handler: Arc::new(Handler),
        initializer: Some(|data| Box::pin(initializer(data))),
    })
}

fn initializer(data: &mut crate::GlobalData) -> InitializerFuture<'_> {
    Box::pin(async move {
        database::migrate(&data.database).await?;
        Ok(())
    })
}

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, _context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot: _data_about_bot, .. } = event {
            println!("Todo component loaded!");
        }
    }
}

/// Check your own to-do list
#[poise::command(slash_command, prefix_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let data = ctx.data();
    let todo_list = database::get_todo_list(user_id, &data.database).await;
    if todo_list.is_empty() {
        ctx.say("There is currently nothing on your to-do list.").await?;
    } else {
        let formatted_list: String = todo_list.join("\n- ");
        let response = format!("These are the items on your to-do list:\n- {}", formatted_list);
        let reply = crate::utils::messages::silent_mentions(response.as_str());
        ctx.send(reply).await?;
    }
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("list", "add", "remove"),
    subcommand_required
)]
pub async fn todo(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn add(ctx: Context<'_>, content: String) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let data = ctx.data();
    database::add_todo(user_id, content.clone(), &data.database).await;
    ctx.say(format!("Successfully added `{content}` to your to-do list!")).await?;
    Ok(())
}

/// Remove an item from your todo list. BROKEN BECAUSE OF A SURREALDB BUG
#[poise::command(prefix_command, slash_command)]
pub async fn remove(ctx: Context<'_>, index: u32) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let data = ctx.data();
    let result = database::remove_todo(user_id, index, &data.database).await;
    if let Err(e) = result {
        let error_message = match e {
            database::TodoError::EmptyList => "Your to-do list is currently empty.",
            database::TodoError::InvalidIndex => "The provided index is invalid.",
        };
        ctx.say(error_message).await?;
        return Ok(());
    } else {
        let result = result.unwrap();
        // ctx.say(format!("Successfully removed `{result}` from your to-do list!")).await?;
        ctx.say("THIS IS BROKEN! Yes ik nothing actually got removed from your todo, but this is because of a surrealdb bug that is supposed to have a fix merged the day after I ship 😭".to_string()).await?;
    }
    Ok(())
}

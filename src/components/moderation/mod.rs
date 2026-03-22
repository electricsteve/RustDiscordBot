use std::sync::Arc;
use crate::component::Component;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;

pub fn component() -> Box<Component> {
    Box::new(Component {
        id: "moderation".to_string(),
        commands: vec![ping],
        event_handler: Arc::new(Handler),
        initializer: None,
    })
}

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, _context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot: _data_about_bot, .. } = event {
            println!("Moderation component loaded!");
        }
    }
}

#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
    #[description = "Message"] message: Option<String>,
) -> Result<(), Error> {
    if let Some(msg) = message {
        ctx.say(format!("{} Pong!", msg)).await?;
    } else {
        ctx.say("Pong!").await?;
    }
    Ok(())
}
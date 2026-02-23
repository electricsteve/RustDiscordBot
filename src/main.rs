mod components;
mod component;

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::env;

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    /*
Set a handler to be called on the `ready` event. This is called when a shard is booted, and
a READY payload is sent by Discord. This payload contains data like the current user's guild
Ids, current user data, private channels, and more.

In this case, just print what the current user's username is.
*/
    async fn ready(&self, _: serenity::Context, ready: serenity::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct Data {} // User data, which is stored and accessible in all command invocations

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// #[poise::command(slash_command, prefix_command)]
// async fn ping(
//     ctx: Context<'_>,
//     #[description = "Message"] message: Option<String>,
// ) -> Result<(), Error> {
//     if let Some(msg) = message {
//         ctx.say(format!("{} Pong!", msg)).await?;
//     } else {
//         ctx.say("Pong!").await?;
//     }
//     Ok(())
// }

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands globally with discord
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // Return Data struct
                Ok(Data {})
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        serenity::Client::builder(&token, intents).framework(framework).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
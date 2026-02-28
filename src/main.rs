mod components;
mod component;

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::env;
use serenity::all::Event;

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

struct RawHandler;

#[serenity::async_trait]
impl serenity::RawEventHandler for RawHandler {
    async fn raw_event(&self, _ctx: serenity::all::Context, _ev: Event) {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let components = components::get_components();
    let commands = components.iter().flat_map(|component| component.commands.iter()).map(|cmd| cmd()).collect::<Vec<_>>();
    let event_handlers = components.into_iter().map(|component| component.event_handler).collect::<Vec<_>>();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
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
    let mut client_builder = serenity::Client::builder(&token, intents).framework(framework).event_handler(Handler).raw_event_handler(RawHandler);
    // for event_handler in event_handlers {
    //     client_builder = client_builder.event_handler_arc(event_handler)
    // }
    let mut client =
        client_builder.await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
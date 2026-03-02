mod components;
mod component;

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use serenity::all::FullEvent;
use std::sync::Arc;

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot , .. } = event {
            println!("{} is connected!", data_about_bot.user.name);
        }
        let data: Arc<Data> = context.data();
        for component in &data.components {
            component.event_handler.dispatch(context, event).await;
        }
    }
}

struct Data {
    // TODO: component management
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/6
    // Turn individual components on and off at runtime.
    components: Vec<component::Component>,
    // TODO: database
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/5
    // A database components and use to store data.
    // Also use the database for storing component management data.
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, owners_only)]
async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    let commands = &ctx.framework().options().commands;
    poise::builtins::register_globally(ctx.http(), commands).await?;

    ctx.say("Successfully registered slash commands!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // TODO: remove dotenv dependency
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/4
    dotenv::dotenv().ok();
    // Login with a bot token from the environment
    let token = serenity::Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let components = components::get_components();
    let mut commands = components.iter().flat_map(|component| component.commands.iter()).map(|cmd| cmd()).collect::<Vec<_>>();

    commands.push(register_commands());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .build();

    let data = Data {
        components
    };
    let client_builder = serenity::Client::builder(token, intents).framework(Box::new(framework)).event_handler(Arc::new(Handler)).data(Arc::new(data));
    let mut client =
        client_builder.await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
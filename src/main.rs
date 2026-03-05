mod components;
mod component;

use poise::{BoxFuture, Command, PrefixFrameworkOptions, serenity_prelude::{self as serenity}};
use serenity::all::FullEvent;
use std::sync::{Arc, Mutex};

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot , .. } = event {
            println!("{} is connected!", data_about_bot.user.name);
        }
        let data: Arc<Data> = context.data();
        for component in &data.components {
            if !data.enabled_components.lock().unwrap().contains(&component.id) {
                continue;
            }
            component.event_handler.dispatch(context, event).await;
        }
    }
}

struct Data {
    // TODO: component management
    // Turn individual components on and off at runtime.
    components: Vec<component::Component>,
    enabled_components: Mutex<Vec<String>>,
    // TODO: database
    // A database components and use to store data.
    // Also use the database for storing component management data.
}

struct CommandData {
    component_id: String,
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

#[poise::command(prefix_command, slash_command, owners_only, custom_data = "CommandData { component_id: \"core\".to_string() }")]
async fn toggle_component(ctx: Context<'_>, #[description = "The ID of the component to toggle"] component_id: String) -> Result<(), Error> {
    let data = ctx.data();
    if !data.components.iter().any(|c| c.id == component_id) {
        ctx.say(format!("Component with ID `{component_id}` not found!")).await?;
        return Ok(());
    }
    if data.enabled_components.lock().unwrap().contains(&component_id) {
        data.enabled_components.lock().unwrap().retain(|id| id != &component_id);
        ctx.say(format!("Component `{component_id}` disabled!")).await?;
    } else {
        data.enabled_components.lock().unwrap().push(component_id.clone());
        ctx.say(format!("Component `{component_id}` enabled!")).await?;
    }
    // ctx.say("Successfully registered slash commands!").await?;
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
    let mut commands : Vec<Command<crate::Data, crate::Error>> = Vec::new();

    for component in &components {
        for command_fn in &component.commands {
            let mut command = command_fn();
            command.custom_data = Box::new(CommandData {
                component_id: component.id.clone(),
            });
            commands.push(command);
        }
    }

    commands.push(register_commands());
    commands.push(toggle_component());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            command_check: Some(command_check),
            ..Default::default()
        })
        .build();

    let data = Data {
        enabled_components: Mutex::new(components.iter().map(|c| c.id.clone()).collect()),
        components,
    };
    data.enabled_components.lock().unwrap().push("core".to_string());
    let client_builder = serenity::Client::builder(token, intents).framework(Box::new(framework)).event_handler(Arc::new(Handler)).data(Arc::new(data));
    let mut client =
        client_builder.await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

fn command_check(ctx: poise::Context<'_, Data, Error>) -> BoxFuture<'_, Result<bool, Error>> {
    Box::pin(async move {
        let component_id = match &ctx.command().custom_data.downcast_ref::<CommandData>() {
            Some(command_data) => &command_data.component_id,
            None => {
                // TODO: add tracing
                // tracing::warn!("Command custom data is not of type CommandData");
                ctx.say("An error occured while checking command component!").await?;
                return Ok(true);
            }
        };
        let data = ctx.data();
        if !data.enabled_components.lock().unwrap().contains(component_id) {
            ctx.say("This component is not enabled!").await?;
            Ok(false)
        } else {
            Ok(true)
        }
    })
}
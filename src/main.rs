mod components;
mod component;
mod core_component;

use crate::component::Component;
use poise::{serenity_prelude::self as serenity, Command, PrefixFrameworkOptions};
use serenity::all::FullEvent;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;

// TODO: global config through env and/or config file
// Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/8
// Also relates to #4

#[tokio::main]
async fn main() {
    // Get environment
    // TODO: remove dotenv dependency
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/4
    dotenv::dotenv().ok();
    let ( token, database_path ) = get_environment();
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Setup components & commands
    let components = components::get_components();
    let mut commands : Vec<Command<GlobalData, Error>> = Vec::new();
    core_component::custom_data(&components, &mut commands);
    commands.append(&mut core_component::commands());

    // Setup framework
    let framework = get_framework(commands);
    let db = get_database(database_path).await;
    db.use_ns("rust_discord_bot").use_db("main").await.expect("Failed to select database namespace");
    let mut data = get_data(db, components);

    // Run component initializers. Collect first to avoid borrowing `data` both immutably and mutably.
    let initializers: Vec<(String, crate::component::Initializer)> = data
        .components
        .iter()
        .filter_map(|component| component.initializer.map(|initializer| (component.id.clone(), initializer)))
        .collect();

    for (component_id, initializer) in initializers {
        if let Err(why) = initializer(&mut data).await {
            println!("Error initializing component {}: {why:?}", component_id);
        }
    }

    // Build client
    let client_builder = serenity::Client::builder(token, intents).framework(Box::new(framework)).event_handler(Arc::new(MainEventHandler)).data(Arc::new(data));
    let mut client =
        client_builder.await.expect("Error creating client");

    // Start bot
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

struct MainEventHandler;

#[serenity::async_trait]
impl serenity::EventHandler for MainEventHandler {
    async fn dispatch(&self, context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot , .. } = event {
            println!("{} is connected!", data_about_bot.user.name);
        }
        let data: Arc<GlobalData> = context.data();
        for component in &data.components {
            if !data.enabled_components.lock().unwrap().contains(&component.id) && component.id != "core" {
                continue;
            }
            component.event_handler.dispatch(context, event).await;
        }
    }
}

struct GlobalData {
    // TODO: component management
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/6
    // Turn individual components on and off at runtime.
    components: Vec<Component>,
    enabled_components: Mutex<Vec<String>>,
    todo_map: Mutex<HashMap<serenity::UserId, Vec<String>>>,
    #[allow(dead_code)]
    database: Surreal<Db>,
    // TODO: database
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/5
    // A database components and use to store data.
    // Also use the database for storing component management data.
}


type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, GlobalData, Error>;

fn get_environment() -> (serenity::Token, String) {
    let token = serenity::Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment");
    let database_path = env::var("DATABASE_PATH").unwrap_or("database".to_string());
    (token, database_path)
}

fn get_data(db: Surreal<Db>, components: Vec<Component>) -> GlobalData {
    GlobalData {
        enabled_components: Mutex::new(components.iter().map(|c| c.id.clone()).collect()),
        components,
        todo_map: Mutex::new(HashMap::new()),
        database: db,
    }
}

fn get_framework(commands: Vec<Command<GlobalData, Error>>) -> poise::Framework<GlobalData, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            command_check: Some(core_component::command_check),
            ..Default::default()
        })
        .build()
}


async fn get_database(path: String) -> Surreal<Db> {
    Surreal::new::<SurrealKv>(path).await.expect("Failed to initialize database")
}
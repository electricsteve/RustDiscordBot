mod component;
mod components;
mod core;
pub mod init;
pub mod types;
pub mod utils;

use crate::component::Component;
use poise::{Command, PrefixFrameworkOptions, serenity_prelude as serenity};
use std::env;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, SurrealKv};

pub use types::{Context, Error, GlobalData};

// TODO: global config through env and/or config file
// Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/8
// Also relates to #4

#[tokio::main]
async fn main() {
    let (token, database_path) = get_environment();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Setup components & commands
    let components = components::get_components();
    let mut commands: Vec<Command<GlobalData, Error>> = Vec::new();
    init::get_commands(&components, &mut commands);
    commands.append(&mut core::commands());

    // Setup framework
    let framework = get_framework(commands);
    let db = get_database(database_path).await;
    db.use_ns("rust_discord_bot")
        .use_db("main")
        .await
        .expect("Failed to select database namespace");
    let mut data = get_data(db, components);

    // Run component initialisers. Collect first to avoid borrowing `data` both immutably and mutably.
    let initializers: Vec<(String, component::Initializer)> = data.get_initializers();
    let core_initializers: Vec<component::Initializer> = vec![core::database::migrate];

    for initializer in core_initializers {
        if let Err(why) = initializer(&mut data).await {
            println!("Error initializing core component: {why:?}");
            println!("The bot will now exit");
            return;
        }
    }
    for (component_id, initializer) in initializers {
        if let Err(why) = initializer(&mut data).await {
            println!("Error initializing component {}: {why:?}", component_id);
        }
    }

    // Build client
    let client_builder = serenity::Client::builder(token, intents)
        .framework(Box::new(framework))
        .event_handler(Arc::new(core::events::MainEventHandler))
        .data(Arc::new(data));
    let mut client = client_builder.await.expect("Error creating client");

    // Start bot
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

fn get_environment() -> (serenity::Token, String) {
    let token =
        serenity::Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment");
    let database_path = env::var("DATABASE_PATH").unwrap_or("database".to_string());
    (token, database_path)
}

fn get_data(db: Surreal<Db>, components: Vec<Component>) -> GlobalData {
    GlobalData { components, database: db }
}

fn get_framework(commands: Vec<Command<GlobalData, Error>>) -> poise::Framework<GlobalData, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            command_check: Some(core::command_check),
            ..Default::default()
        })
        .build()
}

async fn get_database(path: String) -> Surreal<Db> {
    Surreal::new::<SurrealKv>(path).await.expect("Failed to initialize database")
}

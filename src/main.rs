mod component;
mod components;
mod core;
pub mod environment;
pub mod init;
pub mod types;
pub mod utils;

use poise::{serenity_prelude as serenity, Command, PrefixFrameworkOptions};
use std::sync::Arc;
use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;

use crate::environment::Environment;
pub use types::{Context, Error, ErrorType, GlobalData};

#[tokio::main]
async fn main() {
    let mut env = Environment::default();
    env.load_env();
    let token = env.token.clone().expect("Failed to get discord token");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Setup components & commands
    let components = components::get_components();
    let mut commands: Vec<Command<GlobalData, Error>> = Vec::new();
    init::get_commands(&components, &mut commands);
    commands.append(&mut core::commands());

    // Setup framework
    let framework = get_framework(commands, &env);
    let db = Surreal::new::<SurrealKv>(env.database_path.clone())
        .await
        .expect("Failed to initialize database");
    db.use_ns(env.database_namespace.clone())
        .use_db(env.database_database.clone())
        .await
        .expect("Failed to select database namespace & database");
    let mut data = GlobalData { components, database: db };

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

fn get_framework(
    commands: Vec<Command<GlobalData, Error>>,
    env: &Environment,
) -> poise::Framework<GlobalData, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(env.prefix.clone().into()),
                ..Default::default()
            },
            command_check: Some(core::command_check),
            ..Default::default()
        })
        .build()
}

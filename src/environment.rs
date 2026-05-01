use poise::serenity_prelude::Token;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Environment {
    // TODO: environment variables macro
    // Issue URL: https://github.com/electricsteve/Rustey/issues/17
    // Specify fields here, with environment variable names, and let the macro handle them to use in load_env/load_file
    /// PLEASE don't use the discord token unless absolutely necessary. If you know how I can lock it down to only main.rs please tell me.
    pub(crate) token: Option<Token>,
    pub database_path: PathBuf,
    pub database_namespace: String,
    pub database_database: String,
    pub prefix: String,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            token: None,
            database_namespace: "rust_discord_bot".to_string(),
            database_database: "main".to_string(),
            database_path: PathBuf::from("database"),
            prefix: "!".to_string(),
        }
    }
}

impl Environment {
    pub fn load_env(&mut self) {
        // Custom logic
        let token = Token::from_env("DISCORD_TOKEN").ok();
        let database_path = env::var("DB_PATH").ok().map(PathBuf::from);
        if token.is_some() {
            self.token = token;
        }
        if let Some(path) = database_path {
            self.database_path = path;
        }
        // Generic types
        if let Ok(database_namespace) = env::var("DB_NAMESPACE") {
            self.database_namespace = database_namespace;
        }
        if let Ok(database_database) = env::var("DB_DATABASE") {
            self.database_database = database_database;
        }
        if let Ok(prefix) = env::var("PREFIX") {
            self.prefix = prefix;
        }
    }

    // TODO: load config from file
    // Issue URL: https://github.com/electricsteve/Rustey/issues/16
    // Add another method to load from a file
}

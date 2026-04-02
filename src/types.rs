use crate::component;
use crate::component::Component;
use std::fmt;
use std::fmt::Formatter;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;

pub struct GlobalData {
    // TODO: component management
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/6
    // Turn individual components on and off at runtime.
    pub components: Vec<Component>,
    #[allow(dead_code)]
    pub database: Surreal<Db>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, GlobalData, Error>;

impl GlobalData {
    pub fn get_initializers(&self) -> Vec<(String, component::Initializer)> {
        self.components
            .iter()
            .filter_map(|component| {
                component.initializer.map(|initializer| (component.id.clone(), initializer))
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum ErrorType {
    IllegalArgument(String),
    NotFound(String),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::IllegalArgument(msg) => write!(f, "Illegal argument: {msg}"),
            ErrorType::NotFound(msg) => write!(f, "Not found: {msg}"),
        }
    }
}

impl std::error::Error for ErrorType {}

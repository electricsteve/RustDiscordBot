use crate::component::Component;
use std::sync::Mutex;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use crate::component;

pub struct GlobalData {
    // TODO: component management
    // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/6
    // Turn individual components on and off at runtime.
    pub components: Vec<Component>,
    pub enabled_components: Mutex<Vec<String>>,
    #[allow(dead_code)]
    pub database: Surreal<Db>,
}


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, GlobalData, Error>;

impl GlobalData {
    pub fn get_initializers(&self) -> Vec<(String, component::Initializer)> {
        self
            .components
            .iter()
            .filter_map(|component| component.initializer.map(|initializer| (component.id.clone(), initializer)))
            .collect()
    }
}
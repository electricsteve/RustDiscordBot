use poise::{BoxFuture, Command};
use crate::{Context, GlobalData, Error};
use crate::component::Component;

pub fn commands() -> Vec<Command<GlobalData, Error>> {
    vec![register_commands(), toggle_component()]
}

const CORE_COMPONENT_ID: &str = "core";

struct CommandData {
    component_id: String,
}

fn core_custom_data() -> CommandData {
    CommandData {
        component_id: CORE_COMPONENT_ID.to_string()
    }
}

#[poise::command(prefix_command, owners_only, custom_data = "core_custom_data()")]
async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    let commands = &ctx.framework().options().commands;
    poise::builtins::register_globally(ctx.http(), commands).await?;

    ctx.say("Successfully registered slash commands!").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, owners_only, custom_data = "core_custom_data()")]
async fn toggle_component(ctx: Context<'_>, #[description = "The ID of the component to toggle"] component_id: String) -> Result<(), Error> {
    let data = ctx.data();
    if !data.components.iter().any(|c| c.id == component_id) {
        ctx.say(format!("Component with ID `{component_id}` not found!")).await?;
        return Ok(());
    }
    if data.is_component_enabled(&component_id) {
        if let Err(error) = data.disable_component(component_id.clone()) {
            ctx.say(format!("An error occurred while toggling component! Error: {error}")).await?;
            return Ok(());
        }
        ctx.say(format!("Component `{component_id}` disabled!")).await?;
    } else {
        if let Err(error) = data.enable_component(component_id.clone()) {
            ctx.say(format!("An error occurred while toggling component! Error: {error}")).await?;
            return Ok(());
        }
        ctx.say(format!("Component `{component_id}` enabled!")).await?;
    }
    // ctx.say("Successfully registered slash commands!").await?;
    Ok(())
}

impl GlobalData {
    fn enable_component(&self, component: String) -> Result<(), String> {
        if !Self::component_is_allowed(&component) {
            return Err("This component cannot be enabled or disabled!".to_string());
        }
        self.enabled_components.lock().unwrap().push(component.clone());
        Ok(())
    }
    fn disable_component(&self, component: String) -> Result<(), String> {
        if !Self::component_is_allowed(&component) {
            return Err("This component cannot be enabled or disabled!".to_string());
        }
        self.enabled_components.lock().unwrap().retain(|c| c != &component);
        Ok(())
    }
    fn is_component_enabled(&self, component: &String) -> bool {
        if !Self::component_is_allowed(component) {
            return true;
        }
        self.enabled_components.lock().unwrap().contains(component)
    }
    fn component_is_allowed(component: &String) -> bool {
        component != CORE_COMPONENT_ID
    }
}

pub fn command_check(ctx: poise::Context<'_, GlobalData, Error>) -> BoxFuture<'_, Result<bool, Error>> {
    Box::pin(async move {
        let component_id = match &ctx.command().custom_data.downcast_ref::<CommandData>() {
            Some(command_data) => &command_data.component_id,
            None => {
                // TODO: add tracing
                // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/7
                // tracing::warn!("Command custom data is not of type CommandData");
                ctx.say("An error occurred while checking command component!").await?;
                return Ok(true);
            }
        };
        if component_id == CORE_COMPONENT_ID {
            return Ok(true);
        }
        let data = ctx.data();
        if !data.enabled_components.lock().unwrap().contains(component_id) {
            ctx.say("This component is not enabled!").await?;
            Ok(false)
        } else {
            Ok(true)
        }
    })
}

fn add_custom_data(command: &mut Command<GlobalData, Error>, component: &Component) {
    command.custom_data = Box::new(CommandData {
        component_id: component.id.clone(),
    });
    if !command.subcommands.is_empty() {
        for subcommand in &mut command.subcommands {
            // Pray this goes well and doesn't ever cause an infinite recursion
            add_custom_data(subcommand, component);
        }
    }
}


pub fn custom_data(components: &Vec<Component>, commands: &mut Vec<Command<GlobalData, Error>>) {
    for component in components {
        for command_fn in &component.commands {
            let mut command: Command<GlobalData, Error> = command_fn();
            add_custom_data(&mut command, component);
            commands.push(command);
        }
    }
}
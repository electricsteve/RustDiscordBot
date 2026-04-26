pub mod commands;
pub mod database;
pub mod events;

use crate::core::database::{ComponentData, Enabled};
use crate::types::ErrorType::{IllegalArgument, NotFound};
use crate::{Error, GlobalData};
use poise::{BoxFuture, Command};

pub fn commands() -> Vec<Command<GlobalData, Error>> {
    vec![commands::register_commands(), commands::toggle_component()]
}

const CORE_COMPONENT_ID: &str = "core";

pub struct CommandData {
    pub component_id: String,
}

/// CommandData added to core commands
fn core_custom_data() -> CommandData {
    CommandData { component_id: CORE_COMPONENT_ID.to_string() }
}

// Add some handy methods to GlobalData
impl GlobalData {
    /// Enable specific component.
    /// DO NOT CALL THIS UNLESS YOU'RE IN CORE COMPONENT
    async fn enable_component(&self, component_id: String) -> Result<(), Error> {
        if !Self::component_is_allowed(&component_id) {
            return Err(
                IllegalArgument(format!("Component {} is not allowed.", component_id)).into()
            );
        }
        let _: Option<ComponentData> = self
            .database
            .update(ComponentData::id_from_component_string(&component_id))
            .merge(Enabled { enabled: true })
            .await?;
        Ok(())
    }
    /// Disable specific component.
    /// DO NOT CALL THIS UNLESS YOU'RE IN CORE COMPONENT
    async fn disable_component(&self, component_id: String) -> Result<(), Error> {
        if !Self::component_is_allowed(&component_id) {
            return Err(
                IllegalArgument(format!("Component {} is not allowed.", component_id)).into()
            );
        }
        let _: Option<ComponentData> = self
            .database
            .update(ComponentData::id_from_component_string(&component_id))
            .merge(Enabled { enabled: false })
            .await?;
        Ok(())
    }
    /// Check if specific component is enabled
    async fn is_component_enabled(&self, component_id: &String) -> Result<bool, Error> {
        if !Self::component_is_allowed(component_id) {
            return Ok(true);
        }
        // TODO: optimize component enabled check
        // Add caching to this so we don't query the database on EVERY COMMAND TRIGGER
        let component: Option<ComponentData> =
            self.database.select(ComponentData::id_from_component_string(component_id)).await?;
        let enabled = match component {
            Some(component) => component.enabled,
            None => {
                return Err(NotFound(format!(
                    "Component {component_id} not found in the database."
                ))
                .into());
            },
        };
        Ok(enabled)
    }
    /// Check if component is allowed to be enabled/disabled
    fn component_is_allowed(component: &String) -> bool {
        component != CORE_COMPONENT_ID
    }
}

/// Check if a command is allowed to run
pub fn command_check(
    ctx: poise::Context<'_, GlobalData, Error>,
) -> BoxFuture<'_, Result<bool, Error>> {
    Box::pin(async move {
        let component_id = match &ctx.command().custom_data.downcast_ref::<CommandData>() {
            Some(command_data) => &command_data.component_id,
            None => {
                // TODO: add tracing
                // Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/7
                // Also add some nice logging for when components load, etc.
                // tracing::warn!("Command custom data is not of type CommandData");
                ctx.say("An error occurred while checking command component!").await?;
                return Ok(true); // Currently runs if it can't get the component id, this may change
            },
        };
        if component_id == CORE_COMPONENT_ID {
            return Ok(true); // Always allow core component
        }
        let data = ctx.data();
        if !data.is_component_enabled(component_id).await? {
            ctx.say("This component is not enabled!").await?;
            Ok(false)
        } else {
            Ok(true)
        }
    })
}

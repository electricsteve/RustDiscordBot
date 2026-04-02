use crate::component::Component;
use crate::{Error, GlobalData};
use poise::Command;

fn add_custom_data(command: &mut Command<GlobalData, Error>, component: &Component) {
    command.custom_data = Box::new(crate::core::CommandData { component_id: component.id.clone() });
    if !command.subcommands.is_empty() {
        for subcommand in &mut command.subcommands {
            // Pray this goes well and doesn't ever cause an infinite recursion
            add_custom_data(subcommand, component);
        }
    }
}

/// Get commands from components with custom data attached
///
/// # Arguments
///
/// * `components`: The components to take the commands from
/// * `commands`: The mutable vec to output the commands to
///
pub fn get_commands(components: &Vec<Component>, commands: &mut Vec<Command<GlobalData, Error>>) {
    for component in components {
        for command_fn in &component.commands {
            let mut command: Command<GlobalData, Error> = command_fn();
            add_custom_data(&mut command, component);
            commands.push(command);
        }
    }
}

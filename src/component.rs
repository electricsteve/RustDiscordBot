use poise::Command;
use serenity::all::EventHandler;

// TODO: implement standardized permission checks
// Implement permission checks for commands so modules don't have to check them manually.
pub struct Component {
    /// List of commands to register with the bot.
    /// Some checks should be manually done, such as permission checks,
    /// but it's not needed to check if the module is active.
    commands: Vec<Command<crate::Data, crate::Error>>,
    /// An event handler struct.
    event_handler: dyn EventHandler + 'static,
}
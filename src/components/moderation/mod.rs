use crate::component::Component;
use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;
use std::sync::Arc;

pub fn component() -> Box<Component> {
    Box::new(Component {
        id: "moderation".to_string(),
        commands: vec![],
        event_handler: Arc::new(Handler),
        initializer: None,
    })
}

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, _context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot: _data_about_bot, .. } = event {
            println!("Moderation component loaded!");
        }
    }
}

// TODO(moderation): basic commands
// Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/10
//  ## Description
//
//  Add some basic commands to the moderation module.
//
//  ## Requirements
//
//  Ban and mute must have time options.
//  User datils must have:
//  - Basic info
//   - Display name
//   - Username
//   - Discord join date
//   - Server join date
//   - About me
//  - Server info
//   - Roles
//  - Connections (w/ links)
//
//  ## Checklist
//
//  - [ ] User details
//  - [ ] Banning
//  - [ ] Muting
//  - [ ] Kicking
//  - [ ] Role management

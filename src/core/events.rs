use crate::GlobalData;
use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;
use std::sync::Arc;

/// The main event handler that calls all component event handlers
pub struct MainEventHandler;

#[serenity::async_trait]
impl serenity::EventHandler for MainEventHandler {
    async fn dispatch(&self, context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot, .. } = event {
            println!("{} is connected!", data_about_bot.user.name);
        }
        let data: Arc<GlobalData> = context.data();
        for component in &data.components {
            match data.is_component_enabled(&component.id).await {
                Ok(enabled) => {
                    if !enabled {
                        continue;
                    }
                },
                Err(e) => {
                    println!("Error checking if component {} is enabled: {e:?}", component.id);
                    continue;
                },
            }
            component.event_handler.dispatch(context, event).await;
        }
    }
}

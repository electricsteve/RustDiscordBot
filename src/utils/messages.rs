use poise::CreateReply;
use poise::serenity_prelude::builder::CreateAllowedMentions as Am;

pub fn silent_mentions(content: &'_ str) -> CreateReply<'_> {
    CreateReply::default().content(content).allowed_mentions(Am::new())
}

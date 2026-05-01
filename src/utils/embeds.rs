use crate::Context;
use crate::core::get_context_component;
use crate::utils::capitalize;
use bytes::Bytes;
use poise::serenity_prelude::{
    CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
};

/// Adds the author and footer for an embed to the bot defaults.
///
/// ALSO INCLUDE THE BOT ICON ATTACHMENT, OTHERWISE IT WILL NOT WORK. See [`get_bot_icon_attachment`].
pub fn embed_add_details<'a>(ctx: Context<'_>, embed: CreateEmbed<'a>) -> CreateEmbed<'a> {
    let name = crate::utils::bot_info::get_name();
    let version = crate::utils::bot_info::get_version();
    let text = name + " " + &version;
    let component_id = capitalize(&get_context_component(&ctx).unwrap_or_else(|| "Unknown".into()));
    embed
        .footer(CreateEmbedFooter::new(text).icon_url(BOT_ICON_URL))
        .author(CreateEmbedAuthor::new(format!("{component_id} Component")))
}

const BOT_ICON_URL: &str = "attachment://bot-icon.png";

/// Creates a Discord attachment containing the bot icon.
///
/// Meant to be used together with [`embed_add_details`]
pub fn get_bot_icon_attachment() -> CreateAttachment<'static> {
    CreateAttachment::bytes(BOT_ICON, "bot-icon.png") // The filename is important, it needs to match the URL in `BOT_ICON_URL`
}

const BOT_ICON: Bytes = Bytes::from_static(include_bytes!("../../assets/bot.png"));

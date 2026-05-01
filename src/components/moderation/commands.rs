use crate::utils::embeds::{embed_add_details, get_bot_icon_attachment};
use crate::{Context, Error, ErrorType};
use chrono::{Duration, Utc};
use poise::CreateReply;
use poise::serenity_prelude::{CreateEmbed, Member, Timestamp, User};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("user", "timeout", "ban", "kick"),
    subcommand_required
)]
pub async fn moderation(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Get details about a user.
#[poise::command(slash_command, prefix_command)]
async fn user(ctx: Context<'_>, user: User) -> Result<(), Error> {
    // General info
    let display_name = user.display_name();
    let user_name = user.name.as_str();
    let discord_join_date = user.id.created_at().format("%B %e, %Y").to_string();
    // Sadly no way to actually get the bio of a user :(
    // Guild specific info
    let guild = if let Some(guild) = ctx.guild() {
        guild.clone()
    } else {
        // We intentionally fail if no specific guild details found, because this command is only meant to be run in that context.
        return Err(ErrorType::NotFound(
            "No guild found to get guild specific details".to_string(),
        )
        .into());
    };
    let member = guild.member(ctx.as_ref(), user.id).await?;
    let guild_join_date = member.joined_at.map_or(
        // We don't fail here, because this isn't a big enough of a problem
        "Unknown".to_string(),
        |joined_at| joined_at.format("%B %e, %Y").to_string(),
    );
    let mut roles = member.roles(ctx.as_ref()).ok_or_else(|| {
        ErrorType::NotFound("No guild found to get guild specific details".to_string())
    })?;
    roles.sort();
    roles.reverse(); // Sort roles in descending order so higher roles are shown first
    let role_field = if roles.is_empty() {
        "No roles".to_string()
    } else {
        roles.iter().map(|role| format!("- <@&{}>", role.id)).collect::<Vec<String>>().join("\n")
    };

    let embed = CreateEmbed::default()
        .title(format!("{display_name}'s User Details"))
        .description(format!("Details about the Discord user {display_name}."))
        .timestamp(Timestamp::now())
        .thumbnail(member.avatar_url().unwrap_or_else(|| user.avatar_url().unwrap_or_default()))
        .fields(vec![
            ("Display name", display_name, true),
            ("User Name", user_name, true),
            ("Roles", role_field.as_str(), false),
            ("Guild Join Date", &guild_join_date, true),
            ("Discord Join Date", &discord_join_date, true),
        ]);
    let embed = embed_add_details(ctx, embed);
    let attachment = get_bot_icon_attachment();
    let reply = CreateReply::default().embed(embed).attachment(attachment);
    ctx.send(reply).await?;
    Ok(())
}

/// Timeout a user for the specified amount of time.
#[poise::command(slash_command, prefix_command, required_permissions = "MODERATE_MEMBERS")] // MODERATE MEMBERS is "Time out members" in the app, refer to just above here: https://docs.discord.com/developers/topics/permissions#permission-hierarchy
async fn timeout(
    ctx: Context<'_>,
    mut member: Member,
    #[description = "Amount of seconds to time someone out for"]
    #[max = 2419200]
    // 28 days in seconds, which is the maximum timeout duration allowed by Discord
    timeout: u64,
) -> Result<(), Error> {
    let mut date_time = Utc::now();
    date_time += Duration::seconds(timeout as i64);
    let result = member.disable_communication_until(ctx.as_ref(), date_time.into()).await;
    if let Err(error) = result {
        println!("Failed to timeout user: {:?}", error);
        return Err(error.into());
    }
    ctx.say(format!("Successfully timed out {}", member.display_name())).await?;
    Ok(())
}

/// Ban a user.
#[poise::command(slash_command, prefix_command, required_permissions = "BAN_MEMBERS")]
async fn ban(ctx: Context<'_>, member: Member) -> Result<(), Error> {
    let result = member.ban(ctx.as_ref(), 0, None).await;
    if let Err(error) = result {
        println!("Failed to ban user: {:?}", error);
        return Err(error.into());
    }
    ctx.say(format!("Successfully banned {}", member.display_name())).await?;
    Ok(())
}

/// Kick a user
#[poise::command(slash_command, prefix_command, required_permissions = "KICK_MEMBERS")]
async fn kick(ctx: Context<'_>, member: Member) -> Result<(), Error> {
    let result = member.kick(ctx.as_ref(), None).await;
    if let Err(error) = result {
        println!("Failed to kick user: {:?}", error);
        return Err(error.into());
    }
    ctx.say(format!("Successfully kicked {}", member.display_name())).await?;
    Ok(())
}
